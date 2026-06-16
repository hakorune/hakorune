# 296x-860 MIMALLOC-MAP-GET-DIRECT-MAPBOX-SCALAR-DEMAND-VALIDATION-001

Status: Landed
Date: 2026-06-16

## Purpose

Validate the direct `MapBox.get` scalar-demand implementation from source MIR
through exact-AOT object emission.

This row is validation only. It does not claim a benchmark win because the
available C pair for `kilo_leaf_map_getset_has` is still not a real map lookup.

## Result

```text
output_contract=hako-mimalloc-map-get-direct-mapbox-scalar-demand-validation-v0
source_evidence=296x-859
row_kind=validation

target_front=kilo_leaf_map_getset_has
c_pair_comparison_valid=0
c_pair_measurement_used=0

source_mir_loop_get_route=map_load_scalar_i64
source_mir_final_get_route=map_load_scalar_i64
source_mir_scalar_helper=nyash.map.scalar_load_hi
source_mir_slot_load_hh_count=0

source_aot_object_emitted=1
source_aot_object_scalar_helper_symbol=nyash.map.scalar_load_hi
source_aot_object_birth_symbol=nyash.map.birth_h
source_aot_object_store_symbol=nyash.map.slot_store_hhh
source_aot_object_slot_load_hh_symbol_present=0
source_aot_object_runtime_data_get_hh_symbol_present=0

source_aot_exe_ny_main_scalar_helper_call=1
source_aot_exe_ny_main_slot_load_hh_call=0

benchmark_source_changed=0
product_default_changed=0
winner_claim=0

selected_next=MIMALLOC-MAP-GET-DIRECT-MAPBOX-SCALAR-DEMAND-MEASUREMENT-001
summary=ok
```

## Stop Lines

- do not claim winner from the invalid C pair
- do not change benchmark source
- do not add typed i64-key map storage
- do not emit stored-value constants in this route
- do not change mixed RuntimeDataBox.get fallback
- do not route unproven direct MapBox.get to scalar helper

## Proof Commands

```bash
bash tools/checks/k2_wide_phase296x_map_get_direct_mapbox_scalar_demand_validation_guard.sh
```
