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
- 完了: loop_simple_while / if_phi_join / loop_continue_only / loop_true_early_exit / bool_predicate_scan / accum_const_loop を plan/extractors へ移設し、legacy_rules を削除
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## P2: chosen_rule 撤去 + loop_simple_while planner-first（subset）

- 指示書: `docs/development/current/main/phases/phase-29aj/P2-CHOSEN_RULE-REMOVE-PATTERN1-PLANNER-FIRST-INSTRUCTIONS.md`
- ねらい: outcome の未使用フィールド撤去と loop_simple_while の planner-first 化（仕様不変）
- 完了: chosen_rule を削除し、loop_simple_while facts→planner を single_planner に接続
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## P3: if_phi_join planner-first（subset）

- 指示書: `docs/development/current/main/phases/phase-29aj/P3-PATTERN3-IFPHI-PLANNER-FIRST-INSTRUCTIONS.md`
- ねらい: if_phi_join を Facts→Planner-first に接続し、extractor 依存を削減
- 完了: if_phi_join facts/planner/single_planner を接続
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## P4: loop_continue_only planner-first（subset）

- 指示書: `docs/development/current/main/phases/phase-29aj/P4-PATTERN4-CONTINUE-PLANNER-FIRST-INSTRUCTIONS.md`
- ねらい: loop_continue_only を Facts→Planner-first に接続し、extractor 依存を削減
- 完了: loop_continue_only facts/planner/single_planner を接続
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` / `./tools/smokes/v2/run.sh --profile integration --filter "phase286_pattern4_frag_poc"`

## P5: loop_true_early_exit planner-first（subset）

- 指示書: `docs/development/current/main/phases/phase-29aj/P5-PATTERN5-INFINITE-EARLY-EXIT-PLANNER-FIRST-INSTRUCTIONS.md`
- ねらい: loop_true_early_exit を Facts→Planner-first に接続し、extractor 依存を削減
- 完了: loop_true_early_exit facts/planner/single_planner を接続
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` / `./tools/smokes/v2/run.sh --profile integration --filter "phase143_"`

## P6: JoinIR regression gate SSOT + phase143_* isolation

- 指示書: `docs/development/current/main/phases/phase-29aj/P6-JOINIR-REGRESSION-SSOT-ISOLATE-PHASE143-INSTRUCTIONS.md`
- ねらい: JoinIR 回帰の gate を `phase29ae_regression_pack_vm.sh` に固定し、phase143_* は対象外を明記
- 対象外理由: LoopBuilder 撤去 / plugin disable 固定 / LLVM exe 期待が古い
- 完了: phase143_* を legacy pack に隔離し、SSOT の受け入れは JoinIR pack のみに統一
- 検証: `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` / `./tools/smokes/v2/profiles/integration/joinir/phase143_legacy_pack.sh` (SKIP)

## P7: bool_predicate_scan planner-first（subset）

- 指示書: `docs/development/current/main/phases/phase-29aj/P7-PATTERN8-BOOLPREDICATE-PLANNER-FIRST-INSTRUCTIONS.md`
- ねらい: bool_predicate_scan を Facts→Planner-first に接続し、extractor 依存を削減
- 完了: bool_predicate_scan facts/planner/single_planner を接続
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## P8: accum_const_loop planner-first（subset）

- 指示書: `docs/development/current/main/phases/phase-29aj/P8-PATTERN9-ACCUM-CONST-PLANNER-FIRST-INSTRUCTIONS.md`
- ねらい: accum_const_loop を Facts→Planner-first に接続し、extractor 依存を削減
- 完了: accum_const_loop facts/planner/single_planner を接続
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## P9: phase286_pattern9 legacy isolation (docs/smokes)

- 指示書: `docs/development/current/main/phases/phase-29aj/P9-ISOLATE-PHASE286-PATTERN9-LEGACY-INSTRUCTIONS.md`
- ねらい: JoinIR 回帰の SSOT は phase29ae pack に固定し、phase286_pattern9_* は legacy pack (SKIP) に隔離
- 完了: phase286_pattern9_legacy_pack を追加し、運用ルールを docs に固定
- 検証: `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` / `./tools/smokes/v2/profiles/integration/joinir/phase286_pattern9_legacy_pack.sh` (SKIP)

## P10: single_planner planner-first 形の統一（挙動不変）

- 指示書: `docs/development/current/main/phases/phase-29aj/P10-SINGLE_PLANNER-UNIFY-PLANNER-FIRST-SHAPE-INSTRUCTIONS.md`
- ねらい: 全 loop route を planner-first → extractor フォールバックの共通形に統一
- 完了: RuleKind と single_planner の分岐形を統一し、log_none の意味を SSOT 化
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Verification (SSOT)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
