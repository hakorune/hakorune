# 296x-890 LOCAL-I64-MAP-GET-PILOT-GUARD-SURFACE-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-i64-map-get-pilot-guard-surface-v0
source_evidence=296x-889
row_kind=guard_surface
target_front=kilo_leaf_map_get_dynamic_covered_i64

selected_consumer=llvm_py_collection_method_call
selected_metadata=map_repr.local_i64_key_map_shadow
selected_route_kind=map_load_scalar_i64
allowed_backend_change=shadow_metadata_get_consumer_only
allowed_helper_target=local_i64_map_get_pilot_helper

post_local_i64_map_shadow_get_consumer_enabled=1
post_product_mapbox_storage_changed=0
post_product_hasher_swap=0
post_sidecar_storage=0
post_mirbuilder_map_storage_ownership=0
post_benchmark_name_branch=0
post_helper_name_inference=0
implementation_allowed=1
next_task=LOCAL-I64-MAP-GET-PILOT-001
summary=ok
```

## Scope

The implementation row may add one backend consumer for
`map_repr.local_i64_key_map_shadow` rows whose source route kind is
`map_load_scalar_i64`.

The consumer must be driven by MIR metadata at the call site, not by benchmark names, helper-symbol inference, receiver variable names, or product `MapBox` storage changes.

## Post Target

The next row should prove:

```text
local_i64_map_shadow_get_consumer_enabled=1
product_mapbox_storage_changed=0
product_hasher_swap=0
sidecar_storage=0
mirbuilder_map_storage_ownership=0
benchmark_name_branch=0
helper_name_inference=0
```

## Stop Lines

- do not change product `MapBox` storage
- do not swap product hasher
- do not add sidecar storage
- do not move map storage ownership into MIRBuilder
- do not infer from `nyash.map.scalar_load_hi`
- do not branch on benchmark name, method name alone, or receiver variable name
- do not claim Hako-vs-C winner without an equivalent C hashmap pair
