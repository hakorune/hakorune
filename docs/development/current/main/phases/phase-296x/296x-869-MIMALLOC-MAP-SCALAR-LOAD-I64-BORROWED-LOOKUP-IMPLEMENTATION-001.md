# 296x-869 MIMALLOC-MAP-SCALAR-LOAD-I64-BORROWED-LOOKUP-IMPLEMENTATION-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-mimalloc-map-scalar-load-i64-borrowed-lookup-implementation-v0
source_evidence=296x-868
row_kind=implementation
target_front=kilo_leaf_map_get_dynamic_covered_i64

implemented_shape=scalar_helper_borrowed_lookup
implemented_helper=nyash.map.scalar_load_hi
implemented_key_text_helper=map_key_text_from_i64
implemented_mapbox_helper=get_scalar_i64_key_str

scalar_load_hi_uses_key_text=1
scalar_load_hi_uses_borrowed_scalar_read=1
scalar_load_hi_uses_map_key_string_from_i64=0
scalar_load_hi_uses_visible_read_clone=0

slot_load_hi_changed=0
slot_load_hh_changed=0
map_key_string_from_i64_kept=1
mapbox_get_opt_key_str_kept=1
mapbox_clone_for_visible_read_kept=1
mapbox_storage_change_enabled=0
i64_sidecar_storage_enabled=0
mapbox_public_get_contract_changed=0
mapbox_public_set_contract_changed=0
runtime_data_get_route_change_enabled=0
product_default_changed=0
winner_claim=0
selected_next=MIMALLOC-MAP-SCALAR-LOAD-I64-BORROWED-LOOKUP-VALIDATION-001
summary=ok
```

## Implementation

This row changes only the scalar helper internals:

```text
nyash.map.scalar_load_hi
  old:
    map_key_string_from_i64 -> String
    map_slot_load_str_with -> MapBox::get_opt_key_str -> visible clone/share

  new:
    map_key_text_from_i64 -> stack i64 key text
    map_scalar_load_i64_str -> MapBox::get_scalar_i64_key_str -> borrowed scalar read
```

Public MapBox materialization paths are intentionally kept:

```text
nyash.map.slot_load_hi
nyash.map.slot_load_hh
MapBox::get_opt_key_str
MapBox::clone_for_visible_read
```

## Stop Lines

- do not change `MapBox` storage representation
- do not add an i64 sidecar
- do not change public key alias semantics
- do not change public `MapBox.get` / `MapBox.set`
- do not change `slot_load_hi` or `slot_load_hh`
- do not claim a performance winner before validation / measurement

