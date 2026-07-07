# 3313 - MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-PARITY-001

## Purpose
Prove the read-only SymbolRef resolution contract before any actual lookup.

## Rows
- `simple_local_i`
- `renamed_local_count`
- `shadowed_name_not_claimed`

## Positive Claims
- `symbol_ref_resolution_contract_v1 = 1`
- `symbol_id_to_source_name_mapping = 1`
- `source_name_to_expected_rust_variable_key_mapping = 1`
- `rust_variable_key_readonly_observed = 1`
- `rust_current_valueid_readonly_observed = 1`
- `contract_verified_rows = 2`

Optional read-only oracle evidence:

- `rust_oracle_current_valueid_observed = 1`
- `programjson_symbol_contract_matches_rust_observation = 1`

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
- `source_selfhost_claim = 0`

## Selected Next
```text
MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-BRIDGE-001
```

## Guard
```bash
bash tools/checks/rust_lifecycle_mirbuilder_compare_rhs_symbolref_lookup_contract_parity_gate.sh
```
