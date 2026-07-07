# 3312 - MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-CONSULTATION-001

## Purpose
Select the contract boundary before opening actual SymbolRef RHS `ValueId`
lookup.

## Decision
Select:

```text
B_SYMBOL_TABLE_CONTRACT_FIRST
```

C-style Rust oracle evidence may be folded into the 3313 guard as read-only
observation.

## Selected Next
```text
MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-PARITY-001
```

## Rejected For Now
- name-based lookup as public API / fallback authority
- separate runtime-adjacent oracle-shadow lane
- immediate actual SymbolRef lookup

## Required Contract
`SymbolRefResolutionContractV1` must include:

- `symbol_id`
- `source_name`
- `expected_rust_variable_key`
- `rust_variable_key_present`
- `rust_current_valueid_present`
- `rust_current_valueid_nonzero`
- `scope_contract_kind`
- `shadowing_claimed`
- `current_ssa_claimed`
- `local_ssa_materialization_claimed`
- `readonly`

## Explicit Non-Claims
- actual SymbolRef lookup: `0`
- SymbolRef `ValueId` resolution: `0`
- LocalSSA `finalize_compare`: `0`
- MIR Compare / Branch emission: `0`
- route selection / runtime route switch: `0`
- ProgramJSON runtime authority / runtime fallback: `0`
- Source Selfhost: `0`

## Guard
```bash
bash tools/checks/rust_lifecycle_mirbuilder_compare_rhs_symbolref_lookup_contract_consultation_guard.sh
```
