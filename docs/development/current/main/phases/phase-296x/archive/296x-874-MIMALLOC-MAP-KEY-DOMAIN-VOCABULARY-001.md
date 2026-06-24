# 296x-874 MIMALLOC-MAP-KEY-DOMAIN-VOCABULARY-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-mimalloc-map-key-domain-vocabulary-v0
source_evidence=296x-873
row_kind=implementation
target_front=kilo_leaf_map_get_dynamic_covered_i64

implemented_shape=MapKeyDomain_vocabulary
implemented_module=crates/nyash_kernel/src/plugin/map_key_domain.rs
implemented_variants=CanonicalI64,Text
implemented_from_i64=1
implemented_from_text=1
implemented_public_text=1

canonical_i64_text_alias_tested=1
noncanonical_numeric_text_reject_tested=1
public_text_roundtrip_tested=1

mapbox_storage_change_enabled=0
mapbox_storage_consumes_map_key_domain=0
scalar_load_hi_consumes_map_key_domain=0
i64_sidecar_storage_enabled=0
hashmap_hasher_swap_enabled=0
public_semantics_change_enabled=0
winner_claim=0
selected_next=MIMALLOC-MAP-KEY-DOMAIN-STORAGE-DESIGN-001
summary=ok
```

## Implementation

This row adds vocabulary only:

```rust
enum MapKeyDomain {
    CanonicalI64(i64),
    Text(String),
}
```

The helper preserves public stringified-key alias semantics:

```text
MapKeyDomain::from_i64(1) == MapKeyDomain::from_text("1")
MapKeyDomain::from_i64(1) != MapKeyDomain::from_text("01")
MapKeyDomain::from_i64(0) != MapKeyDomain::from_text("-0")
```

No storage consumer is added in this row.

## Validation

```bash
rustc --edition=2021 --test crates/nyash_kernel/src/plugin/map_key_domain.rs \
  -o /tmp/hakorune_map_key_domain_test
/tmp/hakorune_map_key_domain_test --nocapture
```

Note: full `cargo test -p nyash_kernel ...` currently compiles unrelated
existing lib-test modules with stale include/import errors. This row validates
the standalone vocabulary module directly and keeps the wider crate-test cleanup
out of scope.

## Stop Lines

- do not change `MapBox` storage
- do not route `scalar_load_hi` through `MapKeyDomain` yet
- do not add i64 sidecar storage
- do not change public MapBox key semantics
- do not claim a performance winner from vocabulary
