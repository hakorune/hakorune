# 296x-928 MAP-HASH-OWNER-REFRESH-AFTER-ENTRY-TABLE-MATERIALIZATION-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-map-hash-owner-refresh-after-entry-table-materialization-v0
source_evidence=296x-927
row_kind=owner_refresh_closeout
target_front=kilo_leaf_map_get_dynamic_covered_i64

previous_owner=product_map_key_domain_hash_lookup_boundary
previous_owner_source=296x-904,296x-913
previous_owner_removed_from_hot_loop=1
ny_main_hot_loop_map_helper_call_count=0
post_loop_slot_load_hh_call_count=1
post_loop_fallback_hot_owner=0

selected_owner_for_current_front=none
codegen_owner_selected=0
product_hasher_swap_allowed=0
product_mapbox_storage_change_allowed=0
sidecar_storage_allowed=0
mirbuilder_map_storage_ownership=0
post_loop_fallback_optimization_deferred=1

entry_table_materialization_lane_closed=1
fresh_front_selection_required=1
dedicated_owner_refresh_guard_added=0
next_task=MIMALLOC-FRESH-FRONT-SELECTION-AFTER-ENTRY-TABLE-CLOSEOUT-001
summary=ok
```

## Reading

Earlier owner-refresh rows selected `MapBox::get_scalar_i64_key_domain` /
`BuildHasher::hash_one` because the hot loop still reached product `MapBox`
storage through `nyash.map.local_i64_get_hi`.

`296x-927` changes the current evidence:

```text
hot loop:
  local i64 entry table materialization
  no map helper call

post-loop:
  one public fallback nyash.map.slot_load_hh call
```

The old hash owner is therefore not a current hot-loop owner for this front.
Continuing hasher work from this target would be stale owner chasing.

## Decision

Close the LocalI64Map entry-table materialization lane as a target-front
reachability and hot-loop removal success. Do not continue product `MapBox`
hasher/storage changes from this evidence.

The remaining post-loop fallback is intentionally deferred. It is not the
current hot owner and does not justify a new direct MapBox / helper-name
special case.

## Next Lane

Return to fresh front selection:

```text
MIMALLOC-FRESH-FRONT-SELECTION-AFTER-ENTRY-TABLE-CLOSEOUT-001
```

The next front must be selected from current perf evidence, not from stale
`BuildHasher` ownership observed before entry-table materialization.

## Stop Lines

- no product hasher swap
- no product `MapBox` storage change
- no sidecar storage
- no MIRBuilder map storage ownership
- no post-loop-only helper special case
- no helper-name / benchmark-name inference
- no Hako-vs-C winner claim
