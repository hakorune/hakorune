# 3314 - MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-BRIDGE-001

## Purpose
Open actual SymbolRef RHS `ValueId` lookup for contract-verified no-shadow
local rows only.

## Scope
The bridge consumes `SymbolRefResolutionContractV1` rows that are already green
in 3313.

Allowed rows:

- `simple_local_i`
- `renamed_local_count`
- `unmapped_symbol_id_rejects`

## Positive Claims
- `symbol_ref_valueid_resolution_no_shadow_local = 1`
- `symbol_lookup_execution = 1`
- `existing_valueid_returned = 1`
- `rhs_value_id_present = 1`
- `rhs_value_id_nonzero = 1`
- `contract_verified_symbol_lookup = 1`
- `simple_local_i_lookup = 1`
- `renamed_local_count_lookup = 1`

## Explicit Non-Claims
- `symbol_ref_valueid_resolution_general = 0`
- `shadowing_symbol_lookup = 0`
- `current_ssa_authority = 0`
- `local_ssa_finalize_compare = 0`
- `valueid_allocated = 0`
- `literal_constant_valueid_allocation = 0`
- `constant_mir_emission = 0`
- `mir_compare_emission = 0`
- `mir_branch_emission = 0`
- `basicblock_mutation = 0`
- `route_selection = 0`
- `runtime_route_switch = 0`
- `programjson_runtime_authority = 0`
- `runtime_fallback = 0`
- `source_selfhost_claim = 0`

## Selected Next
```text
MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-DESIGN-STOP-001
```

## Guard
```bash
bash tools/checks/rust_lifecycle_mirbuilder_compare_rhs_symbolref_lookup_bridge_gate.sh
```
