Status: Active  
Scope: Phase 26‑H.C ― Normalized JoinIR (Pattern1) を MIR に落とす最小ブリッジの設計と比較テスト計画（dev専用）。

# Phase 26‑H.C 指示書 — Normalized→MIR ブリッジ（Pattern1 最小）＋比較テスト

## ゴール
- Normalized JoinIR（NormalizedModule）から MIR への「本物のブリッジ」を、Pattern1 最小ケースだけで実装する。
- Structured→MIR と Structured→Normalized→MIR の結果が一致することをテストで確認する（dev 専用、CLI はまだ触らない）。

## A. Normalized→Structured or Normalized→MIR の経路方針
- Option 1: Normalized→Structured→既存 MIR ブリッジ
  - 長所: 既存の JoinIR→MIR パイプラインを再利用できる。
  - 短所: フェーズが一段増える（Normalized→Structured が joinir→joinir の一歩になる）。
- Option 2: Normalized→MIR を直接作るミニブリッジ（Pattern1 限定） **←今回これを優先**
  - 長所: TailCall/Env をそのまま MIR に落とす感触を掴める。
  - 短所: 一部ロジックが JoinIR→MIR と重複する。
- どちらを選んでも「Pattern1 最小ケース限定」「テスト専用 helper」とする（本番パスへは配線しない）。今回は Option 2 で進め、必要があれば Option 1 のスケルトンも残す。

## B. Normalized→Structured ミニ変換（Option 1 採用時のメモ）
- 追加場所: `src/mir/join_ir/normalized.rs`
- API 例: `pub fn normalize_pattern1_to_structured(norm: &NormalizedModule) -> JoinModule`
- 制約: `norm.phase == JoinIrPhase::Normalized`、関数は 1 つ（loop_step + 仮の k_exit 程度）を想定。
- 変換の要点:
  - EnvLayout の fields から JoinFunction の params を再構成（最小なら1引数でも可）。
  - `JpInst::Let` → `JoinInst::Compute(MirLikeInst::Const/BinOp/UnaryOp/Compare)` に戻す。
  - TailCallFn/TailCallKont/If を Pattern1 が生成していた `JoinInst::Call/Jump/Ret` 相当に戻す。
  - `NormalizedModule.entry` を `JoinModule.entry` に写す。
- `NormalizedModule.structured_backup` は比較用に残すが、ブリッジでは本体から再構成する。

## C. Normalized→MIR ミニブリッジ（今回の主経路: Option 2）
- 追加ファイル案: `src/mir/join_ir/lowering/normalized_pattern1_to_mir.rs`
- API 例:
  ```rust
  pub fn lower_normalized_pattern1_to_mir(norm: &NormalizedModule) -> crate::mir::Module { ... }
  ```
- 最低限やること:
  - EnvLayout の 1 フィールドを MIR のループ変数に対応させる。
  - `JpFunction` 本体を 1 ブロック or 小ブロック列に変換（Let→MIR Assign/Compute、TailCallFn→ループ末尾 Jump 等）。
  - Pattern1 の `loop_min_while` 相当が生成していた MIR と構造的に一致するかをテストで確認。
- ガード: dev/テスト専用 helper から明示的に呼ぶ。ランナー/CLI には配線しない。

## D. 比較用テストの追加
- テストファイル案: `tests/normalized_pattern1_bridge.rs`
- シナリオ:
  1. Structured JoinIR (Pattern1: loop_min_while 相当) を既存 lowerer で生成。
  2. そのコピーを `normalize_pattern1_minimal` に通し NormalizedModule を得る。
  3. 既存経路: Structured → 既存 JoinIR→MIR ブリッジ → 実行 or MIR dump。
  4. 新経路: Structured → Normalized → （C のブリッジ）→ MIR → 実行 or MIR dump。
  5. 比較:
     - 実行結果一致（RC + stdout）。
     - 余裕があれば MIR の基本ブロック数や命令種も比較し、構造乖離がないことを確認。
- テスト名例:
  - `test_normalized_pattern1_minimal_roundtrip`
  - `test_normalized_pattern1_exec_result_matches_structured`

## E. ガードと完了条件
- ガード:
  - 新ブリッジはテスト専用（ランナー/CLI からは呼ばない）。
  - JoinModule.phase が Structured のままでも既存経路は従来どおり動作。
  - Normalized 経路は dev/テスト専用 helper からのみ呼ぶ。
- 完了条件:
  - NormalizedModule から MIR を生成する経路（Option 2）が 1 本通る。
  - Pattern1 最小ケースで Structured→MIR と Structured→Normalized→MIR が同じ結果になるテストが緑。
  - 既存 `cargo test --release` が 0 FAIL のまま。

