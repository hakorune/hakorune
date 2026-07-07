# 3276 - MIRBUILDER-PROGRAMJSON-IF-COND-RECIPE-EQ-BEHAVIOR-PRESERVING-001

Status: landed

## Purpose

Attach `cond_recipe` to existing ProgramJSON If `Var == Int` rows without
expanding accepted If syntax.

This is the first If producer wiring after the shared ProgramJSON compare
reader. It preserves legacy `cond_facts` and keeps existing `==` behavior.

## Implementation

Owner:

`lang/src/compiler/mirbuilder/stmt_handlers/if_stmt_handler.hako`

Changes:

- use `ProgramJsonCompareReaderBox.read_var_int_compare`
- build `BoolRecipeBox.from_numeric_compare_code_map`
- attach through `RecipeItemBox.if_item_with_cond_recipe` only when the recipe
  is valid
- keep legacy `cond_facts` as `VarEqInt`
- keep non-`==` If operators rejected

## Gate

`tools/checks/rust_lifecycle_mirbuilder_programjson_if_cond_recipe_eq_behavior_preserving_gate.sh`

The gate runs two AOT rows through `ProgramJsonV0PhaseStateBox.parse` and
checks:

- parse succeeds
- legacy `cond_facts` remains a map
- `cond_rhs_int` remains unchanged
- `cond_recipe` is present and summarizes as `Eq`

## Claims

- `if_cond_recipe_attached = 1`
- `if_eq_behavior_preserved = 1`
- `legacy_cond_facts_preserved = 1`
- `shared_compare_reader_used = 1`

## Non-Claims

- If operator expansion
- Loop nested If `cond_recipe`
- Rust loop condition Eq/Ne
- CondSkeleton::IfCond
- RecipeMatcher input authority
- BoolRecipe lowering
- MIR compare/branch emission
- route selection
- runtime route switch
- ProgramJSON runtime route authority
- runtime fallback
- Source Selfhost

## Next

`MIRBUILDER-PROGRAMJSON-IF-COND-RECIPE-FIRST-NON-EQ-ROW-001`
