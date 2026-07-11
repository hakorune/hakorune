# 3264 - MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-BRIDGE-SELECTION-001

Status: landed

## Scope

Select the first bridge from read-only `BoolRecipeCompareV1` publication into
the `RecipeItem` condition slot.

Selected bridge:

```text
OptionalCondRecipeSidecar
```

The selected bridge keeps existing `cond_facts` required and adds optional
`cond_recipe` only in the implementation card.

## Reason

`cond_facts` is still consumed by:

```text
RecipeVerifierBox
MirJsonV0ShapeBoxRecipeControl
existing RecipeItem DTO snapshots
```

Replacing `cond_facts` now would mix the bridge with verifier and lowering
migration. The first safe bridge is therefore a sidecar.

## Fixture / Gate

Fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-recipeitem-condition-slot-bool-recipe-bridge-selection-v0.json
```

Gate:

```text
tools/checks/rust_lifecycle_mirbuilder_recipeitem_condition_slot_bool_recipe_bridge_selection_guard.sh
```

## Claims

```text
condition_slot_bridge_selection=1
selected_optional_cond_recipe_sidecar=1
```

## Non-Claims

```text
recipe_item_attachment=0
recipe_matcher_input_authority=0
bool_recipe_lowering=0
mir_cmp_emission=0
branch_emission=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
source_selfhost_claim=0
```

## Verification

```bash
bash tools/checks/rust_lifecycle_mirbuilder_recipeitem_condition_slot_bool_recipe_bridge_selection_guard.sh
```

Expected summary:

```text
selected_bridge=OptionalCondRecipeSidecar
selected_next_card=MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-SIDECAR-BRIDGE-001
summary=ok
```

## Next

```text
MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-SIDECAR-BRIDGE-001
```
