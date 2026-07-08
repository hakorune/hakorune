# 2136 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-POST-ADOPTION-RERUN-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-POST-ADOPTION-RERUN-001
```

## Purpose

Rerun the Write Set scoped surface selector after the narrow MapStoreI64
`.hako` adoption decision.

This card consumes the adopted `SetSurfacePolicy / MapStoreI64` parity pilot
and selects the next basis-only direct closeout contract card for MapStoreI64.
It does not materialize direct closeout and does not open the `MapStoreAny`
Any write boundary.

## Rerun Result

```text
write_set_mapstore_i64_hako_adopted = 1
basis_selection_eligible_surface_count = 1
selected_scoped_surface = SetSurfacePolicy/MapStoreI64
mapstore_any_deferred = 1
any_write_boundary_opened = 0

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-TYPED-DIRECT-CLOSEOUT-CONTRACT-BASIS-001
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_set_mapstore_i64_post_adoption_rerun_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-write-set-mapstore-i64-post-adoption-rerun-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_scalar_known_write_set_mapstore_i64_post_adoption_rerun.py
```

## Non-Claims

```text
mapstore_any_deferred = 1
any_write_boundary_opened = 0
write_set_mapstore_i64_direct_closeout_materialized = 0
write_direct_closeout_materialized = 0
write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0
component_specific_direct_contract_materialized = 0
source_selfhost_claim = 0
new_route_authority = 0
behavior_change = 0
runtime_mutation_authority = 0
publication_execution = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
native_seed_materialization = 0
hako_generation = 0
manual_subsurface_selection = 0
route_count_as_proof = 0
apparent_simplicity_as_proof = 0
accepted_read_contract_similarity_as_proof = 0
```
