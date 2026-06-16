# 296x-912 LOCAL-I64-MAP-STORAGE-REALIZATION-CLOSEOUT-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-i64-map-storage-realization-closeout-v0
source_evidence=296x-905..296x-911
row_kind=closeout
target_front=kilo_leaf_map_get_dynamic_covered_i64

closed_family=local_i64_map_storage_realization
metadata_surface_landed=1
backend_loader_landed=1
fact_plus_plan_guard_landed=1
legacy_shadow_consumer_retired=1

backend_fastpath_owner=LocalFastPathFact_plus_LocalMapStorageRealizationPlan
backend_reads_fallback_evidence=0
backend_reads_helper_symbol=0
backend_reads_source_variable_name=0

product_mapbox_storage_changed=0
product_hasher_swap=0
sidecar_storage=0
mirbuilder_map_storage_ownership=0
new_runtime_helper_enabled=0
winner_claim=0

next_task=MAP-HASH-OWNER-REFRESH-AFTER-LOCAL-I64-STORAGE-REALIZATION-CLOSEOUT-001
summary=ok
```

## Closeout

This series moved local i64 map storage realization from a legacy shadow
consumer to an explicit Fact+Plan backend contract:

```text
LocalFastPathFact at callsite
+ LocalMapStorageRealizationPlan(receiver_value)
-> local fast-path helper may be emitted
```

Fallback-only evidence, helper symbols, source variable names, and legacy
`map_repr.local_i64_key_map_shadow` metadata are not backend proof.

## Stop Line

This closeout does not claim a new performance win. The measured remaining hot
owner before this cleanup was the product `MapKeyDomain` hash lookup boundary.
The next row must refresh that owner from current evidence before any hasher or
storage-policy implementation.

## Validation

```bash
bash tools/checks/k2_wide_phase296x_local_i64_map_storage_realization_closeout_guard.sh
bash tools/checks/k2_wide_phase296x_local_i64_map_legacy_shadow_consumer_retire_guard.sh
bash tools/checks/k2_wide_phase296x_local_i64_map_storage_realization_shadow_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
