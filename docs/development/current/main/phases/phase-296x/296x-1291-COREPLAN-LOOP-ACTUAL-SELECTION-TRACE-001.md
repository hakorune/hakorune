---
Status: Active
Decision: accepted
Date: 2026-06-19
Scope: Record the legacy route that actually succeeds, instead of relying on
  candidate-only observation.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1290-COREPLAN-LOOP-LEGACY-OBSERVER-001.md
  - docs/development/current/main/phases/phase-296x/296x-1289-COREPLAN-LOOP-ROUTE-RETIRE-SELECTION-001.md
---

# COREPLAN-LOOP-ACTUAL-SELECTION-TRACE

## Decision

Candidate lists are not enough to compare future resolver decisions with the
legacy route system. A registry entry can match its predicate and still return
`Ok(None)` after mode checks, recipe contract checks, compose, verify, or
lowering.

Add an actual selection seam:

```text
LegacyRouteSuccess {
  route: LoopRouteId,
  value: ValueId,
}
```

The router now records the route whose handler actually returns `Some(value)`.
This is still diagnostic only and does not feed back into resolver decisions.

## Implementation

Code changes:

```text
src/mir/builder/control_flow/joinir/route_entry/registry/types.rs
src/mir/builder/control_flow/joinir/route_entry/registry/mod.rs
src/mir/builder/control_flow/joinir/route_entry/router.rs
```

Implementation details:

```text
LegacyRouteSuccess added
try_route_recipe_first_with_success returns Option<LegacyRouteSuccess>
router emits [plan/trace:loop_legacy_selected] route=<route>
legacy candidate observer remains separate
feedback_to_resolver=0
```

## Boundary

Allowed:

```text
record actual legacy selected route
keep route selection behavior unchanged
keep observer diagnostic-only
```

Not allowed:

```text
do not use actual selection to rewrite resolver decisions
do not promote legacy observer to route selection
do not remove loop_cond_break_continue global suppression in this row
```

## Evidence

Local gates:

```bash
cargo test -q registry::tests::
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only selfhost_read_number_continue_staged_min
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Result:

```text
targeted gates green; full suppression inventory is intentionally deferred to
the next row because it must fail-closed on existing fast-gate debt rather than
mask failures.
```

## Next Row

```text
next_task=COREPLAN-LOOP-SUPPRESSION-FULL-INVENTORY-001
```

The next row should use actual selected route traces to inventory remaining
suppression seams without `head -80` sampling or `|| true` failure masking.

## Report

```text
output_contract=coreplan-loop-actual-selection-trace-v0
implementation_changed=1
behavior_changed=0
legacy_route_success_enabled=1
actual_selected_route_trace_enabled=1
candidate_only_observation_retained=1
feedback_to_resolver=0
next_task=COREPLAN-LOOP-SUPPRESSION-FULL-INVENTORY-001
summary=ok
```
