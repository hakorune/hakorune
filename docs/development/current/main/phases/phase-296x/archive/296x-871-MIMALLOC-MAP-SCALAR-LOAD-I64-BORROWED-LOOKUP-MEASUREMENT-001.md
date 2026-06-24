# 296x-871 MIMALLOC-MAP-SCALAR-LOAD-I64-BORROWED-LOOKUP-MEASUREMENT-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-mimalloc-map-scalar-load-i64-borrowed-lookup-measurement-v0
source_evidence=296x-870
row_kind=measurement
target_front=kilo_leaf_map_get_dynamic_covered_i64
measurement_kind=hako_only_microasm
c_pair_measurement_used=0
winner_claim=0

route_reaches_aot=1
ny_main_loop_helper=nyash.map.scalar_load_hi
ny_main_runtime_data_get_hh_import=0
ny_main_final_const_get_helper=nyash.map.slot_load_hh

perf_top_symbol_0=nyash.map.scalar_load_hi
perf_top_symbol_0_overhead_pct=53.94
perf_top_symbol_1=core::hash::BuildHasher::hash_one
perf_top_symbol_1_overhead_pct=41.72
perf_top_symbol_2=nyash_rust::boxes::map_box::MapBox::get_scalar_i64_key_str
perf_top_symbol_2_overhead_pct=2.81
perf_top_symbol_3=<core::hash::sip::Hasher<S> as core::hash::Hasher>::write
perf_top_symbol_3_overhead_pct=0.76
perf_top_symbol_4=__memcmp_evex_movbe
perf_top_symbol_4_overhead_pct=0.65

previous_spec_to_string_top_symbol_removed=1
previous_share_box_top_symbol_removed=1
previous_get_opt_key_str_top_symbol_removed=1
narrow_slice_effect_observed=1

observed_owner_shift=map_scalar_helper_string_hash_key_domain
route_proof_next_owner=none
compiler_route_next_owner=none
map_storage_next_owner=MapBox_string_key_hash_domain_or_key_alias_plan
selected_next=MIMALLOC-MAP-SCALAR-LOAD-I64-HASHMAP-KEY-DOMAIN-OWNER-SELECTION-001
summary=ok
```

## Evidence

Command:

```bash
bash tools/perf/build_perf_release.sh
bash tools/perf/bench_micro_aot_asm.sh kilo_leaf_map_get_dynamic_covered_i64 ny_main 3
```

The generated loop still reaches the intended scalar helper:

```text
call   nyash.map.scalar_load_hi
```

The previous top costs:

```text
<i64 as alloc::string::SpecToString>::spec_to_string
MapBox::share_box
MapBox::get_opt_key_str
```

are no longer top symbols. The remaining hot path is now concentrated in the
scalar helper and `HashMap<String, ...>` hashing:

```text
53.94% nyash.map.scalar_load_hi
41.72% core::hash::BuildHasher::hash_one
 2.81% MapBox::get_scalar_i64_key_str
```

## Decision

The borrowed scalar lookup slice did what it was allowed to do: remove i64 key
heap String construction and visible-read clone/share from the scalar helper
path. It does not finish the MapBox gap because the remaining owner is now
string-key hashing / key-domain representation.

The next row must not jump directly to an i64 sidecar. It must first select a
key-domain plan that preserves public stringified-key alias semantics.

## Stop Lines

- do not claim C parity from this Hako-only measurement
- do not change MapBox storage from this measurement row
- do not introduce i64 sidecar storage without alias-plan design
- do not continue route proof work without fresh route evidence
- do not infer from benchmark names or helper symbols

