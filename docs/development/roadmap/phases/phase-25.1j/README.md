# Phase 25.1j — LoopSSA v2 本体 & Stage‑B harness 強化

Status: planning（.hako 側 LoopSSA v2 本体／Rust SSA/PHI は触らない）

## ゴール

- 25.1i で整えた **LoopSSA + ControlFormBox 観測レイヤー** の上に、.hako 側 LoopSSA v2 の「本体」を載せる準備をする。
  - 特に Stage‑B minimal harness（`tools/test_stageb_min.sh`）Test 2 で出ている  
    `MIR compilation error: Undefined variable: trace` を構造的に解消する。
  - 文字列ハードコードベースの Exit PHI 注入（`_collect_phi_vars` / synthetic `"r{block}_{var}"`）は **即座には捨てず**、
    将来の v2 実装に移行しやすいよう責務を整理する。
- このフェーズでは:
  - LoopSSA / BreakFinderBox / PhiInjectorBox の **trace 変数の扱い**とスコープを整理し、
    「どこで ENV を読むか」「どこまで Box 内のローカルで閉じるか」を決める。
  - Stage‑B Test 2 が Program(JSON v0) 出力まで進み、Rust 側 MirBuilder の SSA/PHI 検証に到達できる状態を作る。
  - Stage‑B Test 3 の `%0`/SSA 問題については「LoopSSA v2 が悪化させていない」ことの確認まで（根治は次フェーズ）。

## 前提（25.1i までで揃っているもの）

- Rust:
  - Loop/If:
    - LoopForm v2 + Conservative PHI Box が ControlForm（`LoopShape`/`IfShape`）経由で統合済み。
    - レガシー `build_loop_legacy` は削除済みで、LoopForm v2 が唯一のループ構築経路。
  - テスト:
    - `mir_loopform_exit_phi` / `mir_stage1_using_resolver_*` / `mir_stageb_loop_break_continue` は緑。
- `.hako`:
  - `lang/src/compiler/builder/ssa/loopssa.hako`:
    - `LoopSSA.stabilize_merges(stage1_json)` が `BreakFinderBox.find_breaks` → `PhiInjectorBox.inject_exit_phis` の簡易パイプライン。
    - `HAKO_LOOPSSA_EXIT_PHI` で Exit PHI 注入の ON/OFF を制御している。
  - `lang/src/compiler/builder/ssa/exit_phi/break_finder.hako`:
    - 文字列ベースで `"loop_header":` / `"loop_exit":` を探す `_find_loops`。
    - `header_id < id < exit_id` の範囲で body block を推定する `_find_loop_body`。
    - break を「`jump` terminator の `target` が `exit_id`」な block として検出。
    - `HAKO_COMPILER_BUILDER_TRACE=1` で `[break-finder] …` と `[loopssa/control] …` ログを出す。
  - `lang/src/shared/mir/control_form_box.hako`:
    - 現行構文で `ControlFormBox` を定義（`kind_name` / `entry` / `exits` + loop/if 用フィールド）。
    - `from_loop(header_id, exit_id, body_blocks)` / `from_if(cond, then, else, merge)` で Loop/If の形を復元。
  - `lang/src/compiler/entry/compiler_stageb.hako`:
    - Stage‑B entry が `CompilerBuilder.apply_all(ast_json)` を通した後の JSON を出力する構成。
    - 25.1i で `using lang.compiler.parser.parser_box as ParserBox` 等の修正により parse error は解消済み。
- Stage‑B minimal harness:
  - `tools/test_stageb_min.sh`:
    - Test1: 直接 VM 実行 → RC=0。
    - Test2: Stage‑B 経由 → LoopSSA/Exit PHI 経路まで進むが最終的に  
      `MIR compilation error: Undefined variable: trace` で停止（LoopSSA v2 経路における trace スコープの問題）。
    - Test3: `NYASH_VM_VERIFY_MIR=1` で `%0` undefined 由来の SSA 問題が残っている（根本は LoopSSA だけとは限らない）。

## 方針（25.1j: LoopSSA v2 本体の入口を整える）

### J‑A: LoopSSA/BREAK/PHI 周辺の trace 変数スコープの整理

- 目的:
  - `.hako` 側 LoopSSA パスの中で `trace` という名前が **常にローカル or Box フィールドとして定義されている** 状態にする。
  - Stage‑B Test2 における `Undefined variable: trace` を根治し、Program(JSON v0) 出力まで進める。
- 方針:
  - 「ENV から読む責務」と「boolean フラグを渡して使う責務」を分離する。
  - 具体的には:
    - `LoopSSA.stabilize_merges` の先頭で `local builder_trace = env.get("HAKO_COMPILER_BUILDER_TRACE")` を読む。
    - `BreakFinderBox.find_breaks` / `PhiInjectorBox.inject_exit_phis` には **数値フラグ `trace_flag`** を第3引数として渡す（0/1）。
    - 各 Box 内では `local trace = trace_flag`（または `me.trace` フィールド）として閉じた名前にする。
  - 既存の `local trace = env.get("HAKO_COMPILER_BUILDER_TRACE")` は LoopSSA に集約し、下位 Box は「引数でもらったフラグだけを見る」構造に変える。

### J‑B: LoopSSA v2 の責務再定義（設計）

- 目的:
  - `LoopSSA.stabilize_merges` が「何を受け取り、どこまで責任を持つか」を Rust LoopForm v2 に揃えた形で **明文化** する。
  - Stage‑B / LoopSSA / BreakFinderBox / PhiInjectorBox の責務分割を Box 単位で固定し、将来の v2 実装時も迷わない足場を作る。
- 方針:
  - LoopSSA パスを 3 層に分解して設計を書く:
    1. **LoopSSA（オーケストレータ箱）**
       - input: Stage‑1 Program(JSON v0)（単一関数 or Program 全体）。
       - output: Loop break に対する exit PHI が挿入された Program(JSON v0)。
       - 責務:
         - dev トレースフラグ（`HAKO_COMPILER_BUILDER_TRACE`）と機能フラグ（`HAKO_LOOPSSA_EXIT_PHI`）を解釈。
         - `BreakFinderBox.find_breaks(stage1_json, trace_flag)` を呼んで Loop + break 情報を収集。
         - `PhiInjectorBox.inject_exit_phis(stage1_json, breaks, trace_flag)` に処理を委譲。
         - 例外時は JSON を変更せず Fail‑Fast する（「部分的に壊れた JSON を返さない」）。
    2. **BreakFinderBox（解析箱 / read‑only）**
       - input: Stage‑1 Program(JSON v0)、trace_flag。
       - output: break 情報の配列（`[{block_id, exit_id, loop_header}, …]`）。
       - 責務:
         - `"loop_header":NNN` / `"loop_exit":MMM` のペアを見つけ、単純な LoopScope を構成する。
         - ControlFormBox に Loop 形（header/exit/body）を写し取る（観測用）。
         - JSON 文字列は一切書き換えない（解析専用）。
    3. **PhiInjectorBox（変換箱 / write‑only）**
       - input: Stage‑1 Program(JSON v0)、break 情報配列、trace_flag。
       - output: Exit PHI 相当の命令を instructions 配列の先頭に挿入した Program(JSON v0)。
       - 責務:
         - break ごとの incoming 値を集約し、`phi_vars = [{name, incoming:[{block,value},…]}, …]` を構成。
         - Exit block の `"instructions":[ … ]` の直後に PHI 相当 JSON をテキスト挿入する。
         - 既存の命令配列順を壊さない（PHI は「先頭に」挿入するのみ）。
  - Carrier / Pinned / Invariant の扱いは **設計としてだけ** 固める:
    - Carrier: ループ内で値が更新され、exit でも必要になる変数（Rust 側 Carrier と対応）。
    - Pinned: pin 付き一時値（`__pin$…`）など、観測目的で特別扱いする変数。
    - Invariant: ループ外で定義され、ループ内で再定義されない値（PHI 不要）。
  - 25.1j では **Exit PHI のアルゴリズム自体はまだ現行の simple 版のまま** としつつ、
    LoopSSA / BreakFinderBox / PhiInjectorBox の責務境界だけを README とソースコードコメントで明確にする。

### J‑C: BreakFinder v2 への足場（ControlFormBox の本格利用準備）

- 目的:
  - 既存の `loop_info = {header, exit, body}` ベースに加えて、ControlFormBox に Loop 形を必ず写す。
- 方針:
  - `BreakFinderBox._find_loops` 内で:
    - `loop_info` 生成に加えて `local cf = new ControlFormBox()` → `cf.from_loop(header_id, exit_id, body_blocks)` を常時呼ぶ。
    - trace ON のとき `[loopssa/control]` ログに加えて ControlFormBox の内容も JSON 風にまとめて出す（header/exit/body_size）。
  - このフェーズでは戻り値の構造はまだ変えず、`find_breaks` は従来どおり breaks 配列を返す。

### J‑D: PhiInjector v2 設計（実装は次フェーズ）

- 目的:
  - Exit PHI 注入を ControlFormBox/LoopScope ベースで書き直すための **設計を先に固める**。
- 方針:
  - 25.1j ではコード本体は変えず、`phi_injector.hako` と README に設計のみ追記:
    - `inject_exit_phis(json_str, breaks, trace_flag)` の将来形（ControlFormBox + break 群 + JSON v0）を定義。
    - Carrier/Pinned/Invariants を Rust LoopForm v2 の概念に揃える。
    - `common_vars = ["i","n","item",…]` のハードコードは「v2 で撤退予定」とコメントで明示。
  - 実アルゴリズムの書き換え（v2 本体）は Phase 25.1k 以降の大きなタスクとして分離する。

## このフェーズで「しない」こと

- `.hako` 側 LoopSSA を Rust LoopForm v2 と完全同型にすること（PHI アルゴリズムの全面移植）は行わない。
- Stage‑B Test3 の `%0`/SSA エラーを「必ずゼロにする」こと:
  - 25.1j では **trace 変数のスコープ修正と LoopSSA v2 の責務整理** に集中する。
  - `%0` の根治は、LoopSSA v2 本体（Carrier/Pinned/Exit PHI）を実装する次フェーズのターゲットとする。
