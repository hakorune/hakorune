# 296x-882 MIMALLOC-MAP-SCALAR-LOAD-I64-KEY-DOMAIN-CONSUMER-IMPLEMENTATION-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-mimalloc-map-scalar-load-i64-key-domain-consumer-implementation-v0
source_evidence=296x-881
row_kind=implementation
target_front=kilo_leaf_map_get_dynamic_covered_i64

implemented_raw_helper=MapBox::get_scalar_i64_key_i64
implemented_kernel_helper=map_scalar_load_i64
scalar_load_hi_uses_map_scalar_load_i64=1
scalar_load_hi_uses_map_key_text_from_i64=0
scalar_load_hi_uses_map_key_string_from_i64=0
map_key_text_from_i64_removed=1
i64_key_text_struct_removed=1

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

## Implementation

The scalar helper now consumes the i64 key-domain directly:

```text
nyash.map.scalar_load_hi
  -> map_scalar_load_i64(handle, key_i64)
  -> MapBox::get_scalar_i64_key_i64(key_i64)
  -> MapKeyDomain::from_i64(key_i64)
```

The public/text helper routes are intentionally unchanged:

```text
nyash.map.slot_load_hi
nyash.map.slot_load_hh
```

The obsolete borrowed decimal text helper is removed because it became a dead
middle layer after storage moved to `MapKeyDomain`.

## Validation

```bash
cargo fmt --check
cargo check --release --bin hakorune
cargo test --lib test_key_domain_i64_text_alias -- --nocapture
```

## Next

```text
selected_next=MIMALLOC-MAP-SCALAR-LOAD-I64-KEY-DOMAIN-CONSUMER-MEASUREMENT-001
```

Measure before selecting any hasher/storage-substrate follow-up.
