# 296x-868 MIMALLOC-MAP-SCALAR-LOAD-I64-BORROWED-LOOKUP-DESIGN-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-mimalloc-map-scalar-load-i64-borrowed-lookup-design-v0
source_evidence=296x-867
row_kind=design
target_front=kilo_leaf_map_get_dynamic_covered_i64

selected_shape=scalar_helper_borrowed_lookup
selected_helper=nyash.map.scalar_load_hi
selected_owner=MapBox_no_publication_scalar_read
selected_next=MIMALLOC-MAP-SCALAR-LOAD-I64-BORROWED-LOOKUP-IMPLEMENTATION-001

new_mapbox_helper=get_scalar_i64_key_str
new_key_text_helper=map_key_text_from_i64
helper_scope=scalar_load_hi_internal_only

scalar_helper_publication_enabled=0
visible_read_clone_on_scalar_path=0
i64_key_heap_string_on_scalar_path=0

mapbox_storage_change_enabled=0
i64_sidecar_storage_enabled=0
slot_load_hi_changed=0
slot_load_hh_changed=0
mapbox_public_get_contract_changed=0
mapbox_public_set_contract_changed=0
runtime_data_get_route_change_enabled=0
product_default_changed=0
winner_claim=0
implementation_started=0
summary=ok
```

## Design

The selected implementation shape is deliberately smaller than a MapBox storage
redesign:

```text
nyash.map.scalar_load_hi
  -> map_key_text_from_i64(key_i64)
  -> MapBox::get_scalar_i64_key_str(key_text.as_str())
  -> read scalar under the existing RwLock
  -> return i64 directly
```

This avoids two costs that 296x-866 exposed:

```text
1. heap String allocation from key_i64.to_string()
2. visible-read clone/share when the caller only needs an i64 scalar
```

It intentionally does not remove the current `HashMap<String, ...>` hashing
cost. That belongs to a later storage or key-domain row after the public
stringified-key alias contract is designed.

## Semantics

`MapBox` public semantics remain unchanged:

```text
MapBox storage key domain = String
MapBox public keys are stringified
MapBox.get / set / slot_load_hh / slot_load_hi keep their visible materialization behavior
```

The new borrowed scalar helper is a no-publication internal read surface. It may
read an `IntegerBox` or `BoolBox` value under the map read lock and return a
scalar. Missing or non-scalar values keep the existing scalar helper behavior:
return `0`.

## Stop Lines

- do not change `MapBox` storage representation
- do not add an i64 sidecar
- do not change public key alias semantics
- do not change public `MapBox.get` / `MapBox.set`
- do not change `slot_load_hi` or `slot_load_hh`
- do not route unproven get calls to scalar load
- do not claim a winner in this design row

