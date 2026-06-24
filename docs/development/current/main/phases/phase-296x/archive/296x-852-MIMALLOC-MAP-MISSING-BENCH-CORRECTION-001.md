# 296x-852 MIMALLOC-MAP-MISSING-BENCH-CORRECTION-001

Status: Landed
Date: 2026-06-16

## Purpose

Correct the `kilo_leaf_map_get_missing` interpretation before continuing
optimization.

The previous perf comparison treated `kilo_leaf_map_get_missing` as a valid
Hako-vs-C map lookup comparison. That is false: the C pair does not perform a
map lookup. It only performs a volatile integer comparison.

This row is docs/selection only. It does not change compiler lowering, runtime
helpers, benchmark source, ObjectPlan, RoutePlan, or product runtime behavior.

## Verified Evidence

```text
c_pair_source=benchmarks/c/bench_kilo_leaf_map_get_missing.c
c_pair_performs_map_lookup=0
c_pair_shape=volatile_i64_compare_only
c_pair_contains_hashmap=0
c_pair_contains_lookup=0

hako_source=benchmarks/bench_kilo_leaf_map_get_missing.hako
hako_source_performs_map_get=1
hako_loop_call=map.get(0)

benchmark_pair_apples_to_oranges=1
map_missing_c_comparison_valid=0
map_missing_route_winner_claim_retracted=1
map_missing_previous_ratio_claim_valid=0
```

The `MapMissingEmptyRoute` semantic route remains a separate compiler
optimization fact. This correction only retracts the C-comparison winner claim.

## Runtime Gap

The benchmark correction does expose a real next investigation target:
`MapBox.get` does not have the same i64 scalar route shape that `MapBox.has`
already has.

Current route shape:

```text
MapHas i64:
  route_kind=map_contains_i64
  helper=nyash.map.probe_hi

MapGet current generic path:
  route_kind=runtime_data_load_any or map_load_any
  helper=nyash.runtime_data.get_hh or nyash.map.slot_load_hh
```

Observed substrate gap:

```text
map_get_i64_scalar_route_enabled=0
map_get_i64_probe_helper_enabled=0
map_get_i64_key_string_conversion_visible=1
map_get_runtime_data_facade_visible=1
host_handle_boundary_visible=1
```

The next work must inventory this route gap before implementation. Do not infer
a fix from helper names alone.

## Result

```text
output_contract=hako-mimalloc-map-missing-bench-correction-v0
source_evidence=benchmarks/c/bench_kilo_leaf_map_get_missing.c,benchmarks/bench_kilo_leaf_map_get_missing.hako
row_kind=correction
implementation_started=0
perf_claim_correction=1

target_front=kilo_leaf_map_get_missing
c_pair_source=benchmarks/c/bench_kilo_leaf_map_get_missing.c
c_pair_performs_map_lookup=0
c_pair_shape=volatile_i64_compare_only
hako_source_performs_map_get=1
benchmark_pair_apples_to_oranges=1

map_missing_c_comparison_valid=0
map_missing_route_winner_claim_retracted=1
map_missing_previous_ratio_claim_valid=0
map_missing_empty_route_semantic_fact_invalidated=0

map_has_i64_scalar_route_present=1
map_get_i64_scalar_route_present=0
map_get_runtime_data_facade_visible=1
map_get_i64_key_string_conversion_visible=1

array_text_loop_session_plan_surface_still_landed=1
array_text_loop_session_inventory_resume_after_map_correction=1

benchmark_source_changed=0
compiler_lowering_changed=0
runtime_helper_changed=0
product_default_changed=0

selected_next=MIMALLOC-MAP-GET-I64-SCALAR-ROUTE-INVENTORY-001
summary=ok
```

## Stop Line

```text
do not claim Hako beat C on kilo_leaf_map_get_missing from the old C pair
do not use volatile-compare C timing as map lookup evidence
do not invalidate the semantic MapMissingEmptyRoute solely from benchmark hygiene
do not add MapGetI64 lowering before route inventory
do not infer MapGetI64 legality from helper symbols alone
do not change benchmark source in this correction row
do not lose the 296x-851 ArrayTextLoopSessionPlan resume pointer
```

## Proof Bundle

```bash
bash tools/checks/k2_wide_phase296x_map_missing_bench_correction_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
