# 2033 - MIRBUILDER-ID-SCALAR-SOURCE-PLAN-BASIS-COMPONENT-PRIORITY-RESOLUTION-002

## Token

```text
MIRBUILDER-ID-SCALAR-SOURCE-PLAN-BASIS-COMPONENT-PRIORITY-RESOLUTION-002
```

## Purpose

Rerun SourcePlan basis component priority after owner-scope boundedness and
native seed file boundary basis are available.

## Result

```text
bounded_owner_count = 2
native_seed_file_boundary_derivable_count = 2
state_target_count = 22

decision:
  SelectBasisComponent

selected_component_id:
  IdDomainBoundary

selected_next_card:
  MIRBUILDER-ID-SCALAR-ID-DOMAIN-BOUNDARY-BASIS-001
```

## Boundary

This card only selects the next basis component by dependency rule. It does
not materialize ID domain policy, SourcePlanAndRecipe, verifier fixtures, or a
native seed.

## Non-Claims

```text
manual_component_selection = 0
manual_owner_selection = 0
source_plan_materialization = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
source_selfhost_claim = 0
```
