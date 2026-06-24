# 296x-915 LOCAL-I64-MAP-DIRECT-STORAGE-PLAN-SURFACE-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-i64-map-direct-storage-plan-surface-v0
source_evidence=296x-914
row_kind=passive_metadata_surface
target_front=kilo_leaf_map_get_dynamic_covered_i64

metadata_surface=FunctionMetadata.local_i64_map_direct_storage_plans
mir_json_surface=metadata.local_i64_map_direct_storage_plans
selected_representation=closed_world_i64_key_value_table
known_i64_key_set_count_exported=1
scalar_get_count_exported=1
entry_value_tracking_enabled=0
publication_materialization_required=1

product_mapbox_storage_changed=0
product_hasher_swap=0
sidecar_storage=0
mirbuilder_map_storage_ownership=0
new_runtime_helper_enabled=0
backend_loader_enabled=0
backend_lowering_enabled=0
winner_claim=0

next_task=LOCAL-I64-MAP-DIRECT-STORAGE-BACKEND-LOADER-001
summary=ok
```

## Decision

This row adds a passive direct storage plan descriptor for unpublished local i64
maps. The descriptor names the first selected representation,
`closed_world_i64_key_value_table`, and exports only the shape counts needed for
the next reader row.

The descriptor is deliberately not a lowering permission:

```text
entry_value_tracking_enabled=0
backend_loader_enabled=0
backend_lowering_enabled=0
runtime_helper_enabled=0
```

## Surface

The row adds:

```text
FunctionMetadata.local_i64_map_direct_storage_plans
metadata.local_i64_map_direct_storage_plans
```

Each row records:

```text
receiver_value
representation=closed_world_i64_key_value_table
known_i64_key_set_count
scalar_get_count
entry_value_tracking_enabled=0
publication_materialization_required=1
backend_lowering_enabled=0
runtime_helper_enabled=0
```

## Stop Lines

- no product `MapBox` storage change
- no product hasher swap
- no sidecar storage
- no MIRBuilder map storage ownership
- no new runtime helper
- no backend loader in this row
- no backend lowering in this row
- no entry value table materialization in this row
- no helper-name / benchmark-name inference
- no winner claim

## Next

`LOCAL-I64-MAP-DIRECT-STORAGE-BACKEND-LOADER-001` may teach the Python backend
to read `local_i64_map_direct_storage_plans` as metadata only. It must still keep
lowering disabled until a later guarded consumer row.
