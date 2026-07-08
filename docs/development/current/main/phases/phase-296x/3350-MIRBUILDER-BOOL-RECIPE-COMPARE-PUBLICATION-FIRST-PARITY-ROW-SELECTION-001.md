# 3350 - MIRBUILDER-BOOL-RECIPE-COMPARE-PUBLICATION-FIRST-PARITY-ROW-SELECTION-001

## Token

```text
MIRBUILDER-BOOL-RECIPE-COMPARE-PUBLICATION-FIRST-PARITY-ROW-SELECTION-001
```

## Purpose

Select the first scoped BoolRecipe compare publication parity row after the
MapStoreI64 `.hako` fast-path shadow-consume gate is green.

This is a selection-only card. It does not execute the publication parity gate.

## Selected Row

```text
selected_row_id = var_le_literal
source_program_row = local_loop_body_if_branch_return
output_contract = BoolRecipeComparePublicationV1
```

## Result

```text
bool_recipe_compare_publication_first_parity_row_selected = 1
selected_row_id = var_le_literal
selection_only = 1
source_selfhost_claim = 0
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_bool_recipe_compare_publication_first_parity_row_selection_guard.sh
```

## Selected Next

```text
selected_next_card:
  MIRBUILDER-BOOL-RECIPE-COMPARE-PUBLICATION-FIRST-PARITY-ROW-GATE-001
```

## Non-Claims

```text
parity_executed = 0
recipe_item_attachment = 0
recipe_matcher_input_authority = 0
bool_recipe_lowering = 0
mir_cmp_emission = 0
branch_emission = 0
route_selection = 0
runtime_route_switch = 0
source_selfhost_claim = 0
```
