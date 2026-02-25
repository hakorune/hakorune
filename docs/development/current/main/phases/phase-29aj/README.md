# Phase 29aj: PlannerOutcome observability SSOT

Goal: planner の facts/plan を 1 本の outcome に集約し、観測の SSOT を planner 側に固定する（仕様不変）。

## P0: PlannerOutcome（Facts+Plan）SSOT

- 指示書: `docs/development/current/main/phases/phase-29aj/P0-PLANNER-OUTCOME-SSOT-INSTRUCTIONS.md`
- ねらい: single_planner の観測が planner outcome の facts だけに依存する状態へ統一
- 完了: build_plan_with_facts を追加し、single_planner のタグ出力は outcome.facts 参照に収束
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## P1: Remove single_planner legacy_rules（Plan extractor SSOT）

- 指示書: `docs/development/current/main/phases/phase-29aj/P1-REMOVE-LEGACY-RULES-INSTRUCTIONS.md`
- ねらい: plan 層に extractor を集約し、single_planner の JoinIR 依存を撤去
- 完了: Pattern1/3/4/5/8/9 を plan/extractors へ移設、legacy_rules を削除
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## P2: chosen_rule 撤去 + Pattern1 planner-first（subset）

- 指示書: `docs/development/current/main/phases/phase-29aj/P2-CHOSEN_RULE-REMOVE-PATTERN1-PLANNER-FIRST-INSTRUCTIONS.md`
- ねらい: outcome の未使用フィールド撤去と Pattern1 の planner-first 化（仕様不変）
- 完了: chosen_rule を削除し、Pattern1 facts→planner を single_planner に接続
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## P3: Pattern3 (If-Phi) planner-first（subset）

- 指示書: `docs/development/current/main/phases/phase-29aj/P3-PATTERN3-IFPHI-PLANNER-FIRST-INSTRUCTIONS.md`
- ねらい: Pattern3 を Facts→Planner-first に接続し、extractor 依存を削減
- 完了: Pattern3 facts/planner/single_planner を接続
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## P4: Pattern4 (Continue) planner-first（subset）

- 指示書: `docs/development/current/main/phases/phase-29aj/P4-PATTERN4-CONTINUE-PLANNER-FIRST-INSTRUCTIONS.md`
- ねらい: Pattern4 を Facts→Planner-first に接続し、extractor 依存を削減
- 完了: Pattern4 facts/planner/single_planner を接続
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` / `./tools/smokes/v2/run.sh --profile integration --filter "phase286_pattern4_frag_poc"`

## P5: Pattern5 (Infinite Early Exit) planner-first（subset）

- 指示書: `docs/development/current/main/phases/phase-29aj/P5-PATTERN5-INFINITE-EARLY-EXIT-PLANNER-FIRST-INSTRUCTIONS.md`
- ねらい: Pattern5 を Facts→Planner-first に接続し、extractor 依存を削減
- 完了: Pattern5 facts/planner/single_planner を接続
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` / `./tools/smokes/v2/run.sh --profile integration --filter "phase143_"`

## P6: JoinIR regression gate SSOT + phase143_* isolation

- 指示書: `docs/development/current/main/phases/phase-29aj/P6-JOINIR-REGRESSION-SSOT-ISOLATE-PHASE143-INSTRUCTIONS.md`
- ねらい: JoinIR 回帰の gate を `phase29ae_regression_pack_vm.sh` に固定し、phase143_* は対象外を明記
- 対象外理由: LoopBuilder 撤去 / plugin disable 固定 / LLVM exe 期待が古い
- 完了: phase143_* を legacy pack に隔離し、SSOT の受け入れは JoinIR pack のみに統一
- 検証: `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` / `./tools/smokes/v2/profiles/integration/joinir/phase143_legacy_pack.sh` (SKIP)

## P7: Pattern8 (BoolPredicateScan) planner-first（subset）

- 指示書: `docs/development/current/main/phases/phase-29aj/P7-PATTERN8-BOOLPREDICATE-PLANNER-FIRST-INSTRUCTIONS.md`
- ねらい: Pattern8 を Facts→Planner-first に接続し、extractor 依存を削減
- 完了: Pattern8 facts/planner/single_planner を接続
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## P8: Pattern9 (AccumConstLoop) planner-first（subset）

- 指示書: `docs/development/current/main/phases/phase-29aj/P8-PATTERN9-ACCUM-CONST-PLANNER-FIRST-INSTRUCTIONS.md`
- ねらい: Pattern9 を Facts→Planner-first に接続し、extractor 依存を削減
- 完了: Pattern9 facts/planner/single_planner を接続
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## P9: phase286_pattern9 legacy isolation (docs/smokes)

- 指示書: `docs/development/current/main/phases/phase-29aj/P9-ISOLATE-PHASE286-PATTERN9-LEGACY-INSTRUCTIONS.md`
- ねらい: JoinIR 回帰の SSOT は phase29ae pack に固定し、phase286_pattern9_* は legacy pack (SKIP) に隔離
- 完了: phase286_pattern9_legacy_pack を追加し、運用ルールを docs に固定
- 検証: `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` / `./tools/smokes/v2/profiles/integration/joinir/phase286_pattern9_legacy_pack.sh` (SKIP)

## P10: single_planner planner-first 形の統一（挙動不変）

- 指示書: `docs/development/current/main/phases/phase-29aj/P10-SINGLE_PLANNER-UNIFY-PLANNER-FIRST-SHAPE-INSTRUCTIONS.md`
- ねらい: 全パターンを planner-first → extractor フォールバックの共通形に統一
- 完了: RuleKind と single_planner の分岐形を統一し、log_none の意味を SSOT 化
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Verification (SSOT)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
