# 296x-908 LOCAL-I64-MAP-STORAGE-REALIZATION-BACKEND-LOADER-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-i64-map-storage-realization-backend-loader-v0
source_evidence=296x-907
row_kind=backend_metadata_loader
target_front=kilo_leaf_map_get_dynamic_covered_i64

metadata_field=local_map_storage_realization_plans
loader=src/llvm_py/builders/function_metadata.py::_load_local_map_storage_realization_plan_metadata
context_field=local_map_storage_realization_plans_by_receiver
resolver_field=local_map_storage_realization_plans_by_receiver
lookup_key=receiver_value

normalizes_receiver_value=1
normalizes_candidate_counts=1
normalizes_enablement_booleans=1
backend_lowering_enabled=0
runtime_helper_enabled=0
product_mapbox_storage_changed=0
product_hasher_swap=0
sidecar_storage=0
mirbuilder_map_storage_ownership=0
winner_claim=0

next_task=LOCAL-I64-MAP-STORAGE-REALIZATION-SHADOW-001
summary=ok
```

## Implementation

This row adds only a Python backend metadata loader:

```text
local_map_storage_realization_plans
  -> local_map_storage_realization_plans_by_receiver[receiver_value]
```

The loader normalizes receiver and counter fields to integers and enablement
fields to booleans. It does not change any lowering path.

## Stop Lines

- no backend lowering
- no runtime helper enablement
- no product `MapBox` storage change
- no product hasher swap
- no sidecar storage
- no MIRBuilder map storage ownership
- no use of fallback evidence as backend proof
- no helper-name or source-variable-name inference
- no performance winner claim

## Validation

```bash
python3 -m unittest src.llvm_py.tests.test_fastmem_metadata_loader.TestFastMemMetadataLoader.test_local_map_storage_realization_plan_loader_indexes_receivers
bash tools/checks/k2_wide_phase296x_local_i64_map_storage_realization_backend_loader_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
