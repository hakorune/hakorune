---
Status: Active
Decision: accepted
Date: 2026-06-19
Scope: Inventory the current loop route debt before adding more named-shape
  patches.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1284-COREPLAN-LOOP-RESOLVER-REAGGREGATION-TASKBOARD-001.md
  - docs/development/current/main/design/loop-cond-break-continue-ssot.md
  - docs/development/current/main/design/recipe-tree-and-parts-ssot.md
  - docs/development/current/main/design/generic-loop-v1-acceptance-by-recipe-ssot.md
---

# COREPLAN-LOOP-ROUTE-DEBT-INVENTORY

## Decision

The active compiler debt is route ownership and loop-lowering carrier wiring,
not a missing PHI SSOT.

```text
phi_lifecycle_ssot_missing=0
named_loop_route_debt=1
registry_suppression_as_primary_fix=0
selected_short_term_patch=COREPLAN-CONTINUE-PARTIAL-CARRIER-PHI-001
selected_resolver_path=loop_resolver_b_lite
```

`LoopSimpleWhile` negative acceptance remains a valid compatibility cleanup if
its own predicate incorrectly accepts a break/continue-bearing loop. It is not
the current primary owner because the active failure shows a concrete
continue-edge carrier PHI wiring bug after a loop route is already selected.

## Evidence

Active fixture:

```text
apps/tests/phase29bq_selfhost_blocker_read_number_continue_staged_min.hako
```

Current expected-fail mode:

```text
[freeze:contract][mir/verify:dominator_violation]
use_block=BasicBlockId(19)
def_block=BasicBlockId(23)
value=ValueId(21)
inst=Copy { dst: ValueId(68), src: ValueId(21) }
def_inst=Phi { dst: ValueId(21), inputs: [(BasicBlockId(22), ValueId(8))] }
```

MIR shape summary:

```text
bb17:
  pos   = phi [entry, bb16], [step_pos, bb19]
  count = phi [entry, bb16], [step_count, bb19]
  seen  = phi [entry, bb16], [step_seen, bb19]

bb21:
  continue edge computes an edge-local pos+1 value
  count and seen are preserved from predecessor-dominating values
  br bb19

bb19:
  count/seen step PHIs exist
  pos step PHI is missing
  step_pos is recomputed from a sibling/fallthrough-only phi value
```

The invalid value is defined on a sibling/fallthrough path and does not dominate
the continue edge. The continue edge already has enough local information to
provide all carrier inputs; the lowerer must route that through the
`ContinueWithPhiArgs` / step-join contract.

## Route Family Inventory

```text
route=LoopSimpleWhile
status=legacy_named_shape
current_role=may still appear in route logs for surrounding/simple loops
primary_owner_for_active_failure=0
next_action=do not add registry suppression as primary fix

route=loop_cond_break_continue
status=legacy_named_shape
current_role=partial-carrier continue candidate family
primary_owner_for_active_failure=maybe_surface_owner
next_action=finish carrier/step PHI behavior through shared lowerer seam

route=loop_true_break_continue
status=legacy_named_shape
current_role=overlap candidate for true-condition break/continue loops
primary_owner_for_active_failure=0
next_action=observe in B-lite resolver shadow before further suppression

route=generic_loop_v1
status=recipe_route
current_role=accepted scanner multi-exit via ThenOnlyExit
primary_owner_for_active_failure=0
next_action=no broad rewrite before resolver shadow

route=LoopV0/CoreLoopPlan lowering
status=shared lowering seam
current_role=carrier frame / step PHI / deferred continue input wiring
primary_owner_for_active_failure=1
next_action=COREPLAN-CONTINUE-PARTIAL-CARRIER-PHI-001
```

## Why Not Another Suppression Patch

The current failure is not fixed by making the registry skip one more route
name. A registry suppression can reduce visible overlap, but it does not prove
that every selected loop route writes correct per-edge carrier inputs.

Allowed cleanup:

```text
LoopSimpleWhile predicate rejects break/continue in its own owner predicate.
```

Forbidden as primary fix:

```text
global registry suppression decides correctness between loop routes
```

## Next Task Order

```text
1. COREPLAN-CONTINUE-PARTIAL-CARRIER-PHI-001
   Fix explicit continue edges so every carrier gets an edge-dominating
   incoming value through the step join.

2. COREPLAN-LOOP-RESOLVER-B-LITE-SSOT-001
   Document the small resolver seam after the concrete fixture is green.

3. COREPLAN-LOOP-RESOLVER-SHADOW-001
   Run the resolver as report-only next to named routes.

4. COREPLAN-LOOP-ROUTE-RETIRE-SELECTION-001
   Pick the first named route/suppression path to retire from evidence.
```

BoxShape cleanup stays separate:

```text
COREPLAN-RECIPE-VERIFIED-SPLIT-001
COREPLAN-PARTS-STMT-SPLIT-001
COREPLAN-LOOPV0-PARTS-SPLIT-001
```

These are valid cleanup work, but they must not be mixed into the
partial-carrier continue acceptance row.

## Stop Lines

```text
do not add a new named loop route for this blocker
do not add registry suppression as the primary fix
do not make a new PHI SSOT
do not rewrite generic_loop_v1 broadly
do not change json_native or RustSubset app-front behavior
do not branch by method name / fixture name / source filename
```

## Report

```text
output_contract=coreplan-loop-route-debt-inventory-v0
implementation_changed=0
suppression_added=0
phi_lifecycle_ssot_missing=0
named_loop_route_debt=1
active_failure=continue_partial_carrier_step_phi
selected_short_term_patch=COREPLAN-CONTINUE-PARTIAL-CARRIER-PHI-001
selected_resolver_path=loop_resolver_b_lite
registry_suppression_as_primary_fix=0
summary=ok
```
