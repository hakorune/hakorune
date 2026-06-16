# 296x-893 LOCAL-I64-MAP-GET-PILOT-MEASUREMENT-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-i64-map-get-pilot-measurement-v0
source_evidence=296x-892
row_kind=measurement
target_front=kilo_leaf_map_get_dynamic_covered_i64
measurement_command=bash tools/perf/build_perf_release.sh && KEEP_PERF_MICROASM_ARTIFACTS=1 PERF_MICROASM_RUNNER_MODE=direct bash tools/perf/bench_micro_aot_asm.sh kilo_leaf_map_get_dynamic_covered_i64 ny_main 1

ny_main_loop_helper_before=nyash.map.scalar_load_hi
ny_main_loop_helper_after=nyash.map.local_i64_get_hi
local_i64_get_helper_reached=1
scalar_load_hi_loop_call_removed=1
top_symbol_0=nyash_rust::boxes::map_box::MapBox::get_scalar_i64_key_domain
top_symbol_0_pct=50.23
top_symbol_1=core::hash::BuildHasher::hash_one
top_symbol_1_pct=46.72
top_symbol_2=nyash_rust::boxes::map_box::MapBox::get_scalar_i64_key_i64
top_symbol_2_pct=3.03

helper_delegates_to_existing_scalar_load=1
remaining_hot_owner=map_hash_lookup_boundary
winner_claim=0
next_task=LOCAL-I64-MAP-GET-PILOT-CLOSEOUT-001
summary=ok
```

## Evidence

The generated `ny_main` loop now calls the local-i64 pilot helper:

```asm
40e935: 48 89 df              mov    %rbx,%rdi
40e938: e8 63 38 00 00        call   4121a0 <nyash.map.local_i64_get_hi>
40e93d: 49 01 c7              add    %rax,%r15
```

The final post-loop literal get still uses the public fallback helper:

```asm
40e950: e8 eb 34 00 00        call   411e40 <nyash.map.slot_load_hh>
```

Perf top remains inside canonical MapKeyDomain hash lookup:

```text
50.23% MapBox::get_scalar_i64_key_domain
46.72% BuildHasher::hash_one
 3.03% MapBox::get_scalar_i64_key_i64
```

## Decision

This row validates reachability, not a performance win.

The pilot helper currently delegates to existing `map_scalar_load_i64`, so the
remaining hot owner is still `map_hash_lookup_boundary`. The next row should
close the metadata-consumer pilot and select the next storage/hash owner
without touching product `MapBox` defaults.

## Stop Lines

- no Hako-vs-C winner claim
- no product MapBox storage change
- no product hasher swap
- no sidecar storage
- no MIRBuilder map storage ownership
- no benchmark-name / helper-name / variable-name special case
