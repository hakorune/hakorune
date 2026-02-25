# Phase 25.1i — LoopSSA v2 (.hako) & ControlFormBox 統合（Stage‑B 入口）

Status: in-progress（.hako 側 LoopSSA 設計＋足場実装／Rust挙動は変えない）

## ゴール

- Rust 側で整えた LoopForm v2 / ControlForm（LoopShape / IfShape）を、.hako 側 LoopSSA にも導入し、  
  Stage‑B 最小ハーネス（`tools/test_stageb_min.sh`）で見えている「exit PHI / break 周りの赤ログ」に構造的にアプローチできる足場を作る。
- 25.1i のスコープでは、まず:
  - LoopSSA の責務を ControlFormBox 中心の設計に書き換える（設計＋薄い実装）。
  - 既存の文字列ベース `_find_loops` / `_collect_phi_vars` を温存しつつ、  
    ControlFormBox に loop/if の形を写し取るための API を導入する。
  - Stage‑B Test 2/3 の赤ログ（undefined ValueId / `%0`）は「再現と観測」まで（このフェーズで「必ず全部直す」とはしない）。

## 現状（25.1g/25.1i 途中時点）

- Rust:
  - Loop/If:
    - LoopForm v2 + Conservative PHI Box が ControlForm（LoopShape/IfShape）経由で統合済み。
    - レガシー `build_loop_legacy` / 旧ヘルパーは削除済み（LoopForm v2 が唯一のループ構築経路）。
  - テスト:
    - `mir_loopform_exit_phi` / `mir_stage1_using_resolver_*` / `mir_stageb_loop_break_continue` 緑。
    - `tools/test_stageb_min.sh`:
      - Test 1（直接 VM 実行）: 0 exit。
      - Test 2（Stage‑B 経由）: `BreakFinderBox.find_breaks/1` まわりで undefined ValueId(96)。
      - Test 3（MIR verify）: `%0` 由来の undefined value が残っている。
- .hako:
  - `lang/src/compiler/builder/ssa/loopssa.hako`:
    - `LoopSSA.stabilize_merges(stage1_json)` が BreakFinderBox + PhiInjectorBox を呼び出す簡易パイプライン。
  - `lang/src/compiler/builder/ssa/exit_phi/break_finder.hako`:
    - 文字列ベースで `"loop_header":` / `"loop_exit":` マーカーを探して `_find_loops`。
    - loop body を「header_id < id < exit_id のブロック群」として推定する `_find_loop_body`。
    - break を「jump の target が exit_id の block」として検出。
  - `lang/src/compiler/builder/ssa/exit_phi/phi_injector.hako`:
    - 「共通変数名のリスト（i, n, item, ...）」から `_block_uses_var` で使用有無チェック → `_get_var_value` で synthetic value_id を組み立てて PHI JSON を注入する簡易版。
  - `lang/src/shared/mir/control_form_box.hako`:
    - `static box ControlFormBox`（kind_name + loop_* / if_* + entry/exits）の箱を定義。
    - 25.1i で「未来構文」だった `field: TypeBox` 形式を撤去し、現行構文に合わせたフィールド宣言（`kind_name` など）に修正済み。
    - `from_loop` / `from_if` を追加し、LoopSSA/BreakFinderBox から loop/if 形を写し取るための足場として利用開始。
  - Stage‑B ハーネス:
    - `tools/test_stageb_min.sh` Test 2 の `compiler_stageb.hako` parse error（`Unexpected token COLON`）は ControlFormBox の型注釈撤去と
      `using lang.compiler.parser.parser_box as ParserBox` への修正で解消。
    - 現在は `MIR compilation error: Undefined variable: trace` で停止しており、これは LoopSSA/PhiInjector 側の未実装ロジック由来として
      次フェーズ（25.1j 以降）のターゲットに残している。

## 方針（LoopSSA v2 / ControlFormBox 作戦）

- 25.1i では **一気に全部作り替えない**:
  - 既存の BreakFinderBox / PhiInjectorBox は当面残しつつ、  
    ControlFormBox 経由で loop/if の形を扱うための導線を増やす。
  - まずは「LoopSSA / BreakFinderBox の責務分割」と ControlFormBox の利用ポイントを設計として固定する。
- 段階:
  1. LoopSSA 設計の整理（LoopScope / IfScope / Carrier/Pinned を .hako に翻訳）
  2. ControlFormBox を使った loop/if 形の復元 API を追加
  3. BreakFinderBox / PhiInjectorBox の「入力/出力」を ControlFormBox 前提に徐々に寄せる
  4. Stage‑B Test2/3 で undefined ValueId / `%0` の原因箇所を観測し、次フェーズで本格修正

## タスク粒度（25.1i）

### I‑1: LoopSSA / ControlFormBox の責務整理（設計）

- 目的:
  - LoopSSA フェーズで「何をやるべきか」を、Rust 側 LoopForm v2 / ControlForm の設計に揃えて文章化する。
- ステップ:
  - `LoopSSA.stabilize_merges` のコメントを拡張し、
    - input: Stage‑1 JSON v0（単一関数 or Program）
    - output: exit PHI が入った JSON v0
    - 内部ステップ: BreakFinderBox → LoopForm v2 的な Exit PHI パターンを .hako で再現
    を書く。
  - ControlFormBox に対応する Rust 側 LoopShape/IfShape のフィールドと責務を README で対応づける。

### I‑2: ControlFormBox ユーティリティの追加（Layer 2）

- 目的:
  - `ControlFormBox` を単なるフィールド集合から、実際に JSON v0 から Loop/If 形を復元するための「ミニモデル」に昇格させる。
- ステップ:
  - `lang/src/shared/mir/control_form_box.hako` に、最低限のメソッドを追加:
    - `from_loop(header_id, exit_id, body_blocks)`:
      - header / exit / body の block id 群を受け取り、  
        - `kind_name = "loop"`
        - `entry = header_id` or preheader 相当
        - `loop_header` / `loop_exit` / `loop_body` / `loop_preheader` / `loop_latch` を設定。
      - 初期は「header/exit/body の簡易版」で OK（preheader/latch は後から詰める）。
    - `from_if(cond_block, then_block, else_block, merge_block)`:
      - IfShape に対応する形で `kind_name = "if"` と各フィールドを設定。
  - これらはまだ LoopSSA からは呼ばず、ユニットテスト（小さな JSON 片 or 擬似値）で構築が通ることだけ確認。

### I‑3: BreakFinderBox を ControlFormBox ベースに寄せる準備

- 目的:
  - ループの header/exit/body を ControlFormBox に落とし込む第一歩を作る。
- ステップ:
  - `BreakFinderBox._find_loops` で作っている `loop_info`（header/exit/body）に対して:
    - `ControlFormBox.from_loop(header_id, exit_id, body_blocks)` を呼び出して ControlFormBox を生成。
    - trace=1 のとき `"[loopssa/control] Loop header=..., exit=..., body=[...]"` のようなログを追加。
  - まだ `LoopSSA.stabilize_merges` の戻り値には影響させない（従来どおり breaks → PhiInjectorBox のまま）。

### I‑4: Stage‑B Test2/3 の観測ポイント整備

- 目的:
  - `tools/test_stageb_min.sh` の Test 2/3 実行時に、LoopSSA/ControlFormBox まわりの情報を拾えるようにする。
- ステップ:
  - `HAKO_COMPILER_BUILDER_TRACE=1` 時のログに、LoopSSA と BreakFinderBox/ControlFormBox の状況を追加:
    - 例: `[loopssa] loop count=N`, `[loopssa/control] loop#i header=..., exit=..., body=[...]`
  - `.hako` 側ではまだ Exit PHI を ControlFormBox 経由にはしていないので、  
    「どの loop/exit に対してどんな breaks が検出されているか」を見るところまでで止める。

## このフェーズで「しない」こと

- `.hako` 側 LoopSSA で本格的に Exit PHI を再実装すること:
  - synthetic value_id をやめて Rust 側同等の SSA/PHI を .hako で完全再現するのは、次フェーズの大仕事として分ける。
- Stage‑B 最小ハーネスの赤ログを「必ずゼロにする」こと:
  - 25.1i はあくまで LoopSSA v2 / ControlFormBox を設計・組み込み始めるフェーズ。
  - undefined ValueId / `%0` 問題の根治は 25.1j 以降のターゲットにする。
