# 3362 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SCALAR-I64-ROUTES-CLOSEOUT-BASIS-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SCALAR-I64-ROUTES-CLOSEOUT-BASIS-001
```

## Purpose

Collect the scoped WriteScalarI64Routes closeout evidence after MapStoreAny
materialized its scoped direct closeout.

This is a basis-only review card. It does not close `WriteScalarI64Routes`.
It records the three scoped direct closeouts that remain valid, and it records
that `DeleteSurfacePolicy / MapDeleteAny` was retired as an unconnected `.hako`
mirror while the live Rust `MapDeleteAny` route was preserved.

## Evidence

```text
PushSurfacePolicy:
  direct closeout materialized = 1
  route = ArrayAppendAny

SetSurfacePolicy / MapStoreI64:
  direct closeout materialized = 1
  route = MapStoreI64
  write_value_boundary = ScalarI64

SetSurfacePolicy / MapStoreAny:
  direct closeout materialized = 1
  route = MapStoreAny
  write_value_boundary = Any
  any_write_boundary_declared = 1
  any_write_boundary_opened = 0

DeleteSurfacePolicy / MapDeleteAny:
  hako mirror retired = 1
  lifecycle artifacts deleted = 1
  live Rust route preserved = 1
  direct closeout materialized = 0
```

## Result

```text
write_scalar_i64_routes_closeout_basis = 1
scoped_direct_closeout_contract_count = 3
delete_surface_hako_mirror_retired = 1
rust_map_delete_route_preserved = 1
write_scalar_i64_routes_closeout_ready_for_rerun = 1

delete_surface_direct_closeout_materialized = 0
write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0
runtime_mutation_authority = 0
publication_execution = 0
source_selfhost_claim = 0

decision:
  SelectWriteScalarI64RoutesCloseoutRerun

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SCALAR-I64-ROUTES-CLOSEOUT-RERUN-001
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_scalar_i64_routes_closeout_basis_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-write-scalar-i64-routes-closeout-basis-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_scalar_known_write_scalar_i64_routes_closeout_basis.py
```

## Non-Claims

```text
delete_surface_direct_closeout_materialized = 0
write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0
source_selfhost_claim = 0
hako_generation = 0
new_route_authority = 0
behavior_change = 0
runtime_mutation_authority = 0
publication_execution = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
native_seed_materialization = 0
new_python_semantic_projector = 0
manual_axis_selection = 0
manual_carrier_selection = 0
manual_subsurface_selection = 0
row_count_as_proof = 0
route_count_as_proof = 0
source_path_as_authority = 0
owner_name_as_proof = 0
route_membership_alone_as_proof = 0
```
