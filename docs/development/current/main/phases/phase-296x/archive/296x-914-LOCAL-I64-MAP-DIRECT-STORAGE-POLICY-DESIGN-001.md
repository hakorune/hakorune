# 296x-914 LOCAL-I64-MAP-DIRECT-STORAGE-POLICY-DESIGN-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-i64-map-direct-storage-policy-design-v0
source_evidence=296x-913
row_kind=design
target_front=kilo_leaf_map_get_dynamic_covered_i64

selected_policy=exact_aot_unpublished_local_i64_map_direct_storage
selected_owner=MapStoragePlan
selected_candidate_shape=known_i64_key_set_and_scalar_get
selected_first_representation=closed_world_i64_key_value_table
publication_materialization_required=1

product_mapbox_storage_changed=0
product_hasher_swap=0
sidecar_storage=0
mirbuilder_map_storage_ownership=0
new_runtime_helper_enabled=0
backend_lowering_enabled=0
winner_claim=0

first_allowed_slice=passive_direct_storage_plan_descriptor
next_task=LOCAL-I64-MAP-DIRECT-STORAGE-PLAN-SURFACE-001
summary=ok
```

## Decision

The current hot owner is product MapBox hash lookup reached through the local
i64 helper. The next implementation must not mutate product MapBox or swap its hasher.
Instead, the exact-AOT/local-first path may introduce a passive `MapStoragePlan`
descriptor for unpublished local i64 maps.

The first direct storage shape is deliberately narrow:

```text
all observed writes use i64 keys
all observed hot reads use scalar i64 get
map is unpublished before those reads
no keys/values/toJSON/plugin/extern/return publication before reads
publication materialization remains required
```

## Representation

Initial representation:

```text
closed_world_i64_key_value_table
```

This means the plan may describe a small exact-AOT table of known i64 key/value
pairs for local reads. It does not replace product MapBox storage. At any
publication site, materialization into product MapBox semantics is still
required.

## Stop Lines

- no product `MapBox` storage change
- no product hasher swap
- no sidecar storage
- no MIRBuilder map storage ownership
- no new runtime helper
- no backend lowering in this row
- no helper-name / benchmark-name inference
- no winner claim

## Next

`LOCAL-I64-MAP-DIRECT-STORAGE-PLAN-SURFACE-001` may add passive metadata for the
selected representation. It must not lower differently yet.
