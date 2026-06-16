# 296x-891 LOCAL-I64-MAP-GET-PILOT-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-i64-map-get-pilot-v0
source_evidence=296x-890
row_kind=implementation
target_front=kilo_leaf_map_get_dynamic_covered_i64

local_i64_map_shadow_get_consumer_enabled=1
selected_metadata=map_repr.local_i64_key_map_shadow
selected_route_kind=map_load_scalar_i64
selected_backend=src/llvm_py/instructions/mir_call/collection_method_call.py
selected_helper=nyash.map.local_i64_get_hi
helper_implementation=delegates_to_map_scalar_load_i64

product_mapbox_storage_changed=0
product_hasher_swap=0
sidecar_storage=0
mirbuilder_map_storage_ownership=0
benchmark_name_branch=0
helper_name_inference=0
winner_claim=0
next_task=LOCAL-I64-MAP-GET-PILOT-VALIDATION-001
summary=ok
```

## Implementation

The Python LLVM backend now checks call-site `map_repr_plans_by_site` metadata
for a `map_repr.local_i64_key_map_shadow` row whose source route kind is
`map_load_scalar_i64`.

When receiver and key value ids match, the backend emits:

```text
nyash.map.local_i64_get_hi(handle, key_i64)
```

The new kernel symbol currently delegates to existing
`map_scalar_load_i64(handle, key_i64)`.

This row proves the metadata consumer seam only. It does not change product `MapBox` storage or claim a performance winner.

## Tests

```bash
PYTHONPATH=.:src/llvm_py python3 -m unittest \
  src.llvm_py.tests.test_collection_method_call.TestCollectionMethodCall.test_mapbox_local_i64_shadow_get_uses_metadata_pilot_helper

cargo check --release --bin hakorune
```

## Stop Lines

- no product MapBox storage change
- no product hasher swap
- no sidecar storage
- no MIRBuilder map storage ownership
- no benchmark-name / helper-name / variable-name special case
- no Hako-vs-C winner claim
