# 296x-872 MIMALLOC-MAP-SCALAR-LOAD-I64-HASHMAP-KEY-DOMAIN-OWNER-SELECTION-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-mimalloc-map-scalar-load-i64-hashmap-key-domain-owner-selection-v0
source_evidence=296x-871
row_kind=owner_selection
target_front=kilo_leaf_map_get_dynamic_covered_i64

route_proof_status=closed
borrowed_scalar_lookup_status=closed
compiler_route_next_owner=none

current_remaining_hot_owner=HashMap<String>_key_hashing
current_storage_key_domain=String
current_public_key_semantics=stringified_key_namespace
current_i64_public_aliases_string_key=1

selected_owner=map_key_domain_alias_plan
selected_owner_scope=MapBox_storage_semantics_design
selected_next=MIMALLOC-MAP-KEY-DOMAIN-ALIAS-PLAN-DESIGN-001

i64_sidecar_storage_selected=0
typed_i64_map_storage_selected=0
hashmap_hasher_swap_selected=0
public_semantics_change_selected=0
implementation_started=0
winner_claim=0
summary=ok
```

## Evidence

296x-871 removed the previous scalar helper costs:

```text
SpecToString top symbol removed
MapBox::share_box top symbol removed
MapBox::get_opt_key_str top symbol removed
```

The new top owner is now the remaining string-key lookup path:

```text
53.94% nyash.map.scalar_load_hi
41.72% core::hash::BuildHasher::hash_one
```

This points to key-domain / hashing work, not generic-method routing work.

## Decision

Do not implement storage yet.

`MapBox` currently stores keys in a public stringified namespace:

```text
MapBox storage key domain = String
map.set(1, value) and map.set("1", value) address the same public key
```

Therefore, a direct i64 sidecar is not selected as the next implementation. It
needs an alias plan first, otherwise the fast path may silently split the public
key namespace.

The next row is a design row:

```text
MIMALLOC-MAP-KEY-DOMAIN-ALIAS-PLAN-DESIGN-001
```

It must decide whether the first keeper is:

```text
1. keep String storage and only tune scalar helper/hash seam
2. add an i64 sidecar with explicit stringified-key alias invalidation/mirroring
3. introduce a normalized key domain that preserves existing public semantics
```

## Stop Lines

- do not add i64 sidecar storage in this owner-selection row
- do not change `MapBox` public key semantics
- do not swap hashers as a microbench-only shortcut
- do not change route proof / MIRBuilder / C shim routing from this evidence
- do not claim a performance winner from owner selection

