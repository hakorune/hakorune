# 3336 - MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-PARITY-001

## Purpose
Connect the consultation-selected SymbolRef contract parity seam to the current
hard-authority pilot chain after the LiteralI64 const-only bridge.

This card consumes the existing 3313 read-only parity gate. It does not perform
actual SymbolRef lookup and does not return a ValueId to the lowering path.

## Input Evidence
- `MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-LITERAL-I64-CONSTANT-EMISSION-BRIDGE-001`
- `MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-CONSULTATION-001`
- `MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-PARITY-001`
- `rust-lifecycle-mirbuilder-compare-rhs-symbolref-lookup-contract-parity-v0`

## Positive Claims
- `hard_authority_pilot_implemented = 1`
- `compare_rhs_symbolref_lookup_contract_parity_owner = 1`
- `symbol_ref_resolution_contract_v1 = 1`
- `symbol_id_to_source_name_mapping = 1`
- `source_name_to_expected_rust_variable_key_mapping = 1`
- `rust_variable_key_readonly_observed = 1`
- `rust_current_valueid_readonly_observed = 1`
- `contract_verified_rows = 2`

## Explicit Non-Claims
- `symbol_lookup_execution = 0`
- `symbol_ref_valueid_resolution = 0`
- `existing_valueid_returned_as_bridge_response = 0`
- `valueid_allocated = 0`
- `constant_mir_emission = 0`
- `local_ssa_finalize_compare = 0`
- `mir_compare_emission = 0`
- `mir_branch_emission = 0`
- `basicblock_mutation = 0`
- `route_selection = 0`
- `runtime_route_switch = 0`
- `programjson_runtime_authority = 0`
- `runtime_fallback = 0`
- `source_selfhost_claim = 0`
- `new_backend_route = 0`
- `new_abi = 0`

## Selected Next
```text
MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-BRIDGE-001
```

## Guard
```bash
bash tools/checks/rust_lifecycle_mirbuilder_hard_authority_pilot_compare_rhs_symbolref_lookup_contract_parity_guard.sh
```
