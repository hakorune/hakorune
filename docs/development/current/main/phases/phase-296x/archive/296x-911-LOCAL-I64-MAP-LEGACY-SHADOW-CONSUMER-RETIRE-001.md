# 296x-911 LOCAL-I64-MAP-LEGACY-SHADOW-CONSUMER-RETIRE-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-i64-map-legacy-shadow-consumer-retire-v0
source_evidence=296x-910
row_kind=implementation
target_front=kilo_leaf_map_get_dynamic_covered_i64

retired_backend_consumer=map_repr.local_i64_key_map_shadow
retired_backend_function=_current_local_i64_map_shadow_get_plan
legacy_shadow_helper_emission_enabled=0
remaining_fastpath_owner=LocalFastPathFact_plus_LocalMapStorageRealizationPlan
fact_only_fastpath_enabled=0
fact_plus_storage_plan_required=1

legacy_metadata_producer_retained=1
legacy_metadata_backend_consumable=0
product_mapbox_storage_changed=0
product_hasher_swap=0
sidecar_storage=0
mirbuilder_map_storage_ownership=0
new_runtime_helper_enabled=0
winner_claim=0

next_task=LOCAL-I64-MAP-STORAGE-REALIZATION-CLOSEOUT-001
summary=ok
```

## Implementation

This row removes the Python backend consumer for
`map_repr.local_i64_key_map_shadow`. The old metadata may still be emitted by
MIR as observation / historical evidence, but backend lowering no longer treats
it as proof.

The only remaining local map fast path is:

```text
LocalFastPathFact + LocalMapStorageRealizationPlan(receiver_value)
```

## Stop Lines

- no product `MapBox` storage change
- no product hasher swap
- no sidecar storage
- no MIRBuilder map storage ownership
- no new runtime helper
- no fallback evidence as backend proof
- no helper-name or source-variable-name inference
- no performance winner claim

## Validation

```bash
PYTHONPATH=src/llvm_py:. python3 -m unittest src.llvm_py.tests.test_collection_method_call.TestCollectionMethodCall.test_mapbox_local_i64_shadow_get_falls_back_after_consumer_retire
PYTHONPATH=src/llvm_py:. python3 -m unittest src.llvm_py.tests.test_collection_method_call.TestCollectionMethodCall.test_mapbox_local_fastpath_fact_get_uses_known_receiver_direct_call_helper
bash tools/checks/k2_wide_phase296x_local_i64_map_legacy_shadow_consumer_retire_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
