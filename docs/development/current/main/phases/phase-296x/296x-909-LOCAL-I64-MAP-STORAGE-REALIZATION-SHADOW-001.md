# 296x-909 LOCAL-I64-MAP-STORAGE-REALIZATION-SHADOW-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-i64-map-storage-realization-shadow-v0
source_evidence=296x-908
row_kind=backend_guard_refinement
target_front=kilo_leaf_map_get_dynamic_covered_i64

fact_only_fastpath_enabled=0
fact_plus_storage_plan_required=1
storage_plan_lookup_key=receiver_value
required_storage_representation=local_i64_key_map
requires_publication_materialization_required=1
requires_backend_lowering_enabled=0
requires_runtime_helper_enabled=0

legacy_local_i64_shadow_consumer_retained=1
legacy_shadow_retire_required=1
new_runtime_helper_enabled=0
product_mapbox_storage_changed=0
product_hasher_swap=0
sidecar_storage=0
mirbuilder_map_storage_ownership=0
winner_claim=0

next_task=LOCAL-I64-MAP-LEGACY-SHADOW-CONSUMER-RETIRE-DESIGN-001
summary=ok
```

## Implementation

The local fast-path fact consumer now requires a matching
`LocalMapStorageRealizationPlan` for the receiver before it emits
`nyash.map.local_i64_get_hi` through the `local_fastpath_map_get_hi` path.

```text
LocalFastPathFact only
  -> fallback

LocalFastPathFact + LocalMapStorageRealizationPlan(receiver_value)
  -> local fast-path helper
```

This row does not remove the older `map_repr.local_i64_key_map_shadow` consumer.
That legacy consumer remains a separate retire/design task so the row stays
small and avoids mixing old-pilot retirement with the new Fact+Plan guard.

## Stop Lines

- no product `MapBox` storage change
- no product hasher swap
- no sidecar storage
- no MIRBuilder map storage ownership
- no new runtime helper
- no helper-name or source-variable-name inference
- no fallback evidence as backend proof
- no performance winner claim

## Validation

```bash
python3 -m unittest src.llvm_py.tests.test_collection_method_call.TestCollectionMethodCall.test_mapbox_local_fastpath_fact_get_uses_known_receiver_direct_call_helper
python3 -m unittest src.llvm_py.tests.test_collection_method_call.TestCollectionMethodCall.test_mapbox_local_fastpath_fact_get_requires_storage_plan
bash tools/checks/k2_wide_phase296x_local_i64_map_storage_realization_shadow_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
