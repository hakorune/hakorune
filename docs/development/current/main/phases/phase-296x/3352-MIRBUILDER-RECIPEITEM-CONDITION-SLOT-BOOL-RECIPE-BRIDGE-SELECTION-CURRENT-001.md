# 3352 - MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-BRIDGE-SELECTION-CURRENT-001

## Token

```text
MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-BRIDGE-SELECTION-CURRENT-001
```

## Purpose

Consume the existing RecipeItem condition-slot BoolRecipe bridge selection in
the current 335x chain.

The selected bridge remains the existing optional sidecar:

```text
OptionalCondRecipeSidecar
```

## Result

```text
condition_slot_bridge_selection = 1
selected_optional_cond_recipe_sidecar = 1
selected_bridge = OptionalCondRecipeSidecar
legacy_cond_facts_required = 1
cond_recipe_optional = 1
current_wrapper = 1
source_selfhost_claim = 0
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_recipeitem_condition_slot_bool_recipe_bridge_selection_current_guard.sh
```

## Selected Next

```text
selected_next_card:
  MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-SIDECAR-BRIDGE-CURRENT-GATE-001
```

## Non-Claims

```text
recipe_item_attachment = 0
recipe_matcher_input_authority = 0
bool_recipe_lowering = 0
mir_cmp_emission = 0
branch_emission = 0
route_selection = 0
runtime_route_switch = 0
source_selfhost_claim = 0
```
