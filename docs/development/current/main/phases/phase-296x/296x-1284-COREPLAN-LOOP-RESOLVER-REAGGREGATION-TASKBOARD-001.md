---
Status: Active
Decision: accepted
Date: 2026-06-19
Scope: Re-aggregate compiler loop/CorePlan tasks so the active fix does not
  keep growing named-shape debt.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1283-COREPLAN-CONTINUE-PHI-OWNER-DESIGN-001.md
  - docs/development/current/main/design/generic-loop-v1-acceptance-by-recipe-ssot.md
  - docs/development/current/main/design/recipe-tree-and-parts-ssot.md
  - docs/development/current/main/design/compiler-cleanliness-campaign-ssot.md
  - docs/development/current/main/design/loop-cond-break-continue-ssot.md
---

# COREPLAN-LOOP-RESOLVER-REAGGREGATION-TASKBOARD

## Decision

Treat the current staged-loop `continue` blocker as evidence that the old
named loop route family is still too expensive to maintain:

```text
current cost:
  N named loop shapes
  route-priority/suppression logic between shapes
  local fixes that preserve the old system

target:
  one loop resolver over Recipe/Exit/Carrier facts
  route labels become diagnostics/adapters
  suppression logic is retired
```

This does **not** reopen a broad rewrite immediately. The immediate failing
fixture remains real and must either be fixed by the smallest safe old-route
patch or parked while the resolver seam is implemented. The key change is task
ownership: do not spend more time perfecting `LoopSimpleWhile` / `LoopCond*`
priority rules than is needed to keep the lane moving toward the resolver.

## Re-aggregation Goal

The goal of this board is to stop spending compiler time on scattered named
loop-route debt. The target shape is:

```text
facts are frozen
  -> B-lite loop resolver decides Allow/Deny from facts
  -> existing named routes are observed beside the resolver
  -> one named-route/suppression path is selected for retirement
```

This is a BoxShape cleanup lane. It is not a language feature lane and must not
add a new accepted source shape unless a separate BoxCount card is opened.

Completion criteria for this re-aggregation slice:

```text
continue_partial_carrier_fixture_green=1
b_lite_resolver_ssot_written=1
resolver_shadow_report_exists=1
first_route_retire_candidate_selected=1
registry_suppression_as_primary_owner=0
new_named_loop_route_added=0
```

## Current Finding

PHI lifecycle is already SSOT-owned:

```text
docs/development/current/main/design/phi-lifecycle-ssot.md
docs/development/current/main/design/phi-input-strategy-ssot.md
```

The active failure is therefore not a new PHI SSOT problem. The failure is a
route ownership problem:

```text
LoopSimpleWhile can still claim loops with break/continue-shaped behavior.
That forces suppression patches and lets the wrong route build carrier PHIs.
```

## Task Order

Current status:

```text
COREPLAN-LOOP-ROUTE-DEBT-INVENTORY-001:
  done

LOOP-SIMPLE-WHILE-NEGATIVE-ACCEPTANCE-001:
  deferred
  reason=not_current_owner_after_debt_inventory

COREPLAN-CONTINUE-PARTIAL-CARRIER-PHI-001:
  done
  gate=selfhost_read_number_continue_staged_min green

COREPLAN-LOOP-RESOLVER-B-LITE-SSOT-001:
  done

COREPLAN-LOOP-RESOLVER-SHADOW-001:
  done

COREPLAN-LOOP-ROUTE-RETIRE-SELECTION-001:
  done

COREPLAN-LOOP-ROUTE-RETIRE-001:
  next
  target=registry_candidate_suppression
```

### 1. `COREPLAN-LOOP-ROUTE-DEBT-INVENTORY-001`

Docs/report only.

Purpose:

```text
List loop route families, their current acceptance predicates, and their
overlap with the active staged-loop fixtures.
```

Inventory fields:

```text
route=LoopSimpleWhile
route=loop_cond_break_continue
route=loop_cond_continue_only
route=loop_true_break_continue
route=generic_loop_v1

has_break
has_continue
has_continue_prelude_effect
has_partial_carrier_continue
has_step_join_required
selected_route
preempted_route
```

Acceptance:

```text
implementation_changed=0
suppression_added=0
selected_short_term_patch=<task|none>
selected_resolver_path=loop_resolver_b_lite
summary=ok
```

### 2. `LOOP-SIMPLE-WHILE-NEGATIVE-ACCEPTANCE-001`

Deferred compatibility patch, only if later shadow evidence shows
`LoopSimpleWhile` still incorrectly claims break/continue-bearing loops before
the resolver can retire that path.

Purpose:

```text
Make LoopSimpleWhile reject loops that contain break/continue or
continue-prelude effects.
```

Important boundary:

```text
owner=LoopSimpleWhile acceptance predicate
not owner=global registry suppression
```

This is allowed because it removes an invalid owner claim instead of adding a
new named-shape route.

Stop line:

```text
do not add another route-priority table branch unless the acceptance owner
cannot express the negative condition.
```

### 3. `COREPLAN-CONTINUE-PARTIAL-CARRIER-PHI-001`

Closed. The current fixture is green by moving v1 loop-var updates onto
edge-local `ContinueWithPhiArgs` instead of a shared step expression.

Acceptance:

```text
gate_case=selfhost_read_number_continue_staged_min
expected_output=3
planner_required_green=1
dominance_violation=0
method_name_branch=0
json_native_route_changed=0
```

Expected result:

```text
partial-carrier continue edges use ContinueWithPhiArgs / step join
preserved carriers come from edge-dominating values
```

### 4. `COREPLAN-LOOP-RESOLVER-B-LITE-SSOT-001`

Design + passive seam.

Purpose:

```text
Define a small loop resolver that decides loop route eligibility from
Recipe/Exit/Carrier facts instead of named source-shape labels.
```

Model:

```text
input:
  CanonicalLoopFacts
  RecipeBlock / VerifiedRecipeBlock
  Exit facts
  Carrier facts

output:
  Allow(LoopRouteFact)
  Deny(LoopRouteDenyReason)

rules:
  bool-only predicate=0
  resolver_mutates_facts=0
  reachability_feedback_to_resolver=0
  unknown_or_overlap=deny/freeze in strict planner_required
```

Non-goals:

```text
no backend lowering rewrite
no generic_loop_v1 wholesale conversion
no selfhost/app-front behavior changes
```

Required SSOT decisions:

```text
facts_freeze_before_resolver=1
resolver_reads_facts_only=1
resolver_returns_allow_or_deny=1
resolver_mutates_facts=0
reachability_feedback_to_resolver=0
deny_reason_has_next_owner=1
unknown_or_overlap_in_strict=deny_or_freeze
```

Minimum reason-to-owner map:

```text
Deny(NoRecipeFacts):
  owner=recipe_fact_producer

Deny(OverlappingNamedRoutes):
  owner=loop_route_retire_selection

Deny(MissingCarrierInputs):
  owner=carrier_frame_or_continue_phi

Deny(UnsupportedExitShape):
  owner=exit_fact_producer

Deny(Unknown):
  owner=fixture_inventory
```

### 5. `COREPLAN-LOOP-RESOLVER-SHADOW-001`

Report-only.

Purpose:

```text
Run the B-lite resolver as an observer next to existing named routes and report
where route selection disagrees.
```

Acceptance:

```text
behavior_changed=0
shadow_loop_resolver_enabled=1
selected_named_route=<route>
resolver_decision=<allow|deny>
resolver_would_select=<route|none>
overlap_count=<n>
summary=ok
```

The shadow observer must not change the selected lowering route. It only
records:

```text
named_route_selected=<route>
resolver_decision=<allow|deny>
resolver_reason=<reason|none>
resolver_would_select=<route|none>
route_disagreement=0|1
```

### 6. `COREPLAN-LOOP-ROUTE-RETIRE-SELECTION-001`

Only after shadow evidence.

Purpose:

```text
Choose which named route becomes a thin diagnostic adapter or is retired first.
```

Preferred first retire target:

```text
LoopSimpleWhile overlap with break/continue-bearing loops
```

Selection order:

```text
1. retire a global registry suppression path if one is still live
2. narrow a named route predicate if it claims a shape the resolver denies
3. convert a named route into a diagnostic adapter if the resolver fully covers it
```

Acceptance:

```text
retire_candidate_selected=1
evidence_source=resolver_shadow
implementation_changed=0
new_named_route_added=0
summary=ok
```

### 7. BoxShape cleanup queue after fixture green

Keep these behavior-preserving splits separate from acceptance work:

```text
COREPLAN-RECIPE-VERIFIED-SPLIT-001
COREPLAN-PARTS-STMT-SPLIT-001
COREPLAN-LOOPV0-PARTS-SPLIT-001
```

They reduce the implementation cost of the resolver, but they must not be
mixed with the active fixture fix.

## Current Compiler Task List

Immediate:

```text
1. COREPLAN-LOOP-ROUTE-RETIRE-001
   Implement only the selected registry candidate suppression retirement.
```

After retire selection:

```text
2. COREPLAN-RECIPE-VERIFIED-SPLIT-001
   BoxShape cleanup if recipe verification code is still too dense.

3. COREPLAN-PARTS-STMT-SPLIT-001
   Split statement-part analysis only after route ownership is simpler.

4. COREPLAN-LOOPV0-PARTS-SPLIT-001
   Split old loop lowering internals only after the resolver identifies what
   remains live.
```

Parked:

```text
read_next_number_literal full shape:
  queued until a new minimal failing fixture proves a compiler owner

LoopSimpleWhile negative acceptance:
  deferred until shadow evidence says it is still the best first cleanup

new app-front RustSubset syntax:
  paused unless a new real input blocker appears
```

Do not start:

```text
new named loop route
registry suppression as correctness owner
fresh PHI SSOT
json_native / RustSubset app-front changes
VM product-route recovery
```

## Stop Lines

```text
do not create a new PHI SSOT
do not implement read_next_number_literal by name
do not add another named loop shape for this blocker
do not keep adding registry suppression as the primary fix
do not rewrite generic_loop_v1 broadly before inventory/shadow evidence
do not mix RustSubset/json_native app-front work into this compiler task
```

## Report

```text
output_contract=coreplan-loop-resolver-reaggregation-taskboard-v0
active_blocker=COREPLAN-LOOP-ROUTE-RETIRE-001
completed_task=COREPLAN-LOOP-ROUTE-DEBT-INVENTORY-001
completed_task=COREPLAN-CONTINUE-PARTIAL-CARRIER-PHI-001
completed_task=COREPLAN-LOOP-RESOLVER-B-LITE-SSOT-001
completed_task=COREPLAN-LOOP-RESOLVER-SHADOW-001
completed_task=COREPLAN-LOOP-ROUTE-RETIRE-SELECTION-001
deferred_task=LOOP-SIMPLE-WHILE-NEGATIVE-ACCEPTANCE-001
current_bug_class=route_owner_selection
phi_ssot_missing=0
named_loop_shape_debt=1
loop_resolver_direction=accepted_b_lite
current_fix_owner=registry_candidate_suppression_retire
next_task=COREPLAN-LOOP-ROUTE-RETIRE-001
compiler_reaggregation_completion_target=selected_retire_implemented
short_term_old_route_patch_allowed=only_after_shadow_evidence
registry_suppression_as_primary_fix_allowed=0
active_fixture=selfhost_read_number_continue_staged_min
summary=ok
```
