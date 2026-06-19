---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Fix ownership and task order for the partial-carrier continue PHI blocker.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1282-COREPLAN-REAL-SHAPE-TASKBOARD-REFRESH-003.md
  - docs/development/current/main/design/phi-lifecycle-ssot.md
  - docs/development/current/main/design/phi-input-strategy-ssot.md
  - docs/development/current/main/design/loop-cond-break-continue-ssot.md
  - docs/development/current/main/design/recipe-tree-and-parts-ssot.md
---

# COREPLAN-CONTINUE-PHI-OWNER-DESIGN

## Decision

Keep the active blocker narrow:

```text
COREPLAN-CONTINUE-PARTIAL-CARRIER-PHI-001
```

This is a `loop_cond_break_continue` PHI ownership issue, not a broad
`generic_loop_v1` rewrite and not a RustSubset/json_native app-front issue.

This card does **not** define a new PHI SSOT. Existing PHI ownership remains:

```text
PHI lifecycle / insertion / patching:
  docs/development/current/main/design/phi-lifecycle-ssot.md

PHI input shape vocabulary:
  docs/development/current/main/design/phi-input-strategy-ssot.md

loop_cond_break_continue per-edge continue carrier semantics:
  docs/development/current/main/design/loop-cond-break-continue-ssot.md
```

This row only fixes the missing leaf contract for partial-carrier continue
edges under the existing PHI SSOTs.

The current failing fixture is:

```text
apps/tests/phase29bq_selfhost_blocker_read_number_continue_staged_min.hako
```

The fixed contract is:

```text
continue edges in a loop-carried body must provide values for every carrier.
updated carrier:
  use the edge-local updated value

preserved carrier:
  use the predecessor-dominating value for that edge

never:
  use a join value defined only on a sibling/fallthrough path
```

## Worker Audit Summary

The route inventory found these loop routes relevant to the fixture family:

```text
LoopSimpleWhile:
  simple loop skeleton; not owner for break/continue carrier joins

loop_cond_break_continue:
  current owner for loop(cond) + break/continue + carrier PHI joins

loop_cond_continue_only:
  continue-only route; not owner for this break-bearing fixture

generic_loop_v1:
  final canonical intake direction, but currently preempted by narrower route
```

The active fixture belongs to:

```text
owner=loop_cond_break_continue partial carrier PHI handling
```

The canonical primitive already exists:

```text
CoreExitPlan::ContinueWithPhiArgs
```

Its responsibility is positive and complete: a continue edge must carry all
loop-carrier incoming values into the step join. It is not a hint, and it is
not optional once a loop body has carrier PHIs.

## Root Cause

The fixture has a separator branch that updates only `pos` and then continues.
The remaining loop-carried state (`count`, `seen`) must be preserved on that
edge.

The failing shape is caused when the route sends a continue edge directly to
the loop header or otherwise allows a sibling/fallthrough join value to become
the incoming value for the continue edge. That creates a non-dominating PHI
input and is reported as:

```text
failure=mir/verify:dominator_violation
```

This is not evidence for:

```text
read_next_number_literal method-name handling
json_native route changes
broad recursive Recipe rewrite
generic_loop_v0 extension
```

## Ownership

```text
facts/recipe:
  decide that this route is loop_cond_break_continue
  classify continue/break/effect items

parts/exit.rs:
  build ContinueWithPhiArgs for all carriers

loop_cond_bc_continue_if.rs:
  route-local explicit continue branch conversion

loop_cond_bc_cleanup.rs:
  synthetic fallthrough ContinueWithPhiArgs

loop_cond_bc_phi_materializer.rs:
  materialize carrier header/step PHIs consistently

exit_lowering.rs:
  lower ContinueWithPhiArgs into LoopFrame PHI inputs
```

No other layer should infer this from source names, helper names, or app-front
fixtures.

## Implementation Task

```text
COREPLAN-CONTINUE-PARTIAL-CARRIER-PHI-001
```

Scope:

```text
touch target:
  src/mir/builder/control_flow/plan/features/loop_cond_bc_phi_materializer.rs
  src/mir/builder/control_flow/plan/features/loop_cond_bc_continue_if.rs
  adjacent tests/fixtures only if needed

acceptance:
  gate_case=selfhost_read_number_continue_staged_min
  expected_output=3
  planner_required_green=1
  method_name_branch=0
  json_native_route_changed=0
  single_acceptance_shape_added=1
```

Expected implementation direction:

```text
partial-carrier continue:
  route through the step join PHI path
  carry all carriers through ContinueWithPhiArgs
  preserve non-updated carriers from edge-dominating bindings

do not:
  send partial-carrier continue directly to header PHIs
  source preserved carrier values from sibling/fallthrough-only joins
```

## Follow-up BoxShape Tasks

These are structural cleanups. They must not be mixed into the active
BoxCount-style acceptance fix.

```text
COREPLAN-RECIPE-PARTS-BOXSPLIT-SSOT-001
  update recipe-tree-and-parts SSOT with current drift:
    NoExit still carries LoopV0 today
    parts/stmt still owns containers today
  define staged cleanup order

COREPLAN-RECIPE-VERIFIED-SPLIT-001
  split recipe_tree/verified.rs into port_sig / block_contract / exit_shape
  behavior_changed=0

COREPLAN-PARTS-STMT-SPLIT-001
  split parts/stmt.rs into simple / if_join / return_prelude /
  blockexpr_prelude / containers
  behavior_changed=0

COREPLAN-LOOPV0-PARTS-SPLIT-001
  split parts/loop_/loop_v0.rs into carriers / frame / body_dispatch /
  phis / final_values
  behavior_changed=0

COREPLAN-GENERIC-LOOP-V1-UNIFICATION-SELECTION-001
  after the active fixture is green, decide whether loop_cond_break_continue
  stays as a specialized adapter or folds toward generic_loop_v1
```

## Stop Lines

```text
do not implement count_digits_skip_sep by name
do not implement read_next_number_literal by name
do not change json_native or RustSubset converter code for this blocker
do not add a broad recursive Recipe rewrite while this narrow owner exists
do not extend generic_loop_v0 as the active fix
do not remove count/seen carriers from the fixture to pass
do not let Lower re-derive route acceptance from unverified Recipe
```

## Report

```text
output_contract=coreplan-continue-phi-owner-design-v0
active_task=COREPLAN-CONTINUE-PARTIAL-CARRIER-PHI-001
owner=loop_cond_break_continue
canonical_exit_primitive=ContinueWithPhiArgs
continue_edges_must_carry_all_carriers=1
partial_carrier_continue_uses_step_join=1
header_direct_partial_continue_allowed=0
generic_loop_v1_broad_rewrite_allowed=0
json_native_route_changed=0
implementation_started=0
summary=ok
```
