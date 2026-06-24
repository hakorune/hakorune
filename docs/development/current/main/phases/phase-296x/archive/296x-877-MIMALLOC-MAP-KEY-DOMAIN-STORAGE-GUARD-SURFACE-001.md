# 296x-877 MIMALLOC-MAP-KEY-DOMAIN-STORAGE-GUARD-SURFACE-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-mimalloc-map-key-domain-storage-guard-surface-v0
source_evidence=296x-876
row_kind=guard_surface
target_front=kilo_leaf_map_get_dynamic_covered_i64

selected_next=MIMALLOC-MAP-KEY-DOMAIN-STORAGE-IMPLEMENTATION-001

post_storage_shape=HashMap<MapKeyDomain, Box<dyn NyashBox>>
post_mapbox_set_normalizes_key_domain=1
post_mapbox_get_normalizes_key_domain=1
post_mapbox_has_normalizes_key_domain=1
post_mapbox_delete_normalizes_key_domain=1
post_mapbox_keys_uses_public_text=1
post_mapbox_values_order_uses_public_text_sort=1

post_i64_text_alias_test_required=1
post_noncanonical_text_preservation_test_required=1
post_public_keys_text_output_test_required=1

scalar_load_hi_consumes_map_key_domain=0
slot_load_hi_consumes_map_key_domain=0
slot_load_hh_consumes_map_key_domain=0
i64_sidecar_storage_enabled=0
hashmap_hasher_swap_enabled=0
product_default_changed=0
winner_claim=0
implementation_started=0
summary=ok
```

## Post Target

The next row may change `MapBox` storage to:

```rust
HashMap<MapKeyDomain, Box<dyn NyashBox>>
```

The storage implementation must update public MapBox operations as a single
semantic unit:

```text
set/delete/get/has:
  normalize input key through MapKeyDomain

keys/values:
  sort by public_text() so public output remains stable
```

## Required Semantic Fixtures

The implementation row must include tests proving:

```text
map.set(1, value); map.get("1") == value
map.set("01", value); map.get(1) != value
map.keys() returns public text keys
```

## Not Included

This storage row does not yet connect the generated scalar helper path:

```text
nyash.map.scalar_load_hi
nyash.map.slot_load_hi
nyash.map.slot_load_hh
```

Those are separate consumer rows after storage semantics are green.

## Stop Lines

- do not add i64 sidecar storage
- do not swap hashers
- do not change route proof / MIRBuilder / C shim routing
- do not connect scalar helper before MapBox public semantics tests exist
- do not claim a performance winner from guard surface

