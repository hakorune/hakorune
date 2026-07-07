# 3277 - MIRBUILDER-PROGRAMJSON-IF-COND-RECIPE-FIRST-NON-EQ-ROW-001

Status: landed

## Purpose

Open the first user-visible non-`==` If condition row through the shared
ProgramJSON compare reader.

This card admits only:

```text
If Compare(Var != Int)
```

It does not claim all comparison operators.

## Implementation

Owner:

`lang/src/compiler/mirbuilder/stmt_handlers/if_stmt_handler.hako`

Changes:

- allow `!=` beside existing `==`
- keep `ProgramJsonCompareReaderBox.read_var_int_compare` as the reader
- build `BoolRecipeBox.from_numeric_compare_code_map`
- publish `VarNeInt` in legacy `cond_facts` for this row
- keep `<`, `<=`, `>`, `>=` rejected for If producer paths

## Gate

`tools/checks/rust_lifecycle_mirbuilder_programjson_if_cond_recipe_first_non_eq_row_gate.sh`

The gate runs one AOT row through `ProgramJsonV0PhaseStateBox.parse` and
checks:

- parse succeeds
- `cond_facts.cond_kind` is `VarNeInt`
- `cond_recipe` is present
- `BoolRecipe` summary reports `cmp=Ne`

## Claims

- `if_cond_recipe_ne_row = 1`
- `if_first_non_eq_row = 1`
- `shared_compare_reader_used = 1`
- `legacy_cond_facts_var_ne_int = 1`

## Non-Claims

- If accepts all six compare operators
- If `<`, `<=`, `>`, `>=` rows
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

`MIRBUILDER-PROGRAMJSON-LOOP-NESTED-IF-COND-RECIPE-BRIDGE-001`
