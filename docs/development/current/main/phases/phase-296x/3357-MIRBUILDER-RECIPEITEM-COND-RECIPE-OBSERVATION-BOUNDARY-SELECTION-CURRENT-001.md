# 3357 - MIRBUILDER-RECIPEITEM-COND-RECIPE-OBSERVATION-BOUNDARY-SELECTION-CURRENT-001

## Token

```text
MIRBUILDER-RECIPEITEM-COND-RECIPE-OBSERVATION-BOUNDARY-SELECTION-CURRENT-001
```

## Purpose

Consume the existing `RecipeItem.cond_recipe` observation-boundary selection in
the current 335x chain.

This current wrapper proves the selected observer remains
`RecipeItemDiagnosticSummaryObserver`, keeping verifier behavior,
RecipeMatcher input authority, route selection, and lowering unchanged.

## Result

```text
cond_recipe_observation_boundary_selection_current = 1
cond_recipe_observation_boundary_selection = 1
selected_diagnostic_summary_observer = 1
selected_observer = RecipeItemDiagnosticSummaryObserver
current_wrapper = 1
source_selfhost_claim = 0
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_recipeitem_cond_recipe_observation_boundary_selection_current_guard.sh
```

## Selected Next

```text
selected_next_card:
  MIRBUILDER-RECIPEITEM-COND-RECIPE-DIAGNOSTIC-SUMMARY-OBSERVATION-001
```

## Non-Claims

```text
cond_recipe_deep_observation_implementation = 0
verifier_cond_recipe_observer = 0
recipe_matcher_input_authority = 0
bool_recipe_lowering = 0
mir_cmp_emission = 0
branch_emission = 0
route_selection = 0
runtime_route_switch = 0
programjson_runtime_route_authority = 0
runtime_fallback = 0
source_selfhost_claim = 0
```
