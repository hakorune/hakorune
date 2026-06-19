---
Status: Active
Decision: accepted
Date: 2026-06-19
Scope: Select and close the step-placement owner for
  phase29bq_selfhost_blocker_scan_methods_nested_loop_state_machine_min.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1300-COREPLAN-SCAN-METHODS-NESTED-LOOP-DEPTH1-NO-BREAK-OR-CONTINUE-PURE-OWNER-SELECTION-001.md
---

# COREPLAN-SCAN-METHODS-NESTED-LOOP-STATE-MACHINE-STEP-PLACEMENT-OWNER-SELECTION

## Decision

`phase29bq_selfhost_blocker_scan_methods_nested_loop_state_machine_min`
was a real compiler blocker.

The focused gate failed under planner-required mode with:

```text
generic_loop_v1 pipeline failed
reason=no_valid_loop_var_candidates
raw=2
no_increment=1
control_flow_after_step=1
```

The failing loop is the inner scanner state-machine loop. It has several
branch-local `j = j + 1; continue` exits plus a tail `j = j + 1; continue`.
Treating one canonical top-level assignment as a shared loop step is too early:
later control-flow in the same body makes `control_flow_after_step=1`.

The selected owner is therefore:

```text
owner=generic_loop_v1_step_resolution
selected_shape=preferred_loop_var_with_current_loop_continue_and_loop_var_assignment
resolution=body_managed_step_before_canon
```

For this shape, `generic_loop_v1` now keeps the step in the recipe body and lets
the existing edge-local `ContinueWithPhiArgs` / Recipe path own loop-var updates.
This avoids adding a new PHI SSOT or a named route special case.

## Implementation

Code changes:

```text
src/mir/builder/control_flow/plan/generic_loop/facts/extract/v1.rs
```

Guard test:

```text
src/mir/builder/control_flow/plan/generic_loop/facts/extract/tests.rs
```

The new rule is structural:

```text
preferred_loop_var == candidate
current-loop continue exists
current-loop loop-var assignment exists
```

Nested loops are boundaries for the current-loop checks, so an outer loop does
not become body-managed just because a nested loop contains `continue`.

Two stale route expectations also became visible after the focused blocker was
fixed. They are semantic green and now use the stable flowbox adoption tag:

```text
phase29bq_selfhost_blocker_scan_all_boxes_empty_then_min
phase29bq_selfhost_blocker_scan_all_boxes_program_stmt_min
```

## Evidence

Regression unit:

```bash
cargo test -q generic_loop_v1_accepts_state_machine_continue_steps
```

Result:

```text
pass
```

Outer-loop guard:

```bash
cargo test -q generic_loop_v1_accepts_outer_step_after_nested_state_machine_loop
```

Result:

```text
pass
```

Focused gate:

```bash
cargo build -q --bin hakorune --features vm-reference
NYASH_BIN=target/debug/hakorune \
  bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh \
  --only selfhost_scan_methods_nested_loop_state_machine_min
```

Result:

```text
pass
```

Expectation cleanup focused gates:

```bash
NYASH_BIN=target/debug/hakorune \
  bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh \
  --only selfhost_scan_all_boxes_empty_then_min

NYASH_BIN=target/debug/hakorune \
  bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh \
  --only selfhost_scan_all_boxes_program_stmt_min
```

Result:

```text
pass
pass
```

Static checks:

```bash
cargo check --all-targets -q
git diff --check
```

Result:

```text
pass
pass
```

Full phase gate now progresses to:

```text
phase29bq_joinir_scan_loop_comma_close_min
failure=missing planner-first LoopCondBreak tag
observed=[joinir/planner_first rule=LoopSimpleWhile] label=LoopSimpleWhile
observed=[flowbox/adopt box_kind=Loop features=break,continue via=shadow]
stdout=0
rc=0
```

## Stop Lines

```text
do not add a fixture-name or helper-name branch
do not add a new PHI SSOT for this shape
do not treat nested-loop continue as current-loop continue
do not keep chasing historical planner-first tags as semantic failures
```

## Next

The next visible blocker is expectation/owner selection for
`phase29bq_joinir_scan_loop_comma_close_min`.

```text
next_task=COREPLAN-SCAN-LOOP-COMMA-CLOSE-OWNER-SELECTION-001
```

## Report

```text
output_contract=coreplan-scan-methods-nested-loop-state-machine-step-placement-owner-selection-v0
compiler_code_changed=1
selected_owner=generic_loop_v1_step_resolution
body_managed_step_before_canon_enabled=1
nested_loop_boundary_preserved=1
state_machine_unit_guard_green=1
focused_gate_green=1
scan_all_boxes_expectation_cleanup_count=2
cargo_check_all_targets_green=1
full_phase_gate_progressed_to_next_blocker=phase29bq_joinir_scan_loop_comma_close_min
summary=ok
```
