# 296x-858 MIMALLOC-MAP-GET-DIRECT-MAPBOX-SCALAR-DEMAND-DESIGN-001

Status: Landed
Date: 2026-06-16

## Purpose

Retarget the next map-get optimization row after validating
`MapLoadScalarI64`.

The implemented scalar route is consumed by exact-AOT when the lowering plan
contains `route_kind=map_load_scalar_i64`. However, the current product source
front `kilo_leaf_map_getset_has` still leaves the final direct `MapBox.get`
call as `map_load_any -> nyash.map.slot_load_hh` after loop folding.

Therefore, the next implementation is not measurement and not C comparison. It
is a narrow direct `MapBox.get` scalar-demand route using the same scalar-store
proof, while keeping unproven direct get as the existing handle-return route.

## Decision

```text
selected_next_owner=direct_mapbox_get_scalar_demand_route
selected_route_kind=MapLoadScalarI64
selected_helper=nyash.map.scalar_load_hi
selected_surface=MapBox.get
selected_proofs=map_set_scalar_i64_same_key_no_escape,map_set_scalar_i64_dominates_no_escape

unproven_mapbox_get_route_kind=MapLoadAny
unproven_mapbox_get_helper=nyash.map.slot_load_hh
runtime_data_scalar_route_stays_enabled=1
mixed_runtime_data_get_fallback_stays_enabled=1
```

This row does not change map storage. `nyash.map.scalar_load_hi` still uses the
current String-key map storage substrate internally.

## Result

```text
output_contract=hako-mimalloc-map-get-direct-mapbox-scalar-demand-design-v0
source_evidence=296x-857,target/map_get_scalar_probe_857/map_getset_has.mir.json,target/perf_ny_kilo_leaf_map_getset_has.microasm.1085336.exe
row_kind=design
implementation_started=0

target_front=kilo_leaf_map_getset_has
c_pair_comparison_valid=0
c_pair_measurement_used=0

runtime_data_scalar_route_validated=1
source_front_runtime_data_loop_get_route=map_load_scalar_i64
source_front_final_direct_mapbox_get_route=map_load_any
source_front_final_direct_mapbox_get_helper=nyash.map.slot_load_hh

selected_next_owner=direct_mapbox_get_scalar_demand_route
selected_route_kind=MapLoadScalarI64
selected_helper=nyash.map.scalar_load_hi
selected_surface=MapBox.get
selected_proofs=map_set_scalar_i64_same_key_no_escape,map_set_scalar_i64_dominates_no_escape

slot_load_hh_unproven_route_retained=1
runtime_data_load_any_mixed_route_retained=1
typed_i64_key_map_storage_enabled=0
stored_value_constant_emission_enabled=0
benchmark_source_changed=0
product_default_changed=0
winner_claim=0

selected_next=MIMALLOC-MAP-GET-DIRECT-MAPBOX-SCALAR-DEMAND-IMPLEMENTATION-001
summary=ok
```

## Stop Lines

- do not use the invalid C `map_getset_has` pair for winner claims
- do not infer legality from `nyash.map.slot_load_hh` or `nyash.map.scalar_load_hi` symbols
- do not route unproven direct `MapBox.get` to `MapLoadScalarI64`
- do not change mixed `RuntimeDataBox.get` fallback
- do not change direct `MapBox.get` handle contract when scalar proof is absent
- do not add typed i64-key map storage
- do not emit stored-value constants in this route
- do not change benchmark source
