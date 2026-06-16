# 296x-904 MAP-HASH-OWNER-REFRESH-AFTER-LOCAL-FASTPATH-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-map-hash-owner-refresh-after-local-fastpath-v0
source_evidence=296x-903
row_kind=owner_refresh
target_front=kilo_leaf_map_get_dynamic_covered_i64

local_fastpath_fact_reached=1
ny_main_loop_helper=nyash.map.local_i64_get_hi
slot_load_hh_loop_boundary_removed=1
post_loop_slot_load_hh_allowed=1

remaining_hot_owner=map_key_domain_hash_lookup_boundary
mapbox_get_scalar_i64_key_domain_pct=70.20
build_hasher_hash_one_pct=22.41
canonical_i64_hot_lookup_visible=1
text_key_hot_lookup_visible=0
mixed_key_hot_lookup_visible=0

product_hasher_policy=std_default
product_hasher_swap_allowed=0
product_mapbox_i64_only_allowed=0
sidecar_storage_allowed=0
mirbuilder_map_storage_ownership=0
implementation_allowed=0

selected_next=LOCAL-I64-MAP-STORAGE-REALIZATION-DESIGN-001
summary=ok
```

## Reading

`296x-903` proves the backend can consume a positive `LocalFastPathFact` and
route the hot loop to:

```text
nyash.map.local_i64_get_hi
```

That removes the old public slot-load boundary from the loop, but it does not
remove the product `MapBox` storage boundary. The remaining loop still reaches:

```text
MapBox::get_scalar_i64_key_domain
  -> HashMap<MapKeyDomain, Box<dyn NyashBox>>::get
  -> BuildHasher::hash_one
```

The current evidence is not a reason to mutate product `MapBox` hasher policy.
It is evidence that the local-first row has only changed the call boundary, not
the storage representation.

## Decision

The next row should design a real local i64 map storage realization for the
exact-AOT/local-first path:

```text
before publication:
  LocalI64Map storage may use an i64-keyed representation

at publication:
  materialize Product MapBox semantics

after publication:
  product-compatible MapBox route remains
```

This row does not implement that storage. It only selects the owner family after
the `LocalFastPathFact` reachability success.

## Stop Lines

- do not swap the product `HashMap` hasher from this evidence
- do not change product `MapBox` storage to i64-only
- do not add sidecar storage to product `MapBox`
- do not move map storage decisions into MIRBuilder
- do not use benchmark/helper-name inference
- do not claim performance winner from 296x-903

## Validation

```bash
bash tools/checks/k2_wide_phase296x_map_hash_owner_refresh_after_local_fastpath_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
