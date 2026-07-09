# 3268 - MIRBUILDER-RECIPEITEM-COND-RECIPE-OBSERVATION-BOUNDARY-SELECTION-001

Status: landed

## Scope

Select the first observer allowed to read the `RecipeItem.cond_recipe`
sidecar contents.

Selected observer:

```text
RecipeItemDiagnosticSummaryObserver
```

Selected next card:

```text
MIRBUILDER-RECIPEITEM-COND-RECIPE-DIAGNOSTIC-SUMMARY-OBSERVATION-001
```

## Reason

Diagnostic summary observation can prove the sidecar content without changing
verifier behavior, RecipeMatcher input authority, route selection, or lowering.

Rejected for now:

```text
RecipeVerifierObserver
RecipeMatcherInputObserver
LoweringObserver
```

## Fixture / Gate

Fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-recipeitem-cond-recipe-observation-boundary-selection-v0.json
```

Gate:

```text
tools/checks/rust_lifecycle_mirbuilder_recipeitem_cond_recipe_observation_boundary_selection_guard.sh
```

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_recipeitem_cond_recipe_observation_boundary_selection_guard.sh
```

## Claims

```text
cond_recipe_observation_boundary_selection=1
selected_diagnostic_summary_observer=1
```

## Non-Claims

```text
cond_recipe_deep_observation_implementation=0
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
bash tools/checks/rust_lifecycle_mirbuilder_recipeitem_cond_recipe_observation_boundary_selection_guard.sh
```

Expected summary:

```text
selected_observer=RecipeItemDiagnosticSummaryObserver
selected_next_card=MIRBUILDER-RECIPEITEM-COND-RECIPE-DIAGNOSTIC-SUMMARY-OBSERVATION-001
summary=ok
```

## Next

```text
MIRBUILDER-RECIPEITEM-COND-RECIPE-DIAGNOSTIC-SUMMARY-OBSERVATION-001
```
