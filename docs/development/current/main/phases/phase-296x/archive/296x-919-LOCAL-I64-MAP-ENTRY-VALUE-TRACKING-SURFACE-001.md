# 296x-919 LOCAL-I64-MAP-ENTRY-VALUE-TRACKING-SURFACE-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-i64-map-entry-value-tracking-surface-v0
source_evidence=296x-918
row_kind=passive_metadata_surface
target_front=kilo_leaf_map_get_dynamic_covered_i64

metadata_surface=FunctionMetadata.local_i64_map_entry_value_tracking_plans
mir_json_surface=metadata.local_i64_map_entry_value_tracking_plans
entry_tracking_owner=MapStoragePlan
tracked_receiver_value=1
tracked_set_site=1
tracked_key_value=1
tracked_value_value=1
tracked_key_const_if_known=1
tracked_value_const_if_known=1

product_mapbox_storage_changed=0
product_hasher_swap=0
sidecar_storage=0
mirbuilder_map_storage_ownership=0
new_runtime_helper_enabled=0
backend_loader_enabled=0
backend_lowering_enabled=0
helper_emission_changed=0
winner_claim=0

next_task=LOCAL-I64-MAP-ENTRY-VALUE-TRACKING-BACKEND-LOADER-001
summary=ok
```

## Decision

This row adds passive set-site value tracking for local i64 Map direct storage.
The metadata is derived from MIR set callsites already selected by the local i64
map candidate chain.

Each row records:

```text
receiver_value
set_block
set_instruction_index
key_value
value_value
key_const_if_known
value_const_if_known
backend_lowering_enabled=0
runtime_helper_enabled=0
```

The row does not materialize a local table and does not enable backend lowering.

## Stop Lines

- no product `MapBox` storage change
- no product hasher swap
- no sidecar storage
- no MIRBuilder map storage ownership
- no new runtime helper
- no backend loader in this row
- no backend lowering in this row
- no helper emission change in this row
- no entry table materialization
- no helper-name / benchmark-name inference
- no winner claim

## Next

`LOCAL-I64-MAP-ENTRY-VALUE-TRACKING-BACKEND-LOADER-001` may teach the Python
backend to read these rows as metadata only. It must not lower differently.
