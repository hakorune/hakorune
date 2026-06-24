# 296x-917 LOCAL-I64-MAP-DIRECT-STORAGE-SHADOW-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-i64-map-direct-storage-shadow-v0
source_evidence=296x-916
row_kind=shadow_only_consumer
target_front=kilo_leaf_map_get_dynamic_covered_i64

shadow_candidate_requires_local_fastpath_fact=1
shadow_candidate_requires_direct_storage_plan=1
shadow_candidate_requires_representation=closed_world_i64_key_value_table
shadow_candidate_requires_entry_value_tracking_enabled=0
shadow_candidate_requires_backend_lowering_enabled=0
shadow_candidate_requires_runtime_helper_enabled=0
shadow_candidate_requires_publication_materialization_required=1

product_mapbox_storage_changed=0
product_hasher_swap=0
sidecar_storage=0
mirbuilder_map_storage_ownership=0
new_runtime_helper_enabled=0
backend_lowering_enabled=0
helper_emission_changed=0
winner_claim=0

next_task=LOCAL-I64-MAP-DIRECT-STORAGE-ENABLEMENT-DESIGN-001
summary=ok
```

## Decision

The Python collection-call seam now has a shadow-only candidate helper for the
next direct storage route:

```text
LocalFastPathFact + LocalI64MapDirectStoragePlan(receiver)
```

The helper is intentionally not wired into lowering. It only verifies that the
two required proof surfaces can be matched without falling back to helper names,
source variable names, or product `MapBox` internals.

## Stop Lines

- no product `MapBox` storage change
- no product hasher swap
- no sidecar storage
- no MIRBuilder map storage ownership
- no new runtime helper
- no backend lowering in this row
- no helper emission change in this row
- no entry value table materialization in this row
- no helper-name / benchmark-name inference
- no winner claim

## Next

`LOCAL-I64-MAP-DIRECT-STORAGE-ENABLEMENT-DESIGN-001` must decide whether the
first executable slice can be enabled without entry-value tracking, or whether a
separate entry-value tracking row is required first.
