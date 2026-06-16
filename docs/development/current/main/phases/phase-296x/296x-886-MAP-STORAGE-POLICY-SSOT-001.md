# 296x-886 MAP-STORAGE-POLICY-SSOT-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-map-storage-policy-ssot-v0
source_evidence=296x-885
row_kind=design_ssot

product_map_storage_owner=MapBox
product_map_storage_shape=HashMap<MapKeyDomain, Box<dyn NyashBox>>
product_map_hasher_policy=std_default
product_map_public_semantics_owner=MapBox

local_map_storage_plan_reserved=1
local_i64_key_map_reserved=1
local_text_key_map_reserved=1
local_canonical_map_reserved=1
published_mapbox_fallback_reserved=1

hasher_swap_enabled=0
typed_i64_product_map_enabled=0
i64_sidecar_storage_enabled=0
map_storage_substrate_implementation_enabled=0
mirbuilder_map_storage_owner_enabled=0
route_proof_changed=0
winner_claim=0
next_task=MAP-HASH-OWNER-INVENTORY-001
summary=ok
```

## Product Storage

Product `MapBox` remains the public semantic owner:

```text
ProductMapStorage:
  HashMap<MapKeyDomain, Box<dyn NyashBox>>
  default Rust HashMap hasher
  public semantics owner
```

Required product semantics:

```text
1 and "1" alias
"01" remains Text
keys / values / JSON expose public_text()
dynamic/public MapBox APIs remain generic
```

Do not make product `MapBox` i64-only.

## Local-First / Exact-AOT Storage Plan

Future exact-AOT/local-first map representations are separate from product
`MapBox` storage:

```text
MapStoragePlan:
  GenericCanonicalMap
  LocalI64KeyMap
  LocalTextKeyMap
  LocalScalarValueMap
  PublishedMapBoxFallback
```

Rules:

```text
LocalI64KeyMap:
  allowed only when the map is unpublished
  all writes are i64 keys or proven canonical i64 text keys
  no dynamic unknown-key get before publication
  no plugin / extern / HostHandle publication before use
  no keys / values / JSON unless the local plan can reproduce public_text()

PublishedMapBoxFallback:
  materializes ProductMapStorage and preserves public semantics
```

## Hasher Policy

Do not change product `MapBox` hasher from perf-only evidence.

```text
product_hasher_policy:
  std_default

exact_local_map_hasher_policy:
  deferred until MapStoragePlan producer / consumer exists
```

## Stop Lines

- no product `MapBox` hasher swap
- no product `MapBox` i64-only storage
- no sidecar storage without a MapStoragePlan producer/consumer
- no map storage decision in MIRBuilder
- no benchmark/helper-name special case
- no implementation from this design row
