# 296x-884 MIMALLOC-MAP-SCALAR-LOAD-I64-POST-DOMAIN-CONSUMER-OWNER-SELECTION-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-mimalloc-map-scalar-load-i64-post-domain-consumer-owner-selection-v0
source_evidence=296x-883
row_kind=owner_selection
target_front=kilo_leaf_map_get_dynamic_covered_i64

fresh_top_owner=MapBox::get_scalar_i64_key_domain
fresh_secondary_owner=core::hash::BuildHasher::hash_one
selected_owner=map_key_domain_hash_lookup_policy
selected_owner_confidence=high

implementation_allowed=0
reason=requires_map_storage_policy_design
hasher_swap_allowed=0
typed_i64_storage_allowed=0
sidecar_storage_allowed=0
public_mapbox_semantics_changed=0
mirbuilder_changed=0
route_proof_changed=0
winner_claim=0
selected_next=DESIGN-CONSULT-MAP-KEY-DOMAIN-HASH-LOOKUP-POLICY-001
summary=ok
```

## Decision

The previous row is a keeper. It removes i64 decimal text conversion and moves
the active hot owner to the domain-keyed lookup itself:

```text
MapBox::get_scalar_i64_key_domain = 64.18%
core::hash::BuildHasher::hash_one = 31.43%
```

This is no longer a scalar helper seam. The remaining choices affect the Map
storage policy:

```text
Option A:
  specialize MapKeyDomain hashing / hasher policy

Option B:
  introduce typed i64-keyed MapBox representation

Option C:
  close this front after the keeper and avoid storage-substrate expansion
```

## Stop Line

Do not implement another optimization row from this evidence without a design
decision. In particular:

```text
do not swap HashMap hasher as a drive-by change
do not add i64 sidecar storage
do not introduce typed i64 Map storage without a storage-substrate SSOT
do not special-case benchmark names / helper names
do not move object management into MIRBuilder
```

## Recommended Question

Ask design review which path is cleanest:

```text
After scalar_load_hi switched to MapKeyDomain::from_i64, remaining hot symbols
are MapBox::get_scalar_i64_key_domain and BuildHasher::hash_one. Should we:

A. specialize MapKeyDomain hashing / hasher policy,
B. add typed i64-keyed MapBox representation,
C. close this front after the keeper,
or D. choose another front?

Constraints:
  public Map semantics must stay stable
  no sidecar without SSOT
  no MIRBuilder/object-management shift
  no benchmark/helper-name hardcode
```
