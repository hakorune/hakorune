# 3311 - MIRBUILDER-COMPARE-RHS-LITERAL-I64-CONSTANT-EMISSION-BRIDGE-001

## Purpose
Open the first scoped RHS `ValueId` resolution bridge for `LiteralI64` only.

This is the first mutation slice after the read-only RHS resolution plan and
request/response ABI. The bridge emits one integer `Const` instruction and
returns a `CompareRhsValueIdResolutionResponseV1`.

## Implementation
Added:

```text
src/mir/builder/compare_rhs_valueid_resolution_bridge.rs
```

The owner is:

```text
CompareRhsConstantEmissionBridge
```

It delegates integer constant emission to the existing emission SSOT:

```text
emission::constant::emit_integer
```

## Row
- `literal_i64_3`

## Positive Claims
- actual RHS `ValueId` resolution for `LiteralI64`: `1`
- literal constant `ValueId` allocation: `1`
- integer `Const` MIR emission: `1`
- integer type publication: `1`
- mutation performed is const-only: `1`

## Explicit Non-Claims
- general RHS `ValueId` resolution: `0`
- SymbolRef `ValueId` resolution / symbol lookup: `0`
- LocalSSA `finalize_compare`: `0`
- MIR Compare emission: `0`
- MIR Branch emission: `0`
- BasicBlock control-flow mutation: `0`
- route selection / runtime route switch: `0`
- ProgramJSON runtime authority / runtime fallback: `0`
- Source Selfhost: `0`

## Guard
```bash
bash tools/checks/rust_lifecycle_mirbuilder_compare_rhs_literal_i64_constant_emission_bridge_gate.sh
```
