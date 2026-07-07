# 3287 - MIRBUILDER-PROGRAMJSON-LOOP-NESTED-IF-COND-RECIPE-RELATIONAL-ROW-BATCH-001

Status: landed

## Purpose

Open Loop-body nested If relational `Var op Int` rows through the shared
ProgramJSON compare reader and `RecipeItem.cond_recipe`.

## Implementation

Updated:

```text
lang/src/compiler/mirbuilder/stmt_handlers/loop_stmt_handler.hako
lang/src/compiler/mirbuilder/stmt_handlers/loop_nested_if_cond_recipe_bridge_box.hako
```

The Loop body nested If producer now accepts these Compare operators for the
covered `If(... then Return) + Assignment` body shape:

```text
< <= > >=
```

Legacy `cond_facts` stay present and are normalized by the bridge from the
shared reader output:

```text
VarLtInt / VarLeInt / VarGtInt / VarGeInt
```

The public boundary remains a data-only `BoolRecipe::Compare` sidecar. The gate
uses local value staging before summary output so the AOT proof does not depend
on long dynamic string-concat chains.

## Gate

`tools/checks/rust_lifecycle_mirbuilder_programjson_loop_nested_if_cond_recipe_relational_row_batch_gate.sh`

Rows:

- `loop_body_if_var_lt_int_then_return_assignment`
- `loop_body_if_var_le_int_then_return_assignment`
- `loop_body_if_var_gt_int_then_return_assignment`
- `loop_body_if_var_ge_int_then_return_assignment`

## Claims

- `loop_nested_if_cond_recipe_relational_row_batch = 1`
- `loop_nested_if_relational_rows = 4`
- `shared_compare_reader_used = 1`
- `legacy_cond_facts_relational = 1`

## Non-Claims

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

`MIRBUILDER-BOOL-RECIPE-COMPARE-LOWERING-EMISSION-CONSULTATION-001`
