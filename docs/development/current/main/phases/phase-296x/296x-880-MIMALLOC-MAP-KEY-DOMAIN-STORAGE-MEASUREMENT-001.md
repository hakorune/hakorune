# 296x-880 MIMALLOC-MAP-KEY-DOMAIN-STORAGE-MEASUREMENT-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-mimalloc-map-key-domain-storage-measurement-v0
source_evidence=296x-879
row_kind=measurement
target_front=kilo_leaf_map_get_dynamic_covered_i64

measurement_command=bash tools/perf/build_perf_release.sh && bash tools/perf/bench_micro_aot_asm.sh kilo_leaf_map_get_dynamic_covered_i64 ny_main 3
top_symbol_1=core::hash::BuildHasher::hash_one
top_symbol_1_percent=39.73
top_symbol_2=nyash.map.scalar_load_hi
top_symbol_2_percent=31.14
top_symbol_3=<i64 as alloc::string::SpecToString>::spec_to_string
top_symbol_3_percent=23.81
mapbox_get_scalar_i64_key_str_percent=0.88

storage_key_domain_reached=1
scalar_helper_still_stringifies_i64_key=1
selected_owner=scalar_helper_key_domain_consumer
selected_owner_confidence=high

product_default_changed=0
winner_claim=0
summary=ok
```

## Interpretation

Changing `MapBox` storage to `MapKeyDomain` successfully moved the storage
shape, but the active scalar helper still builds an i64 key text before calling
the string-key helper seam.

The top symbols after storage implementation are:

```text
39.73% core::hash::BuildHasher::hash_one
31.14% nyash.map.scalar_load_hi
23.81% <i64 as alloc::string::SpecToString>::spec_to_string
 0.88% MapBox::get_scalar_i64_key_str
```

This means the next owner is not public MapBox semantics and not MIRBuilder.
The next owner is the scalar helper consumer boundary:

```text
current:
  i64 -> decimal text -> MapKeyDomain::from_text -> HashMap lookup

next design:
  i64 -> MapKeyDomain::from_i64 -> HashMap lookup
```

## Stop Lines

- do not change public `MapBox.get/set/has/delete` semantics
- do not add sidecar storage
- do not change the hasher before direct key-domain consumption is tested
- do not alter MIRBuilder or route proof from measurement evidence alone
- do not claim a winner from this measurement row

## Next

```text
selected_next=MIMALLOC-MAP-SCALAR-LOAD-I64-KEY-DOMAIN-CONSUMER-DESIGN-001
```

Design a narrow raw helper seam that lets scalar helper routes consume
`MapKeyDomain::from_i64` without changing public MapBox semantics.
