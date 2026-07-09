# 3359 - MIRBUILDER-RECIPEITEM-COND-RECIPE-CONSUME-BOUNDARY-SELECTION-CURRENT-001

## Token

```text
MIRBUILDER-RECIPEITEM-COND-RECIPE-CONSUME-BOUNDARY-SELECTION-CURRENT-001
```

## Purpose

Consume the existing `RecipeItem.cond_recipe` consume-boundary selection in the
current 335x chain.

This current wrapper proves the selected first non-diagnostic consumer remains
`RecipeVerifierValidateOnlyConsumer` and forwards to the validate-only
RecipeVerifier consume gate.

## Result

```text
cond_recipe_consume_boundary_selection_current = 1
cond_recipe_consume_boundary_selection = 1
selected_recipeverifier_validate_only_consumer = 1
selected_consumer = RecipeVerifierValidateOnlyConsumer
current_wrapper = 1
source_selfhost_claim = 0
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_recipeitem_cond_recipe_consume_boundary_selection_current_guard.sh
```

## Selected Next

```text
selected_next_card:
  MIRBUILDER-RECIPEVERIFIER-COND-RECIPE-VALIDATE-ONLY-CONSUME-001
```

## Non-Claims

```text
recipeverifier_cond_recipe_consume_implementation = 0
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
