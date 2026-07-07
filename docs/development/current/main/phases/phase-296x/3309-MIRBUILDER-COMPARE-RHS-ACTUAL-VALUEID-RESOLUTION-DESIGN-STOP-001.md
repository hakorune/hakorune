# 3309 - MIRBUILDER-COMPARE-RHS-ACTUAL-VALUEID-RESOLUTION-DESIGN-STOP-001

## Purpose
Select the first boundary before opening actual RHS `ValueId` resolution.

## Decision
Selected:

```text
MIRBUILDER-COMPARE-RHS-VALUEID-RESOLUTION-REQUEST-RESPONSE-ABI-001
```

The next card fixes request/response fields before any mutation/allocation
slice.

## Selected Sequence
1. Request/response ABI
2. LiteralI64 constant emission bridge
3. SymbolRef lookup contract consultation
4. SymbolRef lookup bridge
5. LocalSSA `finalize_compare` design-stop

## Rejected For Now
- LiteralI64 constant emission now
- SymbolRef lookup now
- Rust-authority shadow resolution guard now
- Moving to another Layer4 owner

## Explicit Non-Claims
- actual RHS `ValueId` resolution: `0`
- literal constant `ValueId` allocation: `0`
- constant/helper emission: `0`
- SymbolRef lookup execution: `0`
- LocalSSA `finalize_compare` execution: `0`
- MIR Compare emission: `0`
- MIR Branch emission: `0`
- BasicBlock mutation / `ValueId` allocation: `0`
- route selection / runtime route switch: `0`
- ProgramJSON runtime authority / runtime fallback: `0`
- Source Selfhost: `0`

## Guard
```bash
bash tools/checks/rust_lifecycle_mirbuilder_compare_rhs_actual_valueid_resolution_design_stop_guard.sh
```
