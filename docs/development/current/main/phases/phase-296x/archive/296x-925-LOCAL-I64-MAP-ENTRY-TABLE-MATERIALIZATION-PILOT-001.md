# 296x-925 LOCAL-I64-MAP-ENTRY-TABLE-MATERIALIZATION-PILOT-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-i64-map-entry-table-materialization-pilot-v0
source_evidence=296x-924
row_kind=implementation_pilot
target_front=kilo_leaf_map_get_dynamic_covered_i64

implemented_shape=backend_local_const_i64_entry_dispatch
emit_shape=branch_chain_with_phi
entry_source=EntryValueTrackingRows
requires_local_fastpath_fact=1
requires_direct_storage_plan=1
requires_known_i64_const_keys=1
requires_known_i64_const_values=1
fallback_route=nyash.map.slot_load_hh
negative_guard_non_const_value=1
negative_guard_missing_entry_rows=1

runtime_helper_import_required=0
new_runtime_helper_enabled=0
product_mapbox_storage_changed=0
product_hasher_swap=0
sidecar_storage=0
mirbuilder_map_storage_ownership=0
winner_claim=0

next_task=LOCAL-I64-MAP-ENTRY-TABLE-MATERIALIZATION-VALIDATION-001
summary=ok
```

## Implementation

The Python backend now lowers eligible local i64 map get callsites to an
in-function const-key dispatch:

```text
if key == k0 -> v0
if key == k1 -> v1
...
fallback -> nyash.map.slot_load_hh
```

This is only enabled when the callsite has:

- `LocalFastPathFact`
- `LocalI64MapDirectStoragePlan`
- entry value tracking rows
- known i64 const keys
- known i64 const values

No runtime helper is introduced.

## Stop Lines

- no product `MapBox` storage change
- no product hasher swap
- no sidecar storage
- no MIRBuilder map storage ownership
- no new runtime helper
- no publication materialization implementation
- no helper-name / benchmark-name inference
- no winner claim

## Next

`LOCAL-I64-MAP-ENTRY-TABLE-MATERIALIZATION-VALIDATION-001` should validate the
unit-level dispatch shape, current guards, and target-front AOT reachability
before any performance claim.
