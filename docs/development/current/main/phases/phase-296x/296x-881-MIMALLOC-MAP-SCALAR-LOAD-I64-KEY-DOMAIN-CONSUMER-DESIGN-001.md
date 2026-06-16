# 296x-881 MIMALLOC-MAP-SCALAR-LOAD-I64-KEY-DOMAIN-CONSUMER-DESIGN-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-mimalloc-map-scalar-load-i64-key-domain-consumer-design-v0
source_evidence=296x-880
row_kind=design
target_front=kilo_leaf_map_get_dynamic_covered_i64

selected_owner=scalar_helper_key_domain_consumer
selected_shape=scalar_load_hi_uses_i64_domain_helper

new_raw_helper=MapBox::get_scalar_i64_key_i64
new_kernel_helper=map_scalar_load_i64
scalar_load_hi_uses_map_key_text_from_i64=0
scalar_load_hi_uses_map_key_string_from_i64=0
slot_load_hi_unchanged=1
slot_load_hh_unchanged=1
public_mapbox_semantics_changed=0
sidecar_storage_enabled=0
hashmap_hasher_swap_enabled=0
mirbuilder_changed=0
route_proof_changed=0
winner_claim=0
summary=ok
```

## Design

Now that `MapBox` storage is keyed by `MapKeyDomain`, scalar no-publication
loads should not rebuild decimal key text. The narrow seam is:

```text
nyash.map.scalar_load_hi(handle, key_i64)
  -> map_scalar_load_i64(handle, key_i64)
  -> MapBox::get_scalar_i64_key_i64(key_i64)
  -> MapKeyDomain::from_i64(key_i64)
  -> HashMap<MapKeyDomain, ...> lookup
```

The public string-key route remains available for dynamic or text-key surfaces:

```text
slot_load_hi:
  i64 -> String -> public/text-key helper

slot_load_hh:
  any -> String -> public/text-key helper

scalar_load_hi:
  i64 -> MapKeyDomain::from_i64 -> scalar lookup
```

## Not Included

- no public `MapBox.get/set/has/delete` semantic change
- no sidecar storage
- no hasher swap
- no MIRBuilder change
- no route proof change
- no broad helper-name special casing

## Next

```text
selected_next=MIMALLOC-MAP-SCALAR-LOAD-I64-KEY-DOMAIN-CONSUMER-IMPLEMENTATION-001
```

Implement only the helper seam above, then measure before choosing any hasher or
storage-substrate follow-up.
