# 296x-859 MIMALLOC-MAP-GET-DIRECT-MAPBOX-SCALAR-DEMAND-IMPLEMENTATION-001

Status: Landed
Date: 2026-06-16

## Purpose

Implement the narrow direct `MapBox.get` scalar-demand route selected by
296x-858.

The previous `MapLoadScalarI64` row only affected proof-positive
`RuntimeDataBox.get`. This row applies the same scalar-store proof to direct
`MapBox.get` when the map is known to hold an i64 value for the same i64 key and
the receiver has not escaped or been mutated.

Unproven direct `MapBox.get` remains `MapLoadAny -> nyash.map.slot_load_hh`.

## Scope

Included:

- direct `MapBox.get` with `map_set_scalar_i64_same_key_no_escape`
- direct `MapBox.get` with `map_set_scalar_i64_dominates_no_escape`
- helper remains `nyash.map.scalar_load_hi`
- no-publication scalar return shape

Excluded:

- C benchmark changes
- Hako-vs-C winner claim
- typed i64-key MapBox storage
- stored-value constant emission
- mixed `RuntimeDataBox.get` fallback changes
- unproven direct `MapBox.get` handle-return changes

## Result

```text
output_contract=hako-mimalloc-map-get-direct-mapbox-scalar-demand-implementation-v0
source_evidence=296x-858
row_kind=implementation

direct_mapbox_get_scalar_route_enabled=1
direct_mapbox_get_scalar_route_kind=MapLoadScalarI64
direct_mapbox_get_scalar_route_tag=map_load_scalar_i64
direct_mapbox_get_scalar_helper=nyash.map.scalar_load_hi
direct_mapbox_get_scalar_return_shape=ScalarI64OrMissingZero
direct_mapbox_get_scalar_value_demand=ScalarI64
direct_mapbox_get_scalar_publication_policy=NoPublication

unproven_direct_mapbox_get_route_kind=MapLoadAny
unproven_direct_mapbox_get_helper=nyash.map.slot_load_hh
mixed_runtime_data_get_route_kind=RuntimeDataLoadAny
mixed_runtime_data_get_helper=nyash.runtime_data.get_hh

target_front=kilo_leaf_map_getset_has
target_front_loop_get_route=map_load_scalar_i64
target_front_final_get_route=map_load_scalar_i64
target_front_slot_load_hh_after=0

typed_i64_key_map_storage_enabled=0
stored_value_constant_emission_enabled=0
benchmark_source_changed=0
product_default_changed=0
winner_claim=0

selected_next=MIMALLOC-MAP-GET-DIRECT-MAPBOX-SCALAR-DEMAND-VALIDATION-001
summary=ok
```

## Proof Commands

```bash
cargo test --lib map_get_scalar -- --nocapture
cargo test --lib direct_routes -- --nocapture
cargo build --release --bin hakorune
NYASH_GC_MODE=off NYASH_DISABLE_PLUGINS=1 NYASH_SKIP_TOML_ENV=1 \
  target/release/hakorune --backend mir \
  --emit-mir-json target/map_get_scalar_probe_859/map_getset_has.mir.json \
  benchmarks/bench_kilo_leaf_map_getset_has.hako
bash tools/checks/k2_wide_phase296x_map_get_direct_mapbox_scalar_demand_implementation_guard.sh
```

## Stop Lines

- do not route unproven direct `MapBox.get` to `MapLoadScalarI64`
- do not change mixed `RuntimeDataBox.get` fallback
- do not use the invalid C `map_getset_has` pair for winner claims
- do not infer legality from helper symbol names
- do not add typed i64-key map storage
- do not emit stored-value constants in this route
- do not change benchmark source
