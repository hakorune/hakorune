# 296x-923 LOCAL-I64-MAP-ENTRY-TABLE-MATERIALIZATION-DESIGN-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-i64-map-entry-table-materialization-design-v0
source_evidence=296x-922
row_kind=design_decision
target_front=kilo_leaf_map_get_dynamic_covered_i64

selected_materialization=backend_local_const_i64_entry_table
runtime_helper_required=0
runtime_helper_abi_enabled=0
entry_table_owner=PythonBackendExactAOT
entry_table_source=LocalI64MapEntryValueTrackingRows
allowed_value_shape=i64_const_value_only
allowed_key_shape=i64_const_key_only
fallback_if_non_const_entry=generic_product_map_route
fallback_if_incomplete_coverage=generic_product_map_route
publication_materialization_policy=defer_to_product_mapbox_fallback

product_mapbox_storage_changed=0
product_hasher_swap=0
sidecar_storage=0
mirbuilder_map_storage_ownership=0
new_runtime_helper_enabled=0
backend_lowering_enabled=0
helper_emission_changed=0
winner_claim=0

next_task=LOCAL-I64-MAP-ENTRY-TABLE-MATERIALIZATION-GUARD-SURFACE-001
summary=ok
```

## Decision

The first executable entry-table shape should not add a runtime helper.

For v0, the Python exact-AOT backend may later materialize a local const i64
entry table from `LocalI64MapEntryValueTrackingRows` only when every used entry
has known i64 key and known i64 value constants.

The intended lowering shape is backend-local dispatch:

```text
if key == k0: return v0
if key == k1: return v1
...
else: fall back to product-compatible MapBox route
```

This row only selects the shape. It does not enable lowering.

## Rationale

This keeps the first executable slice small:

- no product `MapBox` storage change
- no runtime helper ABI
- no sidecar table in the runtime
- no MIRBuilder map storage ownership
- no publication materialization implementation

Non-const values, incomplete coverage, or publication-sensitive cases stay on
the generic product-compatible route.

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

`LOCAL-I64-MAP-ENTRY-TABLE-MATERIALIZATION-GUARD-SURFACE-001` should define the
post target and reject seams before code changes.
