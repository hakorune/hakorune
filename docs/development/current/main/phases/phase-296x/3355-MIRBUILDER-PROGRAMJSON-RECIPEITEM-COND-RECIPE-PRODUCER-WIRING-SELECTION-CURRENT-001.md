# 3355 - MIRBUILDER-PROGRAMJSON-RECIPEITEM-COND-RECIPE-PRODUCER-WIRING-SELECTION-CURRENT-001

## Token

```text
MIRBUILDER-PROGRAMJSON-RECIPEITEM-COND-RECIPE-PRODUCER-WIRING-SELECTION-CURRENT-001
```

## Purpose

Consume the existing ProgramJSON `RecipeItem.cond_recipe` producer wiring
selection in the current 335x chain.

This current wrapper proves the selected producer remains
`LoopStmtHandlerLoopConditionProducer` and forwards the next task to the
existing LoopStmt condition-recipe sidecar wiring card.

## Result

```text
cond_recipe_producer_wiring_selection_current = 1
cond_recipe_producer_wiring_selection = 1
selected_loop_stmt_handler_cond_recipe_producer = 1
selected_producer = LoopStmtHandlerLoopConditionProducer
current_wrapper = 1
source_selfhost_claim = 0
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_programjson_recipeitem_cond_recipe_producer_wiring_selection_current_guard.sh
```

## Selected Next

```text
selected_next_card:
  MIRBUILDER-PROGRAMJSON-LOOP-STMT-COND-RECIPE-SIDECAR-WIRING-001
```

## Non-Claims

```text
recipe_item_attachment_implementation = 0
if_stmt_cond_recipe_wiring = 0
posthoc_recipeitem_decoration = 0
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
