---
Status: Active
Decision: accepted
Date: 2026-06-19
Scope: Select and fix the owner for the existing
  loop_continue_only_multidelta_min gate debt.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1292-COREPLAN-LOOP-SUPPRESSION-FULL-INVENTORY-001.md
  - docs/development/current/main/phases/phase-296x/296x-1286-COREPLAN-CONTINUE-PARTIAL-CARRIER-PHI-001.md
---

# COREPLAN-LOOP-MULTIDELTA-OWNER-SELECTION

## Decision

`loop_continue_only_multidelta_min` is not owned by `loop_continue_only`.
The fixture name is historical; the shape contains:

```text
break_count=1
continue_count=2
continue branch prelude effects=1
```

The selected owner is `loop_cond_break_continue` because it already has the
recipe vocabulary for continue branches with prelude effects and emits
edge-local `ContinueWithPhiArgs` through the existing step-join contract.

## Implementation

Two narrow changes were made.

1. `loop_continue_only` facts now reject anything except exactly one continue.
   Its facts carrier has one `continue_condition` and one `loop_increment`, so
   accepting multi-continue/multi-delta shapes creates a partial truth.

2. The facts builder no longer nulls `loop_cond_break_continue` just because
   `generic_loop_v1` also has a recipe hint when the loop-cond facts contain
   multiple continue branches with assignment/local prelude effects.

This is not a general route-priority rewrite. It only keeps the already-built
`loop_cond_break_continue` facts for the shape that requires the recipe owner.

## Evidence

Focused gates:

```bash
NYASH_BIN=target/debug/hakorune \
  bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh \
  --only loop_continue_only_multidelta_min
```

Result:

```text
[PASS] phase29bq_fast_gate_cases:loop_continue_only_multidelta_min
```

Planner-required wrapper:

```bash
NYASH_BIN=target/debug/hakorune \
  bash tools/smokes/v2/profiles/integration/joinir/loop_continue_only_multidelta_planner_required_vm.sh
```

Result:

```text
[PASS] loop_continue_only_multidelta_planner_required_vm: PASS
```

Suppression inventory focused on the former blocker:

```bash
python3 tools/smokes/v2/profiles/integration/joinir/phase29bq_loop_route_suppression_inventory.py \
  --only loop_continue_only_multidelta_min \
  --bin target/debug/hakorune \
  --timeout 15
```

Result:

```text
output_contract=coreplan-loop-suppression-full-inventory-v0
case_count=1
observer_case_count=1
actual_selected_case_count=1
suppressed_non_none_case_count=0
actual_selected_route_counts=loop_cond_break_continue:1
suppressed_route_counts=none
failure_masking=0
sampling_limit=none
summary=ok
```

Unit guard:

```bash
cargo test -q accepts_multidelta_break_continue_as_recipe_only
```

Result:

```text
1 passed
```

Full phase gate note:

```bash
cargo build --features vm-reference --bin hakorune
NYASH_BIN=target/debug/hakorune \
  bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh
```

Result:

```text
multidelta fixture passed
phase29bq_mir_preflight_unsupported_reject_vm passed
next blocker=phase29bq_selfhost_blocker_parse_stmt_skipws_min
expected=1
actual=2
selected_route=generic_loop_v1
```

This is outside the multidelta owner-selection row. It is the next loop route
owner-selection debt exposed after the previous fail-closed preflight script
blocker was fixed to run with `--features vm-reference`.

## Stop Lines

```text
do not treat historical fixture names as route ownership
do not let loop_continue_only carry multi-continue state
do not null loop_cond_break_continue facts when continue prelude effects require recipe lowering
do not delete registry suppression from this row
do not promote the legacy observer to an independent resolver
```

## Next

The full suppression inventory can now continue beyond the former first
blocker. The next row should run that inventory fail-closed and select the
smallest proven suppression-retirement target, if any.

```text
next_task=COREPLAN-LOOP-SUPPRESSION-FULL-INVENTORY-CONTINUE-001
next_full_gate_blocker=COREPLAN-PARSE-STMT-SKIPWS-OWNER-SELECTION-001
```

## Report

```text
output_contract=coreplan-loop-multidelta-owner-selection-v0
selected_owner=loop_cond_break_continue
loop_continue_only_multicontinue_rejected=1
loop_cond_break_continue_recipe_owner_kept_for_multicontinue_prelude=1
generic_loop_v1_priority_narrowed=1
registry_suppression_deleted=0
focused_fast_gate_green=1
planner_required_wrapper_green=1
focused_inventory_green=1
full_phase_gate_progressed_to_next_blocker=phase29bq_selfhost_blocker_parse_stmt_skipws_min
summary=ok
```
