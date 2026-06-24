# 296x-855 MIMALLOC-MAP-GET-I64-SCALAR-ROUTE-GUARD-SURFACE-001

Status: Landed
Date: 2026-06-16

## Purpose

Fix the post-implementation guard surface for the proof-backed `MapLoadScalarI64`
route selected in 296x-854.

This row still does not implement the route. It defines exactly what the next
implementation row may touch and what must remain unchanged.

## Allowed Implementation Scope

The next row may add:

```text
Rust route vocabulary:
  GenericMethodRouteKind::MapLoadScalarI64
  tag=map_load_scalar_i64
  helper=nyash.map.scalar_load_hi

Rust route producer:
  only scalar-proof-positive RuntimeDataBox.get from MapBox with i64 key

Runtime substrate:
  export nyash.map.scalar_load_hi(handle, key_i64) -> scalar i64 / missing zero

LLVM C shim route consumer:
  generic_method.get route_kind=map_load_scalar_i64
  helper validation for nyash.map.scalar_load_hi
  declaration need for nyash.map.scalar_load_hi
```

The next row must not touch:

```text
mixed RuntimeDataBox.get
direct MapBox.get handle-return route
benchmark source
typed numeric MapBox storage
stored-value constant emission
array text loop-session plan
product runtime defaults
```

## Post Target

```text
post_map_get_scalar_i64_route_kind_present=1
post_map_get_scalar_i64_route_tag=map_load_scalar_i64
post_map_get_scalar_i64_helper=nyash.map.scalar_load_hi
post_scalar_proof_runtime_data_get_route_kind=MapLoadScalarI64
post_scalar_proof_lowering_tier=WarmDirectAbi

post_mixed_runtime_data_get_route_kind=RuntimeDataLoadAny
post_mixed_runtime_data_get_helper=nyash.runtime_data.get_hh
post_direct_mapbox_get_route_kind=MapLoadAny
post_direct_mapbox_get_helper=nyash.map.slot_load_hh

post_slot_load_hi_scalar_route_usage=0
post_benchmark_source_changed=0
post_product_default_changed=0
```

## Result

```text
output_contract=hako-mimalloc-map-get-i64-scalar-route-guard-surface-v0
source_evidence=296x-854
row_kind=guard_surface
implementation_started=0

allowed_route_kind=MapLoadScalarI64
allowed_route_tag=map_load_scalar_i64
allowed_helper=nyash.map.scalar_load_hi
allowed_receiver_origin_box=MapBox
allowed_key_route=i64_const
allowed_proofs=map_set_scalar_i64_same_key_no_escape,map_set_scalar_i64_dominates_no_escape

post_map_get_scalar_i64_route_kind_present=1
post_map_get_scalar_i64_route_tag=map_load_scalar_i64
post_map_get_scalar_i64_helper=nyash.map.scalar_load_hi
post_scalar_proof_runtime_data_get_route_kind=MapLoadScalarI64
post_scalar_proof_lowering_tier=WarmDirectAbi
post_mixed_runtime_data_get_route_kind=RuntimeDataLoadAny
post_mixed_runtime_data_get_helper=nyash.runtime_data.get_hh
post_direct_mapbox_get_route_kind=MapLoadAny
post_direct_mapbox_get_helper=nyash.map.slot_load_hh
post_slot_load_hi_scalar_route_usage=0

benchmark_source_changed=0
product_default_changed=0
stored_value_constant_emission_enabled=0
typed_i64_key_map_storage_enabled=0

selected_next=MIMALLOC-MAP-GET-I64-SCALAR-ROUTE-IMPLEMENTATION-001
summary=ok
```

## Stop Line

```text
do not implement outside the next implementation row
do not route unproven get calls to MapLoadScalarI64
do not use nyash.map.slot_load_hi as scalar helper
do not change C benchmark source
do not add typed i64-key map storage
do not emit stored_value constants in this helper route
do not change RuntimeDataBox.get mixed return contract
do not change direct MapBox.get handle return contract
```

## Proof Bundle

```bash
bash tools/checks/k2_wide_phase296x_map_get_i64_scalar_route_guard_surface_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
