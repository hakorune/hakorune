# 296x-924 LOCAL-I64-MAP-ENTRY-TABLE-MATERIALIZATION-GUARD-SURFACE-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-i64-map-entry-table-materialization-guard-surface-v0
source_evidence=296x-923
row_kind=guard_surface
target_front=kilo_leaf_map_get_dynamic_covered_i64

post_target=backend_local_const_i64_entry_dispatch
allowed_emit_shape=icmp_chain_or_switch_over_key
allowed_entry_source=EntryValueTrackingRows
allowed_entry_value_shape=i64_const_value_only
allowed_entry_key_shape=i64_const_key_only
allowed_fallback=current_product_compatible_map_route
required_negative_guard_non_const_value=1
required_negative_guard_missing_entry_rows=1
required_negative_guard_missing_fastpath_fact=1

runtime_helper_import_required=0
new_runtime_helper_enabled=0
product_mapbox_storage_changed=0
product_hasher_swap=0
sidecar_storage=0
mirbuilder_map_storage_ownership=0
backend_lowering_enabled=0
helper_emission_changed=0
winner_claim=0

next_task=LOCAL-I64-MAP-ENTRY-TABLE-MATERIALIZATION-PILOT-001
summary=ok
```

## Guard Surface

The next implementation row may add an exact-AOT backend-local dispatch for a
map get callsite only when all required inputs are present:

```text
LocalFastPathFact
LocalI64MapDirectStoragePlan
EntryValueTrackingRows
known i64 const keys
known i64 const values
```

The allowed code shape is an in-function key dispatch:

```text
key == k0 -> v0
key == k1 -> v1
...
fallback -> current product-compatible map route
```

This row itself does not enable the lowering.

## Required Negative Cases

The implementation row must keep the current product-compatible route when:

- entry rows are missing
- the fastpath fact is missing
- the direct storage plan is missing
- any entry value is not a known i64 constant
- any entry key is not a known i64 constant

## Stop Lines

- no product `MapBox` storage change
- no product hasher swap
- no sidecar storage
- no MIRBuilder map storage ownership
- no new runtime helper
- no backend lowering in this row
- no helper emission change in this row
- no publication materialization implementation
- no helper-name / benchmark-name inference
- no winner claim

## Next

`LOCAL-I64-MAP-ENTRY-TABLE-MATERIALIZATION-PILOT-001` may implement the guarded
backend-local const dispatch. It must include the negative guards above.
