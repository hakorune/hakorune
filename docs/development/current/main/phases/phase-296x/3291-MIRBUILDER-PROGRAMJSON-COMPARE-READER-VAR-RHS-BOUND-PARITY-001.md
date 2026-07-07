# 3291 - MIRBUILDER-PROGRAMJSON-COMPARE-READER-VAR-RHS-BOUND-PARITY-001

Status: landed

## Purpose

Extend the shared ProgramJSON Compare reader from LiteralI64 rhs bounds to
SymbolRef rhs bounds for the exact `Compare(lhs=Var, op, rhs=Var)` shape.

## Implemented Surface

`ProgramJsonCompareReaderBox.read_var_int_compare` now publishes:

```text
bound_kind_code=1, bound_i64=<value>, bound_symbol_id=0
```

for `rhs=Int`, and:

```text
bound_kind_code=2, bound_i64=0, bound_symbol_id=<symbol>
```

for `rhs=Var`.

The method name stays unchanged for existing producer compatibility. This
card proves direct reader and `BoolRecipeBox.from_numeric_compare_code_map`
parity only; it does not open If/Loop producer rows.

## Gate

`tools/checks/rust_lifecycle_mirbuilder_programjson_compare_reader_var_rhs_bound_parity_gate.sh`

The gate runs the 3290 follow-on selection guard, then executes six AOT/EXE
rows for:

```text
< <= > >= == !=
```

## Claims

- `compare_reader_var_rhs_bound_parity = 1`
- `var_rhs_bound_implemented = 1`
- `var_rhs_bound_symbol_ref = 1`
- `row_count = 6`

## Non-Claims

- If/Loop producer accepted rows
- length / length-minus bound support
- reversed Var/Var context-aware inference
- BoolRecipe lowering execution
- MIR Compare emission
- Branch emission
- BasicBlock mutation
- ValueId allocation
- route selection
- runtime route switch
- ProgramJSON runtime route authority
- runtime fallback
- Source Selfhost

## Next

`MIRBUILDER-PROGRAMJSON-COMPARE-READER-VAR-RHS-PRODUCER-ROW-SELECTION-001`
