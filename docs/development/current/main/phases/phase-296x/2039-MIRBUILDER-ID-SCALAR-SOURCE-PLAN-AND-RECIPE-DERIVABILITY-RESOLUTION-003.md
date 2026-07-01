# 2039 - MIRBUILDER-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-DERIVABILITY-RESOLUTION-003

## Token

```text
MIRBUILDER-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-DERIVABILITY-RESOLUTION-003
```

## Purpose

Rerun ID scalar SourcePlanAndRecipe derivability after all basis contracts are
available.

## Result

```text
input_candidate_count = 4
source_plan_derivable_count = 2
behavior_recipe_derivable_count = 2
selection_eligible_count = 2
ambiguous_derivable_count = 2

decision:
  KeepStopped

reason_token:
  MultipleEqualIdScalarSourcePlanDerivabilityCandidates

selected_next_card:
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
```

## Boundary

`context_registry` and `emission_ssa_phi` are both derivable. The card does
not choose between them by owner name, row count, lexical order, or manual
preference. A new machine-derived discriminator is required before
SourcePlanAndRecipe materialization.

## Non-Claims

```text
manual_owner_selection = 0
source_plan_materialization = 0
behavior_recipe_materialization = 0
verifier_result_materialization = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
source_selfhost_claim = 0
```
