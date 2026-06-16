# 296x-885 MIMALLOC-MAP-GET-SCALAR-KEYDOMAIN-CLOSEOUT-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-map-get-scalar-keydomain-closeout-v0
source_evidence=296x-884
row_kind=closeout
target_front=kilo_leaf_map_get_dynamic_covered_i64

map_key_domain_storage_enabled=1
public_semantics_preserved=1
int_string_alias_preserved=1
text_noncanonical_key_separate=1
keys_values_json_public_text=1

spec_to_string_hot_path_removed=1
scalar_load_hi_pct_before=31.14
scalar_load_hi_pct_after=3.30
spec_to_string_pct_before=23.81
spec_to_string_pct_after=0
cycles_before=1180142598
cycles_after=484694805
keeper_claim=1

remaining_hot_owner=map_hash_lookup_boundary
remaining_owner_requires_storage_policy=1
implementation_allowed=0
next_task=MAP-STORAGE-POLICY-SSOT-001
summary=ok
```

## Closeout Decision

This lane is closed as a keeper:

```text
i64 text conversion:
  removed from scalar_load_hi hot path

MapBox public semantics:
  preserved through MapKeyDomain

performance:
  cycles reduced by roughly 59%
```

The remaining hot owner is not part of the scalar helper seam:

```text
MapBox::get_scalar_i64_key_domain
core::hash::BuildHasher::hash_one
```

That owner belongs to Map storage policy / local-first representation design.

## Not Continued In This Lane

```text
hasher_swap_enabled=0
typed_i64_product_map_enabled=0
i64_sidecar_storage_enabled=0
map_storage_substrate_implementation_enabled=0
mirbuilder_map_storage_owner_enabled=0
```

## Next Lane

Open a design-only storage policy row:

```text
MAP-STORAGE-POLICY-SSOT-001
```

It must separate product MapBox storage from exact-AOT / local-first Map
representations before any new implementation row.
