# Phase 25.1k — LoopSSA v2 実装 & Stage‑B SSA 安定化（.hako 本体版）

Status: in progress（.hako 側 LoopSSA v2 本体実装／Rust 側は既存 SSA/PHI を SSOT として維持）

## ゴール

- 25.1j までに固めた LoopSSA/BreakFinderBox/PhiInjectorBox の責務・設計をベースに、  
  `.hako` 側 LoopSSA v2 の **実装本体** に踏み込むフェーズだよ。
- 具体的には:
  - Stage‑B minimal harness（`tools/test_stageb_min.sh`）の Test 2/3 で出ている:
    - Test 2: `.hako` Stage‑B コンパイラ（`compiler_stageb.hako`）が `stageb_min_sample.hako` をコンパイルする  
      **「.hako パーサ＋Stage‑B コンパイラ経路（FuncScanner / LoopSSA / BreakFinderBox / PhiInjectorBox）」** での Rust VM エラー  
      （`BreakFinderBox.find_breaks/2` → `_find_loops/2` の receiver 未定義 `use of undefined value ValueId(..)`）
    - Test 3: 同テストファイルを Rust MIR ビルダーで実行したときの `%0` 由来 SSA エラー（`NYASH_VM_VERIFY_MIR=1` 時）
    を **LoopSSA v2 の改善によって減らす/消す** ことを狙う（Rust 側 LoopForm v2 / Conservative PHI Box は既に緑で SSOT 済み）。
  - 文字列ハードコードベースの `_collect_phi_vars` / synthetic `"r{block}_{var}"` を、  
    Carrier/Pinned ベースの設計に一歩近づける（完全置き換えまでは行かなくても OK）。
  - Rust 側 LoopForm v2 / Conservative PHI Box は SSOT として維持し、.hako 側 LoopSSA v2 は dev トグルで常時検証しつつ徐々に寄せていく。

## 前提（25.1j までで揃っているもの）

- Rust 側:
  - LoopForm v2 + Conservative PHI Box + ControlForm が統合済みで、If/Loop の SSA/PHI は緑。
  - Stage‑B 風ループ（Rust テスト）も LoopForm v2 / Conservative PHI で安定している。
- `.hako` 側:
  - LoopSSA パス:
    - `LoopSSA.stabilize_merges(stage1_json)` が `BreakFinderBox.find_breaks(json, trace_flag)` →  
      `PhiInjectorBox.inject_exit_phis(json, breaks, trace_flag)` の 2 段構成で動作。
    - trace/ENV 解釈は LoopSSA に一元化され、下流には 0/1 の `trace_flag` だけ渡す構造に整理済み。
  - BreakFinderBox:
    - `_find_loops(json_str, trace)` が `"loop_header":` / `"loop_exit":` から loop を検出。
    - `loop_info` に `{"header", "exit", "body", "control"}` を格納し、`control` に ControlFormBox を添付。
  - PhiInjectorBox:
    - 現状は `common_vars = ["i","n","item","j","count","val"]` に対する簡易版 `_collect_phi_vars`。
    - value_id は `"r{block_id}_{var_name}"` 形式の synthetic 値（観測用のダミー）を返している。
  - ドキュメント:
    - 25.1j の README で LoopSSA/BreakFinderBox/PhiInjectorBox の責務境界と Carrier/Pinned/Invariants 概念を明文化済み。

## 方針（25.1k: LoopSSA v2 の「中身」を少し前に進める）

### K‑A: Stage‑B Test2 の ValueId(50) 問題の最小再現と LoopSSA 切り分け

- 目的:
  - `BreakFinderBox._find_loops/2` で発生している `use of undefined value ValueId(50)`（現在は 46→39 と推移中）を、  
    「LoopSSA / BreakFinderBox が生成した JSON / MIR の問題なのか」「それ以前の Stage‑B パイプラインの問題なのか」切り分ける。
- ステップ:
  1. `tools/test_stageb_min.sh` Test2 の Program(JSON v0) 出力を一時ファイルに保存（Stage‑B → Program(JSON v0) 直後）。
  2. その JSON に対して:
     - Rust 側 MirBuilder / LoopForm v2 / Conservative PHI を使って `NYASH_VM_VERIFY_MIR=1` を通し、Rust 側 SSA/PHI の健全性を確認する。
     - `.hako` 側 LoopSSA を単独で呼び出す（`LoopSSA.stabilize_merges(json)`）最小ハーネスを用意し、BreakFinderBox / PhiInjectorBox の前後で JSON を比較。
  3. どの時点で `BreakFinderBox._find_loops/2` の receiver が「定義のない pinned ValueId（例: 39/46/50）」になるかを特定し、  
     LoopSSA v2 の変更前後で挙動が悪化していないかを確認する。

### K‑B: BreakFinderBox の LoopScope 精度の向上（保守的に）

- 目的:
  - `_find_loop_body` の「header_id < id < exit_id」ヒューリスティックが過剰/過少に body を拾っていないかを確認・改善する。
- ステップ:
  - ControlFormBox を活用して LoopScope の妥当性をチェック:
    - header/exit/body に対して Rust 側 LoopForm v2 の期待と比較しやすい形でログを出す（block id 範囲など）。
  - 必要であれば:
    - body 集合を「exit から逆到達できる block」など、より保守的な条件に修正（文字列ベースの範囲内で）。
  - ゴール:
    - LoopSSA が「本来そのループに属さない block」を body に含めないようにする（ValueId(50) のような飛び火を防ぐ）。

### K‑B2: BreakFinderBox._find_loops/2 ループ構造の整理（region box 化の一歩）

- 背景:
  - `BreakFinderBox._find_loops/2` のループ内で `header_pos` / `header_id` / `exit_pos` / `exit_id` を扱う際、
    `header_id == null` や `exit_pos` 異常系で `continue` を多用していたため、
    LoopForm v2 / LoopSSA 側から見ると「ループ本体が細かい early-continue に分割された」形になっていた。
  - Stage‑B Test2 では、この経路で `header_pos` や `i` まわりの ValueId が観測しづらくなり、
    SSA バグ調査の足場としても扱いづらかった。
- 対応:
  - `lang/src/compiler/builder/ssa/exit_phi/break_finder.hako::_find_loops` をリファクタし、
    `next_i` ローカルを導入して「1 イテレーション中のすべての分岐が最後に `i = next_i` に合流する」形に整理した。
    - `header_id == null` / `exit_pos < 0` / 範囲外 / `exit_id == null` の各ケースは、
      `next_i` の更新だけで表現し、`continue` を使わない構造に変更。
    - 正常系（`header_id` / `exit_pos` / `exit_id` が全て有効）の場合のみ、
      `body_blocks` / `ControlFormBox` / `loop_info` の構築と `loops.push(loop_info)` を行い、
      そのうえで `next_i = exit_pos + 12` を設定。
  - これにより:
    - LoopForm v2 から見ると「ループ本体が単一の region（`next_i` による合流）」として観測できるようになり、
      break/continue 経路の SSA 構造が単純化された。
    - ループの意味論自体は従来どおり（ヘッダ/出口検出ロジックや body_blocks の定義は不変）で、
      既存の JSON v0 / LoopSSA の挙動には影響しない。

### K‑C: PhiInjectorBox の v2 への一歩（Carrier/Pinned の入口だけ作る）

- 目的:
  - `_collect_phi_vars` の完全置き換えまでは行かずに、今後の移行先となる v2 API の入口を整える。
- ステップ:
  - `PhiInjectorBox` に新しい内部ヘルパーを追加（例: `_collect_phi_vars_v2(json_str, break_list, loop_info)`）:
    - まだ中身は stub でもよいが、「Carrier/Pinned/Invariants」の 3 区分を引数/戻り値で表現できる形にする。
    - 25.1k では v1 実装（`_collect_phi_vars`）を実際に置き換えず、trace=1 の時だけ v2 の診断ログを出すくらいに留める。
  - `loop_info.get("control")` の ControlFormBox から header/exit/body を読み取り、  
    どの変数が本来の Carrier に相当しそうかをログに出す（まだ PHI には使わない）。

### K‑D: Stage‑B Test3 (%0) の「悪化していない」ことの確認

- 目的:
  - LoopSSA v2 の変更が、Stage‑B Test3 の `%0` SSA 問題を悪化させていないことを確認する。
- ステップ:
  - `NYASH_VM_VERIFY_MIR=1` で Test3 を流したときのエラー位置（関数名/ブロック/命令）を記録。
  - 25.1k の差分適用前後で比較し、LoopSSA 関連の変更による新規エラーが出ていないことを確認。
  - 必要なら、Test3 から LoopSSA を一時オフ（`HAKO_LOOPSSA_EXIT_PHI=0`）にした場合のログも取っておき、  
    「LoopSSA が原因の部分」と「それ以外」を明確に切り分ける。

### K‑E: デバッグ用ハーネス・プリセットの整備

- 目的:
  - LoopSSA/BreakFinderBox/PhiInjectorBox 周辺をデバッグしやすくするための「共通の足場」を用意し、  
    25.1k 以降の作業で毎回同じ ENV/コマンドを手で組み立てなくて済むようにする。
- 実装メモ:
  - `tools/stageb_loopssa_debug.sh`:
    - Stage‑B 最小ハーネス `tools/test_stageb_min.sh` を、LoopSSA v2 デバッグ向けの ENV プリセット
      （`HAKO_LOOPSSA_EXIT_PHI=1`, `HAKO_COMPILER_BUILDER_TRACE=1`, `NYASH_VM_TRACE=1`,
      `NYASH_LOCAL_SSA_TRACE=1`, `NYASH_BUILDER_TRACE_RECV=1` など）付きで実行する小さなラッパ。
  - `lang/src/compiler/tests/loopssa_breakfinder_slot.hako` + `tools/test_loopssa_breakfinder_slot.sh`:
    - Program(JSON v0) を直接文字列として持つ LoopSSA ハーネス（現在は最小緑 JSON、将来は Stage‑B Test2 から抽出した失敗 JSON を貼り付ける「スロット」として運用）。
    - `HAKO_LOOPSSA_EXIT_PHI=1` で LoopSSA v2 / BreakFinderBox / PhiInjectorBox の経路だけを通し、ValueId(..) 問題を Stage‑B 抜きで再現できるようにする。

## このフェーズで「しない」こと

- PhiInjectorBox の `_collect_phi_vars` / `_get_var_value` を **完全刷新すること**:
  - これは 25.1k の次、25.1l 以降の「本格 v2 実装」のタスクとして分ける。
- Rust 側 LoopForm v2 / Conservative PHI の設計を変えること:
  - Rust 側はあくまで SSOT であり、.hako 側はそれに追従する形で徐々に近づける。
