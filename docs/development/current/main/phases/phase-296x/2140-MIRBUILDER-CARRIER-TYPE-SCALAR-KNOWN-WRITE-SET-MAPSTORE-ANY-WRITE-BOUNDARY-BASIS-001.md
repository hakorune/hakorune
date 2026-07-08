# 2140 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-WRITE-BOUNDARY-BASIS-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-WRITE-BOUNDARY-BASIS-001
```

## Purpose

Define the basis-only Any write boundary for `SetSurfacePolicy / MapStoreAny`.

This card declares the boundary needed before a `MapStoreAny` Rust-oracle
fixture or `.hako` parity pilot. It does not open runtime mutation authority,
does not execute publication, and does not materialize direct closeout.

## Boundary

```text
basis_id = MapStoreAnyWriteBoundaryBasis
route_kind = MapStoreAny
surface_id = WriteScalarI64Routes
subsurface_id = SetSurfacePolicy/MapStoreAny
write_value_boundary = Any
relationship_to_scalar_known = RemainingScopedWriteSurfaceInScalarKnownCloseoutChain
mapstore_i64_already_scoped_closeout = true
runtime_mutation_authority = false
publication_execution = false
any_write_boundary_opened = false
```

## Result

```text
mapstore_any_write_boundary_basis = 1
any_write_boundary_declared = 1
set_surface_policy_remaining = 1
mapstore_i64_already_scoped_closeout = 1
mapstore_any_deferred_until_boundary = 1
basis_only = 1
hako_pilot_required_before_adoption = 1

any_write_boundary_opened = 0
mapstore_any_hako_pilot_selected = 0
mapstore_any_direct_closeout_materialized = 0
write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0
source_selfhost_claim = 0

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-RUST-ORACLE-PARITY-FIXTURE-001
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_set_mapstore_any_write_boundary_basis_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-write-set-mapstore-any-write-boundary-basis-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_scalar_known_write_set_mapstore_any_write_boundary_basis.py
```

## Non-Claims

```text
any_write_boundary_opened = 0
mapstore_any_hako_pilot_selected = 0
mapstore_any_direct_closeout_materialized = 0
write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0
runtime_mutation_authority = 0
publication_execution = 0
source_selfhost_claim = 0
new_route_authority = 0
new_backend_route = 0
new_abi = 0
runtime_fallback = 0
behavior_change = 0
hako_generation = 0
native_seed_materialization = 0
route_count_as_proof = 0
apparent_simplicity_as_proof = 0
manual_subsurface_selection = 0
source_path_as_authority = 0
owner_name_as_proof = 0
route_membership_alone_as_proof = 0
```
