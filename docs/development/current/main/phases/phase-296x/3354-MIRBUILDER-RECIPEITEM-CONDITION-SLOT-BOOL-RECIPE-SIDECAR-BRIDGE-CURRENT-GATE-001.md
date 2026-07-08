# 3354 - MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-SIDECAR-BRIDGE-CURRENT-GATE-001

## Token

```text
MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-SIDECAR-BRIDGE-CURRENT-GATE-001
```

## Purpose

Consume the existing RecipeItem condition-slot BoolRecipe sidecar bridge gate
in the current 335x chain.

This current wrapper proves the landed `RecipeItemBox` optional `cond_recipe`
sidecar remains green while keeping the legacy `cond_facts` input required.

## Result

```text
recipeitem_cond_recipe_sidecar_bridge_current_gate = 1
recipeitem_cond_recipe_sidecar_bridge = 1
optional_cond_recipe_sidecar = 1
legacy_cond_facts_required = 1
recipe_item_attachment = 1
current_wrapper = 1
source_selfhost_claim = 0
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_recipeitem_condition_slot_bool_recipe_sidecar_bridge_current_gate.sh
```

## Selected Next

```text
selected_next_card:
  MIRBUILDER-PROGRAMJSON-RECIPEITEM-COND-RECIPE-PRODUCER-WIRING-SELECTION-001
```

## Non-Claims

```text
verifier_behavior_change = 0
lowering_behavior_change = 0
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
