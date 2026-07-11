# 3270 - MIRBUILDER-RECIPEITEM-COND-RECIPE-CONSUME-BOUNDARY-SELECTION-001

Status: landed

## Scope

Select the first non-diagnostic consumer allowed to inspect
`RecipeItem.cond_recipe`.

Selected consumer:

```text
RecipeVerifierValidateOnlyConsumer
```

Selected next card:

```text
MIRBUILDER-RECIPEVERIFIER-COND-RECIPE-VALIDATE-ONLY-CONSUME-001
```

## Reason

Verifier validate-only is the narrowest non-diagnostic boundary. It can reject
malformed optional `cond_recipe` without making it RecipeMatcher input
authority, route-selection input, or lowering input.

## Fixture / Gate

Fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-recipeitem-cond-recipe-consume-boundary-selection-v0.json
```

Gate:

```text
tools/checks/rust_lifecycle_mirbuilder_recipeitem_cond_recipe_consume_boundary_selection_guard.sh
```

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_recipeitem_cond_recipe_consume_boundary_selection_guard.sh
```

## Claims

```text
cond_recipe_consume_boundary_selection=1
selected_recipeverifier_validate_only_consumer=1
```

## Non-Claims

```text
recipeverifier_cond_recipe_consume_implementation=0
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
bash tools/checks/rust_lifecycle_mirbuilder_recipeitem_cond_recipe_consume_boundary_selection_guard.sh
```

Expected summary:

```text
selected_consumer=RecipeVerifierValidateOnlyConsumer
selected_next_card=MIRBUILDER-RECIPEVERIFIER-COND-RECIPE-VALIDATE-ONLY-CONSUME-001
summary=ok
```

## Next

```text
MIRBUILDER-RECIPEVERIFIER-COND-RECIPE-VALIDATE-ONLY-CONSUME-001
```
