# 296x-867 MIMALLOC-MAP-SCALAR-LOAD-I64-KEY-STORAGE-OWNER-SELECTION-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-mimalloc-map-scalar-load-i64-key-storage-owner-selection-v0
source_evidence=296x-866
row_kind=owner_selection
target_front=kilo_leaf_map_get_dynamic_covered_i64

route_proof_status=closed
route_proof_next_owner=none
compiler_route_next_owner=none

current_loop_helper=nyash.map.scalar_load_hi
current_i64_key_codec=map_key_string_from_i64
current_i64_key_codec_allocates_string=1
current_scalar_load_uses_visible_read_clone=1
current_map_storage_key_domain=String
current_map_storage_value_domain=Box<dyn NyashBox>

map_public_semantics_key_domain=stringified_key
i64_string_key_alias_semantics_preserved=1

selected_owner=map_scalar_no_publication_borrowed_lookup
selected_owner_scope=scalar_load_hi_internal
selected_first_slice=borrowed_scalar_read_plus_i64_key_text
selected_next=MIMALLOC-MAP-SCALAR-LOAD-I64-BORROWED-LOOKUP-DESIGN-001

no_publication_scalar_read_owner=MapBox
no_publication_scalar_read_may_avoid_visible_clone=1
i64_key_text_may_avoid_heap_string=1
hashmap_string_hash_owner_remains=1

i64_sidecar_storage_selected=0
mapbox_storage_change_enabled=0
mapbox_public_get_contract_changed=0
mapbox_public_set_contract_changed=0
runtime_data_get_route_change_enabled=0
product_default_changed=0
winner_claim=0
implementation_started=0
summary=ok
```

## Evidence

296x-866 proved that the route now reaches AOT and the loop calls
`nyash.map.scalar_load_hi`. The remaining hot symbols are inside the scalar
helper and current MapBox public storage path:

```text
<i64 as alloc::string::SpecToString>::spec_to_string
core::hash::BuildHasher::hash_one
nyash_rust::boxes::map_box::MapBox::get_opt_key_str
MapBox::share_box
nyash.map.scalar_load_hi
```

Current code path:

```text
nyash.map.scalar_load_hi
  -> map_key_string_from_i64(key_i64)
  -> key_i64.to_string()
  -> map_slot_load_str_with(handle, &key_str, ...)
  -> MapBox::get_opt_key_str(&str)
  -> MapBox::clone_for_visible_read(...)
```

`MapBox` storage is currently:

```text
HashMap<String, Box<dyn NyashBox>>
```

That means public key semantics are stringified-key semantics. `map.set(1, v)`
and `map.set("1", v)` must continue to address the same public key namespace.

## Decision

Do not select an i64 sidecar storage implementation from this evidence. A raw
i64 sidecar would be tempting, but it risks splitting the public stringified-key
namespace unless a full alias contract is designed first.

The next owner is the narrower scalar-helper internal path:

```text
map_scalar_no_publication_borrowed_lookup
```

This first slice may remove two current costs without changing public MapBox
semantics:

```text
1. avoid heap String allocation for i64 key text in scalar_load_hi
2. avoid visible-read clone/share when scalar_load_hi only needs an i64 result
```

The remaining `HashMap<String, ...>` hashing cost is expected to remain after
this first slice.

## Stop Lines

- do not change `MapBox` storage representation in this row
- do not introduce an i64 sidecar without a stringified-key alias contract
- do not change public `MapBox.get` / `MapBox.set` semantics
- do not change mixed `RuntimeDataBox.get` route policy
- do not infer from benchmark names or helper symbols
- do not claim a winner from owner selection

