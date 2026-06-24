# 296x-926 LOCAL-I64-MAP-ENTRY-TABLE-MATERIALIZATION-VALIDATION-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-i64-map-entry-table-materialization-validation-v0
source_evidence=296x-925
row_kind=validation
target_front=kilo_leaf_map_get_dynamic_covered_i64

python_entry_dispatch_tests=pass
python_collection_method_call_tests=pass
pilot_guard=pass
guard_surface_guard=pass
current_state_pointer_guard=pass
diff_check=pass
target_aot_reachability=deferred_to_measurement

runtime_helper_import_required=0
new_runtime_helper_enabled=0
product_mapbox_storage_changed=0
product_hasher_swap=0
sidecar_storage=0
mirbuilder_map_storage_ownership=0
winner_claim=0

next_task=LOCAL-I64-MAP-ENTRY-TABLE-MATERIALIZATION-MEASUREMENT-001
summary=ok
```

## Validation Commands

```bash
PYTHONPATH=src/llvm_py:. python3 -m unittest \
  src.llvm_py.tests.test_collection_method_call.TestCollectionMethodCall.test_local_i64_map_entry_table_dispatch_uses_const_tracking_rows \
  src.llvm_py.tests.test_collection_method_call.TestCollectionMethodCall.test_local_i64_map_entry_table_dispatch_rejects_non_const_value

PYTHONPATH=src/llvm_py:. python3 -m unittest \
  src.llvm_py.tests.test_collection_method_call.TestCollectionMethodCall

bash tools/checks/k2_wide_phase296x_local_i64_map_entry_table_materialization_pilot_guard.sh
bash tools/checks/k2_wide_phase296x_local_i64_map_entry_table_materialization_guard_surface_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

The unit-level backend shape is valid:

```text
EntryValueTrackingRows
  + LocalFastPathFact
  + LocalI64MapDirectStoragePlan
  -> backend-local const i64 key dispatch
  -> fallback nyash.map.slot_load_hh
```

This is not a performance claim and not a target-front reachability claim.
The next row must measure whether the exact-AOT target front reaches this
backend-local dispatch and whether it has any meaningful effect.

## Stop Lines

- no Hako-vs-C winner claim
- no product `MapBox` storage change
- no product hasher swap
- no sidecar storage
- no MIRBuilder map storage ownership
- no new runtime helper
- no helper-name or benchmark-name inference
