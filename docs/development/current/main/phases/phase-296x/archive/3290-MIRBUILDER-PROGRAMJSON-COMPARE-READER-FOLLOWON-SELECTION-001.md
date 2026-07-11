# 3290 - MIRBUILDER-PROGRAMJSON-COMPARE-READER-FOLLOWON-SELECTION-001

Status: landed

## Purpose

Select the next ProgramJSON Compare reader axis after closing the If/Loop
`Var op Int` relational row batch.

## Decision

Select:

```text
MIRBUILDER-PROGRAMJSON-COMPARE-READER-VAR-RHS-BOUND-PARITY-001
```

The next implementation should extend the shared reader from:

```text
Compare(lhs=Var, op=< <= > >= == !=, rhs=Int)
```

to the next analysis-only bound shape:

```text
Compare(lhs=Var, op=< <= > >= == !=, rhs=Var)
```

The reader output stays `ProgramJsonCompareReaderCodeMapV1`, with
`bound_kind_code=2` and `bound_symbol_id>0`.

## Rationale

`Var op Var` covers common user syntax such as `i < n` without needing
context-sensitive loop-variable inference. It also maps directly to the
existing `BoundExprBox.symbol_ref` / BoolRecipe `SymbolRef` vocabulary.

Rejected for now:

- length / length-minus bounds, because they require method/member-shape
  parsing beyond the current reader.
- reversed variable-variable inference such as `n >= i`, because that needs
  update-target or loop-variable context.
- mutation-bearing lowering owner selection, because MIR Compare/Branch
  emission still needs operand, rhs materialization, allocation, and block
  ownership boundaries.

## Gate

`tools/checks/rust_lifecycle_mirbuilder_programjson_compare_reader_followon_selection_guard.sh`

## Claims

- `compare_reader_followon_selection = 1`
- `var_rhs_bound_selected = 1`

## Non-Claims

- `Var op Var` implementation
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

`MIRBUILDER-PROGRAMJSON-COMPARE-READER-VAR-RHS-BOUND-PARITY-001`
