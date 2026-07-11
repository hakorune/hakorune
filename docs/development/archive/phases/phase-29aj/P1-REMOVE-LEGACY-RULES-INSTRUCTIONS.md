# Phase 29aj P1: Remove single_planner legacy_rules (Plan extractor SSOT)

Date: 2025-12-29  
Status: Ready for execution  
Scope: plan extractor ownership + single_planner cleanup（仕様不変）  
Goal: plan 層が抽出 SSOT を持ち、single_planner が JoinIR 依存を持たない

## Objective

- single_planner の legacy_rules を撤去し、plan/extractors を直接参照する
- loop_simple_while / if_phi_join を current extractor lane に集約し、他 route family の historical numbered labels 1/3/4/5/8/9 は historical mapping としてだけ残す
- 既定挙動・エラー文字列は不変（抽出実装は移設のみ）

## Implementation Steps

### Step 1: plan/extractors へ移設（SSOT 化）

Files:
- current extractor lane:
  - `src/mir/builder/control_flow/plan/extractors/loop_simple_while.rs`
  - `src/mir/builder/control_flow/plan/extractors/if_phi_join.rs`
- historical extractor token lane:
  - historical numbered labels 4/5/8/9 remain traceability-only in this instruction set

やること:
- JoinIR 側の実装を plan 層へ移動
- `plan/extractors/mod.rs` に module 登録

### Step 2: JoinIR 側は wrapper のみに縮退

Files:
- historical JoinIR wrapper lane（current module surface is `joinir/route_entry`）

やること:
- `pub(crate) use crate::mir::builder::control_flow::plan::extractors::patternX::*;`

### Step 3: single_planner の legacy_rules 撤去

Files:
- `src/mir/builder/control_flow/plan/single_planner/mod.rs`
- `src/mir/builder/control_flow/plan/single_planner/rules.rs`
- `src/mir/builder/control_flow/plan/single_planner/legacy_rules/*`（削除）

やること:
- RuleKind::Simple を `(condition, body)` 署名に変更
- plan/extractors を直接参照
- historical numbered labels 2/6/7 の fallback も plan-side extraction lane へ直結

## Acceptance Criteria

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- single_planner に `legacy_rules` 参照が残っていない

## Commit

- `git add -A && git commit -m "phase29aj(p1): remove legacy_rules via plan extractors"`
