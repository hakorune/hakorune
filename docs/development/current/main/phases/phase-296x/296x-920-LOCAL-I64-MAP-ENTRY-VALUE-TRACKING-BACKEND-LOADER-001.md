# 296x-920 LOCAL-I64-MAP-ENTRY-VALUE-TRACKING-BACKEND-LOADER-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-i64-map-entry-value-tracking-backend-loader-v0
source_evidence=296x-919
row_kind=metadata_loader
target_front=kilo_leaf_map_get_dynamic_covered_i64

metadata_surface=metadata.local_i64_map_entry_value_tracking_plans
backend_loader=local_i64_map_entry_value_tracking_plans_by_receiver
loader_key=receiver_value
set_site_loaded=1
key_value_loaded=1
value_value_loaded=1
key_const_if_known_loaded=1
value_const_if_known_loaded=1

product_mapbox_storage_changed=0
product_hasher_swap=0
sidecar_storage=0
mirbuilder_map_storage_ownership=0
new_runtime_helper_enabled=0
backend_lowering_enabled=0
backend_consumer_enabled=0
helper_emission_changed=0
winner_claim=0

next_task=LOCAL-I64-MAP-ENTRY-VALUE-TRACKING-SHADOW-001
summary=ok
```

## Decision

This row teaches the Python LLVM backend to load
`metadata.local_i64_map_entry_value_tracking_plans` into a receiver-keyed
metadata table:

```text
local_i64_map_entry_value_tracking_plans_by_receiver
```

The loader normalizes the set-site block/index, key/value ValueIds, and known
i64 constants. It is still passive metadata. No collection call consumer reads
the table in this row.

## Stop Lines

- no product `MapBox` storage change
- no product hasher swap
- no sidecar storage
- no MIRBuilder map storage ownership
- no new runtime helper
- no backend lowering in this row
- no backend consumer in this row
- no helper emission change in this row
- no entry table materialization
- no helper-name / benchmark-name inference
- no winner claim

## Next

`LOCAL-I64-MAP-ENTRY-VALUE-TRACKING-SHADOW-001` may add a shadow-only consumer
that cross-checks direct storage plan metadata with entry value tracking rows.
It must not emit a new helper or change lowering.
