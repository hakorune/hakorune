# 296x-875 MIMALLOC-MAP-KEY-DOMAIN-STORAGE-DESIGN-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-mimalloc-map-key-domain-storage-design-v0
source_evidence=296x-874
row_kind=design
target_front=kilo_leaf_map_get_dynamic_covered_i64

storage_truth_owner=nyash_rust::boxes::MapBox
map_key_domain_final_owner=nyash_rust::boxes::map_key_domain
kernel_map_key_domain_role=prototype_only
selected_next=MIMALLOC-MAP-KEY-DOMAIN-CORE-PROMOTION-001

selected_storage_shape=HashMap<MapKeyDomain, Box<dyn NyashBox>>
selected_public_key_output=MapKeyDomain::public_text
selected_public_alias_rule=CanonicalI64_and_canonical_decimal_Text_alias

kernel_vocabulary_duplicate_allowed_temporarily=1
kernel_vocabulary_duplicate_retire_required=1
mapbox_storage_change_enabled=0
scalar_load_hi_consumes_map_key_domain=0
i64_sidecar_storage_enabled=0
hashmap_hasher_swap_enabled=0
public_semantics_change_enabled=0
implementation_started=0
winner_claim=0
summary=ok
```

## Decision

`MapKeyDomain` must move to the core `nyash-rust::boxes` layer before storage
uses it.

Reason:

```text
MapBox storage truth lives in src/boxes/map_box.rs.
nyash_kernel depends on nyash-rust.
nyash-rust cannot depend on nyash_kernel.
```

The 296x-874 kernel-side vocabulary is useful as a prototype and proof surface,
but it is not the final owner for MapBox storage.

Final intended storage shape:

```rust
HashMap<MapKeyDomain, Box<dyn NyashBox>>
```

Public key output remains text:

```text
MapBox.keys() returns sorted StringBox values from MapKeyDomain::public_text()
MapBox.get / set still normalize through public stringified-key semantics
```

## Next Slice

The next implementation row is not a storage rewrite. It is a core promotion:

```text
MIMALLOC-MAP-KEY-DOMAIN-CORE-PROMOTION-001
```

Scope:

```text
add src/boxes/map_key_domain.rs
export boxes::map_key_domain
move/copy vocabulary tests to core owner
keep nyash_kernel prototype duplicate temporarily
do not change MapBox storage
do not route scalar_load_hi through the core domain yet
```

After that, a separate storage guard-surface row can change `MapBox` storage.

## Stop Lines

- do not change `MapBox` storage in this design row
- do not make `nyash-rust` depend on `nyash_kernel`
- do not route scalar helper through `MapKeyDomain` before core promotion
- do not add sidecar storage
- do not change public key output or alias semantics
- do not claim a performance winner from design

