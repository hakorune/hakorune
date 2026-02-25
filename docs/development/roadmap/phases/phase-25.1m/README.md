# Phase 25.1m — Static Method / VM Param Semantics Bugfix

Status: completed（静的メソッド / LoopForm v2 continue + PHI の根治完了）

## ゴール

- Rust MIR/VM 層に残っている「静的メソッド呼び出し時の引数ずれ／暗黙レシーバ」問題を解消し、  
  Stage‑B / Stage‑1 / selfhost / Dev トレース（TraceBox 系）がすべて **同じ呼び出し規約**で動くようにする。
- 具体的には:
  - `static box Foo { method bar(x){...} }` に対して
    - 呼び出し: `Foo.bar("HELLO")`
    - VM 内部: `params.len() == 1`、args 1 本 → `bar` の唯一の引数に `"HELLO"` が入る
  - 「暗黙の receiver（仮想 me）」を静的メソッドにだけ特別扱いしない設計に戻す。

## 実際に起きていた症状（2025-11-18 時点）

- 再現（簡易例）:
  ```hako
  static box TraceTest {
    method log(label){
      if label == null { print("label=NULL") }
      else { print("label=\"\" + label") }
    }
  }

  static box Main {
    method main(args){
      TraceTest.log("HELLO")
      return 0
    }
  }
  ```
  - 期待: `label=HELLO`
  - 実際: `label=NULL`
- 原因の一次切り分け:
  - `MirFunction::new` が「名前に '.' を含み、かつ第 1 パラメータ型が Box でない関数」を「静的メソッド with 暗黙 receiver」とみなし、
    - `signature.params.len() = 1`（`label`）でも `total_value_ids = 2` を予約して `params = [%0, %1]` を組み立てている。
  - VM 側の `exec_function_inner` は `args` をそのまま `func.params` に 1:1 でバインドするため:
    - `args = ["HELLO"]`
    - `%0` ← `"HELLO"`（暗黙 receiver）
    - `%1` ← `Void`（足りない分が Void 埋め） → `label` に null が入る。
  - その結果:
    - 静的メソッドに文字列リテラルを直接渡すと label が null 化される。
    - 25.1d/e で扱っていた Stage‑1 UsingResolver 系テスト（`collect_entries/1`）でも、  
      `%0` / `%1` の扱いに由来する SSA 破綻が見えていた（現在は LoopForm v2/Conservative PHI 側で多くを解消済みだが、根底の呼び出し規約はまだ歪なまま）。

## スコープ（25.1m でやったこと）

1. 呼び出し規約の SSOT を決める
   - 原則:
     - **インスタンスメソッド**: `prepare_method_signature` 側で `me` を明示的に第 1 パラメータに含める。
     - **静的メソッド / Global 関数**: `signature.params` は「実引数と 1:1」のみ。暗黙レシーバを追加しない。
   - 影響範囲の調査:
     - `MirFunction::new` の param reservation ロジック（暗黙 receiver 判定と `total_value_ids` 計算）。
     - `emit_unified_call` / `CalleeResolverBox` の Method/Global 判定と receiver 差し込み。
     - VM 側 `exec_function_inner` の args バインド（ここは既に「params と args を 1:1」としているので、なるべく触らない）。

2. 静的メソッドまわりの SSA/テストの洗い出し
   - 代表ケース:
     - `src/tests/mir_stage1_using_resolver_verify.rs` 内の  
       `mir_stage1_using_resolver_full_collect_entries_verifies`（`Stage1UsingResolverFull.collect_entries/1` を静的メソッドとして使うテスト）。
     - Dev 用トレース箱（今回の `StageBTraceBox` / 既存の TraceTest 相当）。
   - 25.1m では:
     - まず `trace_param_bug.hako` 相当のミニテスト（静的メソッド + 1 引数）を Rust 側にユニットテストとして追加し、  
       Bugfix 前後で「label に null が入らない」ことを固定する。
     - 次に `Stage1UsingResolverFull.collect_entries/1` を LoopForm v2 経路込みで通し、  
       `%0` / `%1` の ValueId 割り当てと PHI が健全であることを `MirVerifier` のテストで確認する。

3. 実装方針（高レベル・結果）
   - `MirFunction::new`（静的メソッド / 暗黙レシーバ）:
     - 暗黙 receiver 判定を是正し、**Box 型の第 1 パラメータを持つ関数だけ**を「インスタンスメソッド with receiver」と見なすようにした。
     - 非 Box 型（`String`, `Integer` など）で始まるパラメータ列を持つ関数は、暗黙の receiver なしの静的メソッド / Global 関数として扱い、  
       `signature.params.len()` と予約 ValueId 数が 1:1 になるように整理した。
   - `build_static_main_box`（`Main.main(args)` の扱い）:
     - `src/mir/builder/decls.rs` にて、`Main.main(args)` を静的エントリとして MIR 化する経路を  
       `NYASH_BUILD_STATIC_MAIN_ENTRY=1` 時のみ有効にし、通常の VM 実行では wrapper `main()` を正規エントリとするように変更した。
   - LoopForm v2 / header PHI sealing:
     - `src/mir/phi_core/loopform_builder.rs::LoopFormBuilder::seal_phis` を拡張し、preheader + latch に加えて  
       **`continue_snapshots` からの入力も header PHI に統合**するようにした。
     - これにより、balanced scan など「ループ本体で変数更新 → `continue` → 次イテレーション」のパターンでも、  
       header で参照される変数が preheader / continue / latch すべての predecessor から正しくマージされる。
   - LoopBuilder から LoopForm への continue 橋渡し:
     - `src/mir/loop_builder.rs::build_loop_with_loopform` から `self.continue_snapshots.clone()` を `seal_phis` に渡し、  
       LoopBuilder が集めた continue 時点のスナップショットを LoopForm メタボックス側の PHI 入力に反映するようにした。
   - ControlForm / LoopShape invariants:
     - `src/mir/control_form.rs::LoopShape::debug_validate` に以下の invariant を追加（debug ビルドのみ）:
       - `continue_targets` の各ブロックから `header` へのエッジが存在すること。
       - `break_targets` の各ブロックから `exit` へのエッジが存在すること。
     - これにより、continue / break 経路の CFG 破綻があれば構造レベルで早期に検知できる。

## 非スコープ（25.1m では扱わなかったこと）

- 言語仕様の変更:
  - Hako/Nyash の静的メソッド構文 (`static box` / `method`) 自体は変更しない。
- Stage‑B / Stage‑1 CLI の構造タスク:
  - Stage‑B body 抽出／bundle/using／RegionBox 観測は 25.1c のスコープに残す。
- VM 命令や Box 実装の追加:
  - 25 フェーズのポリシーに従い、新しい命令・Box 機能は追加しない（既存の呼び出し規約を整えるだけ）。

## 関連フェーズとの関係と結果

- Phase 25.1d/e/g/k/l:
  - Rust MIR 側の SSA/PHI（特に LoopForm v2 + Conservative PHI）と Region 観測レイヤは、静的メソッドを含む多くのケースで安定している。
  - 25.1m はその上に残った「呼び出し規約レベルの歪み」を片付けるフェーズ。
- Phase 25.1c:
  - Stage‑B / Stage‑1 CLI 側の構造デバッグ（RegionBox 的な観測、StageBArgs/BodyExtractor/Driver 分解）に専念。
  - StageBTraceBox は既にインスタンス box 化しており、静的メソッドのバグを踏まないようにしてある。
  - 25.1m で静的メソッド呼び出し規約が直れば、将来的に Trace 系 Box を static 化することも再検討できる。

## 受け入れ結果（25.1m 実績）

- 静的メソッド / 暗黙レシーバ:
  - Stage‑B の `StageBTraceBox.log(label)` 呼び出しで、`"StageBArgsBox.resolve_src:enter"` 等のラベルが **null 化されずに正しく渡る** ことを確認。
  - `Main.main(args)` ループ未実行バグは、静的エントリ生成を env フラグ付きに限定し、wrapper `main()` を正規エントリとしたことで解消。
- LoopForm v2 / continue + PHI:
  - 開発用 Hako（`loop_continue_fixed.hako`）で `RC=3` + PHI エラーなしを確認。
  - Stage‑B balanced scan ループに 228 回のイテレーショントレースを付け、`ch == "{"` ブランチ後の `continue` で正常に次イテレーションへ戻ることを Rust VM 実行で確認。
  - 新規ユニットテスト `test_seal_phis_includes_continue_snapshots` と、既存の
    `mir_stageb_loop_break_continue_verifies` / `mir_stage1_using_resolver_full_collect_entries_verifies`
    の両方が緑であることを `cargo test` ベースで確認。

## 25.1m 完了後に残っている課題（次フェーズ向けメモ）

- Stage‑B 本体:
  - `Main.main` 処理内で `String(...) > Integer(13)` のような異種型比較に起因する型エラーが残っている（continue/PHI 修正とは独立）。
  - これは Stage‑B の JSON 生成 / body_src 構造に属する問題のため、25.1m では扱わず、25.1c 続き or 次フェーズで箱単位に切り出して対応する。
  - Stage‑1 / Stage‑B の JSON v0 defs については、25.1m で `src/runner/json_v0_bridge/lowering.rs` を調整し、
    - `box_name != "Main"` の関数定義をインスタンスメソッドとして扱い、
    - Bridge 側で暗黙 receiver `me` を先頭パラメータにバインドすることで、`me._push_module_entry(...)` のような呼び出し時に `me` が未定義にならないようにした。
