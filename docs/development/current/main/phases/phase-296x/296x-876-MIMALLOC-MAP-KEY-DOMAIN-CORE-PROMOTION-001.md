# 296x-876 MIMALLOC-MAP-KEY-DOMAIN-CORE-PROMOTION-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-mimalloc-map-key-domain-core-promotion-v0
source_evidence=296x-875
row_kind=implementation
target_front=kilo_leaf_map_get_dynamic_covered_i64

implemented_shape=core_map_key_domain_vocabulary
implemented_module=src/boxes/map_key_domain.rs
implemented_export=src/boxes/mod.rs
implemented_variants=CanonicalI64,Text
implemented_from_i64=1
implemented_from_text=1
implemented_public_text=1

core_alias_tests_green=1
kernel_duplicate_kept_temporarily=1
kernel_duplicate_consumed_by_mapbox=0

mapbox_storage_change_enabled=0
mapbox_storage_consumes_map_key_domain=0
scalar_load_hi_consumes_map_key_domain=0
i64_sidecar_storage_enabled=0
hashmap_hasher_swap_enabled=0
public_semantics_change_enabled=0
winner_claim=0
selected_next=MIMALLOC-MAP-KEY-DOMAIN-STORAGE-GUARD-SURFACE-001
summary=ok
```

## Implementation

This row promotes the `MapKeyDomain` vocabulary to the layer that owns
`MapBox` storage:

```text
src/boxes/map_key_domain.rs
src/boxes/mod.rs
```

The existing kernel-side prototype vocabulary remains temporarily so earlier
guards and proof rows stay stable. It is not consumed by `MapBox`.

## Validation

```bash
rustc --edition=2021 --test src/boxes/map_key_domain.rs \
  -o /tmp/hakorune_core_map_key_domain_test
/tmp/hakorune_core_map_key_domain_test --nocapture
```

## Stop Lines

- do not change `MapBox` storage
- do not route `scalar_load_hi` through core `MapKeyDomain` yet
- do not remove the kernel prototype vocabulary in this row
- do not add i64 sidecar storage
- do not claim a performance winner from vocabulary promotion

