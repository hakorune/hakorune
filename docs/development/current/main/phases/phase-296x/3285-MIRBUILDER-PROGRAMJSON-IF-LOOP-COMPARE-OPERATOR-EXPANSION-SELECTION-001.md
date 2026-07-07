# 3285 - MIRBUILDER-PROGRAMJSON-IF-LOOP-COMPARE-OPERATOR-EXPANSION-SELECTION-001

Status: landed

## Purpose

Choose the next user-visible Compare operator expansion after shared reader
parity and BoolRecipe lowering intent are green.

## Decision

Select top-level If relational rows first:

```text
MIRBUILDER-PROGRAMJSON-IF-COND-RECIPE-RELATIONAL-ROW-BATCH-001
```

Rows:

- `if_var_lt_int_then_return_else_null`
- `if_var_le_int_then_return_else_null`
- `if_var_gt_int_then_return_else_null`
- `if_var_ge_int_then_return_else_null`

## Rationale

`ProgramJsonCompareReaderBox` already reads all six `Var op Int` operators.
`IfStmtHandler` still accepts only `==` and `!=`, so relational If is the next
clear user-visible gap.

Loop nested If and top-level Loop route rows remain separate because they touch
loop-body exit facts and route/release semantics.

## Gate

`tools/checks/rust_lifecycle_mirbuilder_programjson_if_loop_compare_operator_expansion_selection_guard.sh`

## Claims

- `operator_expansion_selection = 1`
- `top_level_if_relational_batch_selected = 1`

## Non-Claims

- If accepts all six compare operators
- Loop nested If operator expansion
- top-level Loop route semantics changed
- BoolRecipe lowering executed
- MIR Compare emission
- Branch emission
- route selection
- runtime route switch
- ProgramJSON runtime route authority
- runtime fallback
- Source Selfhost

## Next

`MIRBUILDER-PROGRAMJSON-IF-COND-RECIPE-RELATIONAL-ROW-BATCH-001`
