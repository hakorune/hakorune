# 296x-864 MIMALLOC-MAP-GET-DYNAMIC-COVERED-I64-SCALAR-PROOF-GUARD-SURFACE-001

Status: Landed
Date: 2026-06-16

## Purpose

Fix the post-implementation guard surface for the covered dynamic i64 key
`MapGet` proof before code changes.

## Guard Contract

```text
output_contract=hako-mimalloc-map-get-dynamic-covered-i64-scalar-proof-guard-surface-v0
source_evidence=296x-863
row_kind=guard_surface

target_front=kilo_leaf_map_get_dynamic_covered_i64
target_source=benchmarks/bench_kilo_leaf_map_get_dynamic_covered_i64.hako

post_loop_map_get_route_kind=map_load_scalar_i64
post_loop_map_get_helper=nyash.map.scalar_load_hi
post_loop_runtime_data_get_hh_count=0
post_scalar_route_count_min=1

post_final_const_key_route_may_remain_map_load_any=1
post_slot_load_hh_allowed_for_final_const_get=1

proof_owner=generic_method_route_plan_map_key_coverage_proof
accepted_key_shape=nonnegative_loop_index_mod_const_positive
accepted_preseed_shape=const_i64_key_to_const_i64_value
requires_all_residues_preseeded=1

benchmark_name_branch_count=0
helper_symbol_inference_count=0
map_storage_representation_changed=0
product_mapbox_semantics_changed=0
c_pair_measurement_used=0
winner_claim=0

selected_next=MIMALLOC-MAP-GET-DYNAMIC-COVERED-I64-SCALAR-PROOF-IMPLEMENTATION-001
implementation_started=0
summary=ok
```

## Stop Lines

- do not pass the guard by editing the benchmark shape
- do not special-case `kilo_leaf_map_get_dynamic_covered_i64`
- do not special-case `i % 3` in backend shims
- do not route unproven dynamic keys to `nyash.map.scalar_load_hi`
- do not remove the final const-key fallback unless separately proven
- do not claim C parity or winner status
