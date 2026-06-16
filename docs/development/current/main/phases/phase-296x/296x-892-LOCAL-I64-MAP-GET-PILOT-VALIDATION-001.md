# 296x-892 LOCAL-I64-MAP-GET-PILOT-VALIDATION-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-i64-map-get-pilot-validation-v0
source_evidence=296x-891
row_kind=validation
target_front=kilo_leaf_map_get_dynamic_covered_i64

local_i64_map_shadow_get_consumer_enabled=1
python_metadata_consumer_test=pass
map_repr_plan_unit_tests=pass
release_cargo_check=pass
current_state_pointer_guard=pass
diff_check=pass

product_mapbox_storage_changed=0
product_hasher_swap=0
sidecar_storage=0
mirbuilder_map_storage_ownership=0
winner_claim=0
next_task=LOCAL-I64-MAP-GET-PILOT-MEASUREMENT-001
summary=ok
```

## Validation Commands

```bash
PYTHONPATH=.:src/llvm_py python3 -m unittest \
  src.llvm_py.tests.test_collection_method_call.TestCollectionMethodCall.test_mapbox_local_i64_shadow_get_uses_metadata_pilot_helper

cargo test --lib map_repr_plan -- --nocapture
cargo check --release --bin hakorune
bash tools/checks/k2_wide_phase296x_local_i64_map_get_pilot_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

The metadata consumer seam is valid:

```text
map_repr.local_i64_key_map_shadow
  + source_route_kind=map_load_scalar_i64
  -> nyash.map.local_i64_get_hi
```

This is not a performance claim. The helper still delegates to the existing
scalar Map load. The next row must measure whether the new backend seam reaches
the target front and whether it has any meaningful effect.

## Stop Lines

- no Hako-vs-C winner claim
- no product MapBox storage change
- no product hasher swap
- no sidecar storage
- no MIRBuilder map storage ownership
- no helper-name or benchmark-name inference
