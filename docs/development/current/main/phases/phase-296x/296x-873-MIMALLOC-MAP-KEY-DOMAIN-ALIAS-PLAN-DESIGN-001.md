# 296x-873 MIMALLOC-MAP-KEY-DOMAIN-ALIAS-PLAN-DESIGN-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-mimalloc-map-key-domain-alias-plan-design-v0
source_evidence=296x-872
row_kind=design
target_front=kilo_leaf_map_get_dynamic_covered_i64

selected_shape=normalized_map_key_domain
selected_owner=MapBox_key_domain
selected_next=MIMALLOC-MAP-KEY-DOMAIN-VOCABULARY-001

public_key_semantics=stringified_key_namespace
canonical_i64_text_alias_enabled=1
noncanonical_numeric_text_preserved_as_text=1
map_keys_public_text_output_required=1

map_key_domain_variants=CanonicalI64,Text
canonical_i64_accepts=0,1,-1,i64_MIN,i64_MAX
canonical_i64_rejects=leading_plus,leading_zero_except_zero,negative_zero,empty,whitespace,overflow

i64_sidecar_storage_selected=0
typed_i64_map_storage_selected=0
hashmap_hasher_swap_selected=0
public_semantics_change_selected=0
mapbox_storage_change_enabled=0
implementation_started=0
winner_claim=0
summary=ok
```

## Decision

Use a normalized key-domain model before any storage implementation:

```text
MapKeyDomain:
  CanonicalI64(i64)
  Text(String)
```

The purpose is to preserve existing public stringified-key semantics while
allowing a later storage row to avoid hashing decimal i64 keys as heap strings.

Alias rule:

```text
map.set(1, value)
map.get("1")
```

must still address the same public key. Therefore canonical decimal text must
normalize to the same domain as the corresponding i64 key.

Non-canonical numeric-looking text remains text:

```text
"01"  -> Text("01")
"+1"  -> Text("+1")
"-0"  -> Text("-0")
" 1"  -> Text(" 1")
```

This keeps current string key behavior for values that are not exactly the
public `i64.to_string()` spelling.

## Why Not Sidecar First

An i64 sidecar without alias handling would silently split the namespace:

```text
i64 key 1  -> sidecar[1]
text key "1" -> string map["1"]
```

That is not a performance implementation; it is a semantic change. Sidecar
storage can be reconsidered only after the key-domain rules above are encoded
as a shared helper and tested.

## First Implementation Slice

The next row must be vocabulary-only:

```text
MIMALLOC-MAP-KEY-DOMAIN-VOCABULARY-001
```

Scope:

```text
add MapKeyDomain vocabulary
add canonical decimal parser/formatter helpers
add unit tests for alias and reject cases
do not change MapBox storage
do not route scalar_load_hi through the new domain yet
```

## Stop Lines

- do not change `MapBox` storage in this design row
- do not add an i64 sidecar before `MapKeyDomain` tests exist
- do not change `keys()` / public text output semantics
- do not swap hashers as a shortcut
- do not change route proof / MIRBuilder / C shim routing from this evidence
- do not claim a performance winner from design

