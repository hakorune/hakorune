---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Accept the scanner multi-exit loop body through generic_loop_v1 Recipe/CorePlan.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1279-COREPLAN-REAL-SHAPE-FIXTURE-MULTI-EXIT-001.md
  - apps/tests/phase29bq_selfhost_blocker_scanner_multi_exit_min.hako
  - src/mir/builder/control_flow/plan/facts/exit_only_block.rs
  - src/mir/builder/control_flow/plan/parts/dispatch/if_exit_only.rs
  - tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv
---

# COREPLAN-GENERIC-LOOP-MULTI-EXIT-RECIPE-001

## Decision

`generic_loop_v1` now accepts the scanner multi-exit body shape captured in
296x-1279.

The accepted shape is the symmetric single-sided exit form:

```text
if cond {
  exit
} else {
  fallthrough body
}
```

This is represented as `IfContractKind::ExitAllowed` with
`IfMode::ThenOnlyExit`. It is the counterpart of the existing
`ElseOnlyExit` shape.

## Implementation

Added one Recipe/CorePlan vocabulary shape:

```text
IfMode::ThenOnlyExit
```

Connected seams:

```text
facts:
  build_exit_allowed_block recognizes then-exits / else-fallthrough ifs

verify:
  ThenOnlyExit is not a block-exit item

lowering:
  lower_then_only_exit_if lowers then as exit-only and else as exit-allowed
  post-if state comes from the else branch
```

## Evidence

The previous expected-fail fixture is now a normal green gate case:

```text
case_id=selfhost_scanner_multi_exit_min
expected=-4
allowed_rc=0
planner_tag=[joinir/planner_first rule=LoopSimpleWhile] label=LoopSimpleWhile
```

Command:

```bash
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only selfhost_scanner_multi_exit_min
```

Result:

```text
[PASS] phase29bq_fast_gate_cases:selfhost_scanner_multi_exit_min
[PASS] phase29bq_fast_gate_vm: PASS (mode=selfhost_scanner_multi_exit_min)
```

Build check:

```bash
cargo check -q --features vm-reference --bin hakorune
cargo build --release --features vm-reference --bin hakorune
```

Both passed.

## Stop Lines

```text
do not add method-name branches
do not special-case scanner/read_number/json_native names
do not mix continue support into this row
do not widen all generic loops
do not change json_native source to avoid compiler acceptance
```

## Next Task

```text
COREPLAN-CONTINUE-IN-STAGED-LOOP-001
```

That row should capture or select a minimal fixture where `continue` appears in
a staged scanner loop. It must remain separate from this multi-exit acceptance.

## Report

```text
output_contract=coreplan-generic-loop-multi-exit-recipe-v0
new_if_mode=ThenOnlyExit
new_recipe_acceptance_shape=1
method_name_branch_added=0
json_native_source_changed=0
fixture_expected_fail_flipped_green=1
gate_case=selfhost_scanner_multi_exit_min
next_task=COREPLAN-CONTINUE-IN-STAGED-LOOP-001
summary=ok
```
