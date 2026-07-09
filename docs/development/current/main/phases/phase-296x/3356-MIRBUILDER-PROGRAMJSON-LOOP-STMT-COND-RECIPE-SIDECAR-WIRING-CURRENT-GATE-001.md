# 3356 - MIRBUILDER-PROGRAMJSON-LOOP-STMT-COND-RECIPE-SIDECAR-WIRING-CURRENT-GATE-001

## Token

```text
MIRBUILDER-PROGRAMJSON-LOOP-STMT-COND-RECIPE-SIDECAR-WIRING-CURRENT-GATE-001
```

## Purpose

Consume the existing ProgramJSON LoopStmt condition-recipe sidecar wiring gate
in the current 335x chain.

This current wrapper proves `LoopStmtHandler` still attaches the optional
`cond_recipe` sidecar to loop `RecipeItem`s while the legacy `cond_facts` map
remains required.

## Result

```text
loop_stmt_cond_recipe_sidecar_wiring_current_gate = 1
loop_stmt_cond_recipe_sidecar_wiring = 1
recipe_item_attachment_implementation = 1
legacy_cond_facts_required = 1
current_wrapper = 1
source_selfhost_claim = 0
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_programjson_loop_stmt_cond_recipe_sidecar_wiring_current_guard.sh
```

## Selected Next

```text
selected_next_card:
  MIRBUILDER-RECIPEITEM-COND-RECIPE-OBSERVATION-BOUNDARY-SELECTION-001
```

## Non-Claims

```text
cond_recipe_deep_observation = 0
if_stmt_cond_recipe_wiring = 0
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
