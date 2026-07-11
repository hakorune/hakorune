# 3282 - MIRBUILDER-PROGRAMJSON-IF-LOOP-COMPARE-READER-PARITY-FLOOR-001

Status: landed

## Purpose

Prove that the shared ProgramJSON compare reader and the actual If/Loop
`RecipeItem.cond_recipe` producers agree for the covered condition rows.

This is a parity floor, not an operator-expansion card.

## Scope

Rows:

- top-level If: `i != 3`
- top-level Loop: `i < 5`
- Loop-body nested If: `i < 2`

Each row compares:

```text
ProgramJsonCompareReaderBox.read_var_int_compare
RecipeItemBox.cond_recipe_summary(parsed_recipe_item)
```

## Gate

`tools/checks/rust_lifecycle_mirbuilder_programjson_if_loop_compare_reader_parity_floor_gate.sh`

## Claims

- `if_loop_compare_reader_parity_floor = 1`
- `direct_reader_matches_recipe_cond_recipe = 1`
- `top_level_if_row = 1`
- `top_level_loop_row = 1`
- `loop_body_nested_if_row = 1`

## Non-Claims

- If accepts all six compare operators
- Loop nested If operator expansion
- RecipeMatcher input authority
- BoolRecipe lowering
- MIR compare/branch emission
- route selection
- runtime route switch
- ProgramJSON runtime route authority
- runtime fallback
- Source Selfhost

## Next

`MIRBUILDER-BOOL-RECIPE-COMPARE-LOWERING-BOUNDARY-CONSULTATION-001`
