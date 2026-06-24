# 296x-913 MAP-HASH-OWNER-REFRESH-AFTER-LOCAL-I64-STORAGE-REALIZATION-CLOSEOUT-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-map-hash-owner-refresh-after-local-i64-storage-realization-closeout-v0
source_evidence=296x-912
row_kind=owner_refresh
target_front=kilo_leaf_map_get_dynamic_covered_i64

measurement_runner=tools/perf/bench_micro_aot_asm.sh
measurement_runner_mode=direct
measurement_runs=2
ny_main_hot_loop_calls_local_i64_get_hi=1
ny_main_post_loop_slot_load_hh_call=1

run1_samples=40
run1_event_count=176082326
run1_top_symbol=MapBox::get_scalar_i64_key_domain
run1_top_symbol_percent=60.10
run1_hash_one_percent=36.84
run1_get_scalar_i64_key_i64_percent=3.02

run2_samples=44
run2_event_count=194845255
run2_top_symbol=MapBox::get_scalar_i64_key_domain
run2_top_symbol_percent=67.10
run2_hash_one_percent=21.89
run2_get_scalar_i64_key_i64_percent=5.49
run2_local_i64_get_hi_percent=2.75

selected_owner=product_map_key_domain_hash_lookup_boundary
selected_owner_confidence=medium
local_helper_reaches_product_mapbox_storage=1
codegen_owner_selected=0
product_hasher_swap_allowed=0
product_mapbox_storage_change_allowed=0
sidecar_storage_allowed=0
mirbuilder_map_storage_ownership=0
winner_claim=0

next_task=LOCAL-I64-MAP-DIRECT-STORAGE-POLICY-DESIGN-001
summary=ok
```

## Evidence

The current AOT `ny_main` hot loop calls the local helper:

```text
call nyash.map.local_i64_get_hi
```

The remaining top symbols are inside product `MapBox` lookup:

```text
MapBox::get_scalar_i64_key_domain
core::hash::BuildHasher::hash_one
MapBox::get_scalar_i64_key_i64
```

`nyash.map.local_i64_get_hi` still delegates through:

```text
map_scalar_load_i64
  -> MapBox::get_scalar_i64_key_i64
  -> MapBox::get_scalar_i64_key_domain
```

## Decision

The next owner is not Python backend codegen and not the old shadow consumer.
The remaining boundary is product MapBox hash lookup reached through the local
i64 helper.

Do not swap the product hasher or mutate product `MapBox` storage from this
evidence. The next row must design whether an exact-AOT/local-first
`LocalI64KeyMap` direct storage policy can avoid entering product MapBox lookup
before publication.

## Stop Lines

- no product hasher swap
- no product `MapBox` storage change
- no sidecar storage without a storage-policy row
- no MIRBuilder map storage ownership
- no helper-name / benchmark-name inference
- no performance winner claim
