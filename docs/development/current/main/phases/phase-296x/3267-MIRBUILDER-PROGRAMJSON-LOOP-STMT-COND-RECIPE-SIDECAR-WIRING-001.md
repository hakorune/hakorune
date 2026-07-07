# 3267 - MIRBUILDER-PROGRAMJSON-LOOP-STMT-COND-RECIPE-SIDECAR-WIRING-001

Status: landed

## Scope

Wire `LoopStmtHandler` to attach the optional `cond_recipe` sidecar to Loop
`RecipeItem`s when the ProgramJSON BoolRecipe publication is available.

The legacy `cond_facts` map remains required and is still built by the handler.
This card does not make downstream readers inspect the sidecar contents.

## Implementation

Owner:

```text
lang/src/compiler/mirbuilder/stmt_handlers/loop_stmt_handler.hako
```

Path:

```text
ProgramJSON loop condition
  -> LoopStmtHandler condition observation
  -> BoolRecipeBox.from_numeric_compare_codes
  -> Loop RecipeItem cond_recipe sidecar field
```

## Fixture / Gate

Fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-loop-stmt-cond-recipe-sidecar-wiring-v0.json
```

Gate:

```text
tools/checks/rust_lifecycle_mirbuilder_programjson_loop_stmt_cond_recipe_sidecar_wiring_gate.sh
```

## Claims

```text
loop_stmt_cond_recipe_sidecar_wiring=1
recipe_item_attachment_implementation=1
legacy_cond_facts_required=1
```

## Non-Claims

```text
cond_recipe_deep_observation=0
if_stmt_cond_recipe_wiring=0
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
bash tools/checks/rust_lifecycle_mirbuilder_programjson_loop_stmt_cond_recipe_sidecar_wiring_gate.sh
```

Expected summary:

```text
loop_stmt_cond_recipe_sidecar_wiring=1
recipe_item_attachment_implementation=1
summary=ok
```

## Next

```text
MIRBUILDER-RECIPEITEM-COND-RECIPE-OBSERVATION-BOUNDARY-SELECTION-001
```
