# 3316 - MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-BRIDGE-001

## Purpose
Open only the LocalSSA compare-operand finalization step after lhs/rhs
`ValueId` resolution is available.

## Boundary
This card calls the existing Rust owner:

```text
ssa::local::finalize_compare(lhs, rhs)
```

through a narrow bridge. The bridge may materialize operands according to
LocalSSA policy, but it does not emit MIR Compare, emit Branch, publish a Bool
result type, select a route, or switch runtime authority.

## Positive Claims
- `compare_localssa_finalize_compare_bridge = 1`
- `localssa_finalize_compare_execution = 1`
- `lhs_rhs_valueids_finalized = 1`

## Explicit Non-Claims
- MIR Compare emission: `0`
- MIR Branch emission: `0`
- Bool result type publication: `0`
- route selection / runtime route switch: `0`
- ProgramJSON runtime authority / runtime fallback: `0`
- Source Selfhost: `0`

## Guard
```bash
bash tools/checks/rust_lifecycle_mirbuilder_compare_localssa_finalize_compare_bridge_gate.sh
```
