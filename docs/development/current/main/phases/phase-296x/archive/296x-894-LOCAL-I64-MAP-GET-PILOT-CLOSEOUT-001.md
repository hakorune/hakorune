# 296x-894 LOCAL-I64-MAP-GET-PILOT-CLOSEOUT-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-i64-map-get-pilot-closeout-v0
source_evidence=296x-893
row_kind=closeout
target_front=kilo_leaf_map_get_dynamic_covered_i64

helper_reachability_keeper=1
local_i64_get_helper_reached=1
performance_winner_claim=0
remaining_hot_owner=map_hash_lookup_boundary
pilot_result=metadata_consumer_reachability_only

close_pilot_helper_extension=1
next_design_owner=local_fastpath_eligibility
next_task=LOCAL-FASTPATH-ELIGIBILITY-SSOT-001

product_mapbox_storage_changed=0
product_hasher_swap=0
sidecar_storage=0
mirbuilder_map_storage_ownership=0
benchmark_name_branch=0
helper_name_inference=0
summary=ok
```

## Decision

The LocalI64Map get pilot is closed as a reachability keeper, not as a
performance keeper.

The row proved that call-site metadata can drive a backend-local helper choice:
the hot loop reached `nyash.map.local_i64_get_hi`. The helper still delegates to
the existing scalar-load path, and perf remains dominated by
`MapBox::get_scalar_i64_key_domain` / `BuildHasher::hash_one`.

Do not extend this family by adding more helper aliases. The next optimization
must select a storage / publication / fast-path eligibility owner instead of
renaming or re-routing the same product `MapBox` hash lookup boundary.

## Next Direction

Move from map-specific fallback evidence to a general local-first fast-path
decision boundary:

```text
Observation
  -> Eligibility Decision
  -> LocalFastPathFact
  -> Backend consumer
```

The next SSOT must preserve this split:

```text
fallback evidence:
  report only
  backend-consumable=0

LocalFastPathFact:
  backend-consumable fast-path permission
  requires unpublished local state
  requires RoutePlan + ObjectStoragePlan
```

## Stop Lines

- no Hako-vs-C winner claim
- no product MapBox storage change
- no product hasher swap
- no sidecar storage
- no MIRBuilder map storage ownership
- no helper alias extension as optimization
- no benchmark-name / helper-name / variable-name special case
