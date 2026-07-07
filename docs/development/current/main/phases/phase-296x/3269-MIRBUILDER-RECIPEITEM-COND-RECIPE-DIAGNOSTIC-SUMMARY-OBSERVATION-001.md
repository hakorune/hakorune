# 3269 - MIRBUILDER-RECIPEITEM-COND-RECIPE-DIAGNOSTIC-SUMMARY-OBSERVATION-001

Status: landed

## Scope

Read the `RecipeItem.cond_recipe` sidecar contents through the diagnostic
summary observer.

This proves deep observation of the sidecar in AOT without letting verifier,
RecipeMatcher, lowering, route selection, or runtime switch consume it.

## Fixture / Gate

Fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-recipeitem-cond-recipe-diagnostic-summary-observation-v0.json
```

Gate:

```text
tools/checks/rust_lifecycle_mirbuilder_recipeitem_cond_recipe_diagnostic_summary_observation_gate.sh
```

## Claims

```text
cond_recipe_diagnostic_summary_observation=1
cond_recipe_deep_observation_implementation=1
```

## Non-Claims

```text
verifier_cond_recipe_observer=0
recipe_matcher_input_authority=0
bool_recipe_lowering=0
mir_cmp_emission=0
branch_emission=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
runtime_fallback=0
source_selfhost_claim=0
```

## Verification

```bash
bash tools/checks/rust_lifecycle_mirbuilder_recipeitem_cond_recipe_diagnostic_summary_observation_gate.sh
```

Expected summary:

```text
cond_recipe_diagnostic_summary_observation=1
cond_recipe_deep_observation_implementation=1
summary=ok
```

## Next

```text
MIRBUILDER-RECIPEITEM-COND-RECIPE-CONSUME-BOUNDARY-SELECTION-001
```
