# 296x-916 LOCAL-I64-MAP-DIRECT-STORAGE-BACKEND-LOADER-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-i64-map-direct-storage-backend-loader-v0
source_evidence=296x-915
row_kind=metadata_loader
target_front=kilo_leaf_map_get_dynamic_covered_i64

metadata_surface=metadata.local_i64_map_direct_storage_plans
backend_loader=local_i64_map_direct_storage_plans_by_receiver
loader_key=receiver_value
selected_representation=closed_world_i64_key_value_table
known_i64_key_set_count_loaded=1
scalar_get_count_loaded=1
entry_value_tracking_enabled_loaded=1
publication_materialization_required_loaded=1

product_mapbox_storage_changed=0
product_hasher_swap=0
sidecar_storage=0
mirbuilder_map_storage_ownership=0
new_runtime_helper_enabled=0
backend_lowering_enabled=0
backend_consumer_enabled=0
winner_claim=0

next_task=LOCAL-I64-MAP-DIRECT-STORAGE-SHADOW-001
summary=ok
```

## Decision

The Python exact-AOT backend now reads
`metadata.local_i64_map_direct_storage_plans` into a receiver-keyed resolver
table:

```text
resolver.local_i64_map_direct_storage_plans_by_receiver
```

This is a loader-only row. It makes the metadata visible to later backend rows
but does not authorize direct storage lowering or helper emission.

## Stop Lines

- no product `MapBox` storage change
- no product hasher swap
- no sidecar storage
- no MIRBuilder map storage ownership
- no new runtime helper
- no backend consumer in this row
- no backend lowering in this row
- no entry value table materialization in this row
- no helper-name / benchmark-name inference
- no winner claim

## Next

`LOCAL-I64-MAP-DIRECT-STORAGE-SHADOW-001` may inspect
`LocalFastPathFact + LocalI64MapDirectStoragePlan` together as a shadow-only
candidate. It must still avoid lowering changes.
