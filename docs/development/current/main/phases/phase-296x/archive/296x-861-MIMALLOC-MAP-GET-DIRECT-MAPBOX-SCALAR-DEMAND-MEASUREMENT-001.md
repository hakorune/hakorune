# 296x-861 MIMALLOC-MAP-GET-DIRECT-MAPBOX-SCALAR-DEMAND-MEASUREMENT-001

Status: Landed
Date: 2026-06-16

## Purpose

Measure/attribute the direct `MapBox.get` scalar-demand row without reusing an
invalid C pair or loader-dominated perf sample as a winner claim.

The target source front `kilo_leaf_map_getset_has` now reaches the scalar route,
but it is no longer a good performance keeper front: current exact-AOT lowering
folds the loop into a compact `ny_main` shape with one map store, one scalar map
load, and an add of the folded loop contribution. This validates route
selection but does not provide a meaningful repeated map-get body measurement.

## Evidence

```text
output_contract=hako-mimalloc-map-get-direct-mapbox-scalar-demand-measurement-v0
source_evidence=296x-860
row_kind=measurement_attribution

target_front=kilo_leaf_map_getset_has
c_pair_comparison_valid=0
c_pair_measurement_used=0
winner_claim=0

source_mir_scalar_route_count_min=2
source_mir_slot_load_hh_count=0
source_mir_runtime_data_get_hh_count=0

source_aot_object_scalar_helper_symbol=nyash.map.scalar_load_hi
source_aot_object_birth_symbol=nyash.map.birth_h
source_aot_object_store_symbol=nyash.map.slot_store_hhh
source_aot_object_slot_load_hh_symbol_present=0
source_aot_object_runtime_data_get_hh_symbol_present=0

source_aot_exe_ny_main_body_shape=folded_single_store_single_scalar_load
body_loop_repeated_map_get_measurement_available=0
loader_dominated_perf_sample_observed=1

measurement_keeper_claim=0
implementation_keeper_claim=route_reaches_aot_only
selected_next=MIMALLOC-MAP-GET-NONFOLDED-SCALAR-FRONT-SELECTION-001
summary=ok
```

## Stop Lines

- do not claim Hako-vs-C winner from `kilo_leaf_map_getset_has`
- do not use the invalid C map pair as map lookup evidence
- do not use loader-dominated perf samples as kernel evidence
- do not add benchmark-specific route branches
- do not change benchmark source in this row
- do not pursue more MapGet implementation without a non-folded scalar front

## Next

Select or create a non-folded Hako-only scalar map-get front that keeps repeated
`MapLoadScalarI64` work in the body. The next row must first prove the front is
not closed-form folded before any new implementation or winner claim.
