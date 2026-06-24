# 296x-889 LOCAL-I64-MAP-STORAGE-SHADOW-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-i64-map-storage-shadow-v0
source_evidence=296x-888
row_kind=passive_shadow
target_front=kilo_leaf_map_get_dynamic_covered_i64

map_repr_shadow_kind=local_i64_key_map_shadow
map_repr_shadow_owner=src/mir/map_repr_plan.rs
map_repr_shadow_source=generic_method_routes
local_i64_key_map_shadow_requires_i64_set=1
local_i64_key_map_shadow_requires_scalar_i64_get=1
local_i64_key_map_shadow_rejects_disallowed_route=1
fixture_shadow_route_count=4

backend_lowering_enabled=0
product_mapbox_storage_changed=0
product_hasher_swap_allowed=0
sidecar_storage_allowed=0
mirbuilder_map_storage_ownership=0
implementation_allowed=0
next_task=LOCAL-I64-MAP-GET-PILOT-GUARD-SURFACE-001
summary=ok
```

## Implementation

`src/mir/map_repr_plan.rs` now emits passive
`map_repr.local_i64_key_map_shadow` rows when a MapBox receiver has both:

```text
i64-key set routes
scalar i64 get routes
```

and no disallowed Map route in the same receiver family.

The existing `GenericHashRuntime` rows are still emitted. The shadow row is
metadata only; it is not a backend lowering instruction.

## Tests

```bash
cargo test --lib map_repr_plan -- --nocapture
```

The fixture builds one receiver with three i64-key sets and one covered dynamic
scalar i64 get. The shadow route count is four, and the scalar get row is
present.

## Stop Lines

- no backend lowering from shadow metadata
- no product MapBox i64-only storage
- no product hasher swap
- no sidecar storage
- no MIRBuilder map storage ownership
- no benchmark-name / helper-name / variable-name special case
