# 296x-866 MIMALLOC-MAP-GET-DYNAMIC-COVERED-I64-SCALAR-PROOF-MEASUREMENT-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-mimalloc-map-get-dynamic-covered-i64-scalar-proof-measurement-v0
source_evidence=296x-865
row_kind=measurement
target_front=kilo_leaf_map_get_dynamic_covered_i64
target_source=benchmarks/bench_kilo_leaf_map_get_dynamic_covered_i64.hako
measurement_kind=hako_only_microasm
c_pair_measurement_used=0
winner_claim=0

route_reaches_aot=1
ny_main_loop_helper=nyash.map.scalar_load_hi
ny_main_runtime_data_get_hh_import=0
ny_main_final_const_get_helper=nyash.map.slot_load_hh

perf_top_symbol_0=<i64 as alloc::string::SpecToString>::spec_to_string
perf_top_symbol_0_overhead_pct=29.03
perf_top_symbol_1=core::hash::BuildHasher::hash_one
perf_top_symbol_1_overhead_pct=27.35
perf_top_symbol_2=nyash_rust::boxes::map_box::MapBox::get_opt_key_str
perf_top_symbol_2_overhead_pct=7.08
perf_top_symbol_3=<nyash_rust::boxes::map_box::MapBox as nyash_rust::box_trait::NyashBox>::share_box
perf_top_symbol_3_overhead_pct=6.62
perf_top_symbol_4=nyash.map.scalar_load_hi
perf_top_symbol_4_overhead_pct=5.94

observed_owner_shift=map_scalar_helper_i64_key_string_storage
route_proof_next_owner=none
compiler_route_next_owner=none
map_storage_next_owner=MapBox_i64_key_storage_or_scalar_helper_key_encoding
selected_next=MIMALLOC-MAP-SCALAR-LOAD-I64-KEY-STORAGE-OWNER-SELECTION-001
summary=ok
```

## Evidence

`tools/perf/bench_micro_aot_asm.sh kilo_leaf_map_get_dynamic_covered_i64 ny_main 3`
showed the generated `ny_main` loop calls `nyash.map.scalar_load_hi`:

```text
call   nyash.map.scalar_load_hi
```

The object route no longer imports `nyash.runtime_data.get_hh`. The remaining
hot cost is inside the scalar helper / current MapBox storage path:

```text
29.03% <i64 as alloc::string::SpecToString>::spec_to_string
27.35% core::hash::BuildHasher::hash_one
 7.08% nyash_rust::boxes::map_box::MapBox::get_opt_key_str
 6.62% MapBox::share_box
 5.94% nyash.map.scalar_load_hi
```

## Decision

The route proof is a keeper as a route-to-AOT cleanup, but it does not finish the
hot-path work. The next owner is no longer generic-method routing; it is the
current MapBox i64-key path converting scalar keys through String-key storage.

## Stop Lines

- do not claim C parity from this Hako-only measurement
- do not continue adding route proofs without a fresh route owner
- do not change MapBox storage in this measurement row
- do not introduce benchmark-name branches
- do not bypass product MapBox semantics silently

