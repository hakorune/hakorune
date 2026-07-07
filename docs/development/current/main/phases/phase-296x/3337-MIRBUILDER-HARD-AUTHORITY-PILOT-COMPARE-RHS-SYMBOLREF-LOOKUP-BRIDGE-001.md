# 3337 - MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-SYMBOLREF-LOOKUP-BRIDGE-001

## Purpose
Connect the existing no-shadow SymbolRef lookup bridge to the current
hard-authority pilot chain after the read-only SymbolRef contract parity seam.

This card opens actual SymbolRef RHS `ValueId` lookup only for
contract-verified no-shadow local rows. It returns existing ValueIds and does
not allocate, mutate MIR, run LocalSSA, or emit Compare/Branch instructions.

## Input Evidence
- `MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-PARITY-001`
- `MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-BRIDGE-001`
- `rust-lifecycle-mirbuilder-compare-rhs-symbolref-lookup-bridge-v0`

## Positive Claims
- `hard_authority_pilot_implemented = 1`
- `compare_rhs_symbolref_lookup_bridge_owner = 1`
- `symbol_ref_valueid_resolution_no_shadow_local = 1`
- `symbol_lookup_execution = 1`
- `existing_valueid_returned = 1`
- `rhs_value_id_present = 1`
- `rhs_value_id_nonzero = 1`
- `contract_verified_symbol_lookup = 1`

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
- `new_backend_route = 0`
- `new_abi = 0`

## Selected Next
```text
MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-DESIGN-STOP-001
```

## Guard
```bash
bash tools/checks/rust_lifecycle_mirbuilder_hard_authority_pilot_compare_rhs_symbolref_lookup_bridge_guard.sh
```
