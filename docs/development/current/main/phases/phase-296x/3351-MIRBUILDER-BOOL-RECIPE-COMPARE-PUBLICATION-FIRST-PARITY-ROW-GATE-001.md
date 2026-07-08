# 3351 - MIRBUILDER-BOOL-RECIPE-COMPARE-PUBLICATION-FIRST-PARITY-ROW-GATE-001

## Token

```text
MIRBUILDER-BOOL-RECIPE-COMPARE-PUBLICATION-FIRST-PARITY-ROW-GATE-001
```

## Purpose

Run the scoped first BoolRecipe compare publication parity row selected by
3350.

The underlying fixture is already first-row scoped:

```text
selected_row_id = var_le_literal
publication_rows = 1
```

## Result

```text
bool_recipe_compare_publication_first_parity_row_gate = 1
publication_rows = 1
selected_row_id = var_le_literal
read_only_bool_recipe_compare_publication = 1
analysis_only = 1
source_selfhost_claim = 0
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_bool_recipe_compare_publication_first_parity_row_gate.sh
```

## Selected Next

```text
selected_next_card:
  MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-BRIDGE-SELECTION-001
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
