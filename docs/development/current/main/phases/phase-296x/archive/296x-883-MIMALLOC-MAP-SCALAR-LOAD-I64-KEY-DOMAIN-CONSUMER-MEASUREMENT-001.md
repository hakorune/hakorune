# 296x-883 MIMALLOC-MAP-SCALAR-LOAD-I64-KEY-DOMAIN-CONSUMER-MEASUREMENT-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-mimalloc-map-scalar-load-i64-key-domain-consumer-measurement-v0
source_evidence=296x-882
row_kind=measurement
target_front=kilo_leaf_map_get_dynamic_covered_i64

measurement_command=bash tools/perf/build_perf_release.sh && bash tools/perf/bench_micro_aot_asm.sh kilo_leaf_map_get_dynamic_covered_i64 ny_main 3
cycles_before=1180142598
cycles_after=484694805
cycles_reduction_pct=58.93

spec_to_string_top_before_percent=23.81
spec_to_string_top_after_percent=0
scalar_load_hi_top_before_percent=31.14
scalar_load_hi_top_after_percent=3.30

top_symbol_1=nyash_rust::boxes::map_box::MapBox::get_scalar_i64_key_domain
top_symbol_1_percent=64.18
top_symbol_2=core::hash::BuildHasher::hash_one
top_symbol_2_percent=31.43
top_symbol_3=nyash.map.scalar_load_hi
top_symbol_3_percent=3.30

target_counter_shrinks=1
selected_keeper=scalar_helper_key_domain_consumer
winner_claim=1
selected_next=MIMALLOC-MAP-SCALAR-LOAD-I64-POST-DOMAIN-CONSUMER-OWNER-SELECTION-001
summary=ok
```

## Interpretation

The direct i64-domain scalar helper removed the decimal text conversion owner:

```text
before:
  scalar_load_hi=31.14%
  SpecToString=23.81%
  cycles~=1.18B

after:
  scalar_load_hi=3.30%
  SpecToString=0%
  cycles~=0.48B
```

This is a keeper for the active target. The remaining owner is now inside the
domain-keyed lookup:

```text
MapBox::get_scalar_i64_key_domain
core::hash::BuildHasher::hash_one
```

## Stop Lines

- do not jump directly to a hasher swap
- do not add sidecar storage from this measurement alone
- do not move object management into MIRBuilder
- do not change public MapBox semantics

## Next

Run owner selection before implementation. The likely choices are:

```text
1. specialize MapKeyDomain hashing / lookup shape
2. introduce typed i64-keyed MapBox representation
3. stop this front if further wins require a larger storage-substrate design
```
