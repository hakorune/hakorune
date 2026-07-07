# 3286 - MIRBUILDER-PROGRAMJSON-IF-COND-RECIPE-RELATIONAL-ROW-BATCH-001

Status: landed

## Purpose

Open top-level If relational `Var op Int` rows through the shared ProgramJSON
compare reader and `RecipeItem.cond_recipe`.

## Implementation

Updated:

```text
lang/src/compiler/mirbuilder/stmt_handlers/if_stmt_handler.hako
```

The handler now accepts these top-level If compare operators:

```text
< <= > >= == !=
```

The new relational rows preserve legacy `cond_facts` with:

```text
VarLtInt / VarLeInt / VarGtInt / VarGeInt
```

and attach a `BoolRecipe::Compare` sidecar through the shared reader.

## Gate

`tools/checks/rust_lifecycle_mirbuilder_programjson_if_cond_recipe_relational_row_batch_gate.sh`

Rows:

- `if_var_lt_int_then_return_else_null`
- `if_var_le_int_then_return_else_null`
- `if_var_gt_int_then_return_else_null`
- `if_var_ge_int_then_return_else_null`

## Claims

- `if_cond_recipe_relational_row_batch = 1`
- `if_relational_rows = 4`
- `shared_compare_reader_used = 1`
- `legacy_cond_facts_relational = 1`
- `if_accepts_all_6_compare_operators = 1`

## Non-Claims

- Loop nested If operator expansion
- top-level Loop route semantics changed
- RecipeMatcher input authority
- BoolRecipe lowering
- MIR Compare emission
- Branch emission
- route selection
- runtime route switch
- ProgramJSON runtime route authority
- runtime fallback
- Source Selfhost

## Next

`MIRBUILDER-PROGRAMJSON-LOOP-NESTED-IF-COND-RECIPE-RELATIONAL-ROW-BATCH-001`
