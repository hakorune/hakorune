# 3315 - MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-DESIGN-STOP-001

## Purpose
Stop before mixing SymbolRef lookup with LocalSSA operand materialization.

## Boundary
Previous cards may produce RHS `ValueId`s:

- LiteralI64: newly emitted integer `Const`
- SymbolRef: existing Rust current `ValueId` for no-shadow local rows

This card decides how to expose:

```text
LocalSSA finalize_compare(lhs, rhs)
```

without opening MIR Compare / Branch emission in the same slice.

## Explicit Non-Claims
- LocalSSA bridge implementation: `0`
- MIR Compare emission: `0`
- MIR Branch emission: `0`
- route selection / runtime route switch: `0`
- ProgramJSON runtime authority / runtime fallback: `0`
- Source Selfhost: `0`

## Decision
Selected:

```text
MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-BRIDGE-001
```

The selected next card may call Rust `ssa::local::finalize_compare` as a
scoped bridge, but must not emit MIR Compare / Branch or publish the Bool result
type.

## Guard
```bash
bash tools/checks/rust_lifecycle_mirbuilder_compare_localssa_finalize_compare_design_stop_guard.sh
```
