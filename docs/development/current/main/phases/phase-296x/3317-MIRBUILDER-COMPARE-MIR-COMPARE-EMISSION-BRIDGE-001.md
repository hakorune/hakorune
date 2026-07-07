# 3317 - MIRBUILDER-COMPARE-MIR-COMPARE-EMISSION-BRIDGE-001

## Purpose
Open only MIR Compare emission after lhs/rhs ValueIds have already been
resolved and LocalSSA-finalized.

## Boundary
This card calls the existing Compare emission SSOT:

```text
emission::compare::emit_to(dst, op, lhs, rhs)
```

through a narrow bridge. The bridge allocates the Compare result `ValueId` and
publishes its Bool type, matching the existing Rust owner. It does not emit
Branch, consume the Compare result as a branch condition, select a route, or
switch runtime authority.

## Positive Claims
- `compare_mir_compare_emission_bridge = 1`
- `mir_compare_emission = 1`
- `compare_result_valueid_allocated = 1`
- `bool_result_type_publication = 1`

## Explicit Non-Claims
- MIR Branch emission: `0`
- Branch condition consumption: `0`
- route selection / runtime route switch: `0`
- ProgramJSON runtime authority / runtime fallback: `0`
- Source Selfhost: `0`

## Guard
```bash
bash tools/checks/rust_lifecycle_mirbuilder_compare_mir_compare_emission_bridge_gate.sh
```
