# 296x-887 MAP-HASH-OWNER-INVENTORY-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-map-hash-owner-inventory-v0
source_evidence=296x-886
row_kind=inventory
target_front=kilo_leaf_map_get_dynamic_covered_i64

remaining_hot_owner=map_hash_lookup_boundary
mapbox_get_scalar_i64_key_domain_pct=64.18
build_hasher_hash_one_pct=31.43
canonical_i64_hot_lookup_visible=1
text_key_hot_lookup_visible=0
mixed_key_hot_lookup_visible=0

product_hasher_policy=std_default
product_hasher_swap_allowed=0
product_mapbox_i64_only_allowed=0
sidecar_storage_allowed=0
implementation_allowed=0

selected_next=LOCAL-I64-MAP-FRONT-SELECTION-001
summary=ok
```

## Inventory

The post-keeper hot loop is a canonical i64 lookup:

```text
ny_main
  -> nyash.map.scalar_load_hi(handle, key_i64)
  -> map_scalar_load_i64(handle, key_i64)
  -> MapBox::get_scalar_i64_key_i64(key_i64)
  -> MapKeyDomain::from_i64(key_i64)
  -> HashMap<MapKeyDomain, ...> lookup
```

The current front does not show text-key lookup in the hot loop:

```text
canonical_i64_hot_lookup_visible=1
text_key_hot_lookup_visible=0
mixed_key_hot_lookup_visible=0
```

## Decision

Do not optimize the product hasher directly from this evidence.

The next useful question is whether this front can be represented as a
local-first i64 map before publication:

```text
LocalI64KeyMap candidate?
  map is unpublished in the hot loop
  all hot-loop writes/reads are i64 or proven canonical i64 text
  public MapBox materialization is only needed at publication
```

If no such local-front candidate exists, close this family and return to fresh
owner selection.

## Stop Lines

- no product hasher swap
- no product MapBox i64-only storage
- no sidecar storage
- no MIRBuilder map storage ownership
- no helper-name / benchmark-name special case
- no implementation from inventory evidence
