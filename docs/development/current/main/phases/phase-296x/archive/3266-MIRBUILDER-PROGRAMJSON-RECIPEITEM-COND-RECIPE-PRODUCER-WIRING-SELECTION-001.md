# 3266 - MIRBUILDER-PROGRAMJSON-RECIPEITEM-COND-RECIPE-PRODUCER-WIRING-SELECTION-001

Status: landed

## Scope

Select the first ProgramJSON producer allowed to attach the optional
`cond_recipe` sidecar to a `RecipeItem`.

Selected producer:

```text
LoopStmtHandlerLoopConditionProducer
```

Selected next card:

```text
MIRBUILDER-PROGRAMJSON-LOOP-STMT-COND-RECIPE-SIDECAR-WIRING-001
```

## Reason

The available `BoolRecipeCompareV1` publication is derived from loop
`CanonicalLoopFacts`. The first producer wiring should therefore be the
`LoopStmtHandler` loop condition slot, not general `IfStmtHandler` conditions
or post-hoc RecipeItem decoration.

This keeps the producer boundary explicit:

```text
ProgramJSON loop condition
  -> CanonicalLoopFacts numeric compare consume
  -> BoolRecipeCompareV1 data vocabulary
  -> Loop RecipeItem cond_recipe sidecar
```

## Fixture / Gate

Fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-recipeitem-cond-recipe-producer-wiring-selection-v0.json
```

Gate:

```text
tools/checks/rust_lifecycle_mirbuilder_programjson_recipeitem_cond_recipe_producer_wiring_selection_guard.sh
```

## Claims

```text
cond_recipe_producer_wiring_selection=1
selected_loop_stmt_handler_cond_recipe_producer=1
```

## Non-Claims

```text
recipe_item_attachment_implementation=0
if_stmt_cond_recipe_wiring=0
posthoc_recipeitem_decoration=0
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
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipeitem_cond_recipe_producer_wiring_selection_guard.sh
```

Expected summary:

```text
selected_producer=LoopStmtHandlerLoopConditionProducer
selected_next_card=MIRBUILDER-PROGRAMJSON-LOOP-STMT-COND-RECIPE-SIDECAR-WIRING-001
summary=ok
```

## Next

```text
MIRBUILDER-PROGRAMJSON-LOOP-STMT-COND-RECIPE-SIDECAR-WIRING-001
```
