# 3265 - MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-SIDECAR-BRIDGE-001

Status: landed

## Scope

Add the selected optional `cond_recipe` sidecar to `RecipeItemBox` while
keeping existing `cond_facts` required.

The sidecar is data-only:

```text
RecipeItem.If / RecipeItem.Loop
  cond_facts  = legacy required verifier/lowering input
  cond_recipe = optional BoolRecipeCompareV1 sidecar
```

## Implementation

Owner:

```text
lang/src/compiler/mirbuilder/recipe/recipe_item_box.hako
```

Added API:

```text
if_item_with_cond_recipe(cond_facts, cond_recipe, then_item, else_item)
loop_item_with_cond_recipe(cond_facts, cond_recipe, body_item)
cond_recipe_present(item)
cond_recipe_summary(item)
```

The legacy `if_item` and `loop_item` constructors are unchanged.

## Fixture / Gate

Fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-recipeitem-condition-slot-bool-recipe-sidecar-bridge-v0.json
```

Gate:

```text
tools/checks/rust_lifecycle_mirbuilder_recipeitem_condition_slot_bool_recipe_sidecar_bridge_gate.sh
```

## Claims

```text
recipeitem_cond_recipe_sidecar_bridge=1
optional_cond_recipe_sidecar=1
legacy_cond_facts_required=1
recipe_item_attachment=1
```

## Non-Claims

```text
verifier_behavior_change=0
lowering_behavior_change=0
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
bash tools/checks/rust_lifecycle_mirbuilder_recipeitem_condition_slot_bool_recipe_sidecar_bridge_gate.sh
```

Expected summary:

```text
recipeitem_cond_recipe_sidecar_bridge=1
optional_cond_recipe_sidecar=1
legacy_cond_facts_required=1
summary=ok
```

## Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEITEM-COND-RECIPE-PRODUCER-WIRING-SELECTION-001
```
