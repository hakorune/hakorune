# 3363 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SCALAR-I64-ROUTES-CLOSEOUT-RERUN-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SCALAR-I64-ROUTES-CLOSEOUT-RERUN-001
```

## Purpose

Rerun the WriteScalarI64Routes closeout review after 3362 collected the scoped
Write surface evidence.

This materializes only the scoped WriteScalarI64Routes closeout. It does not
turn the retired DeleteSurfacePolicy mirror into a `.hako` direct closeout, and
it does not close `ScalarKnownTransportAxis`.

## Materialized Closeout

```text
surface_id = WriteScalarI64Routes
closeout_kind = ScopedWriteSurfaceCloseout

scoped direct closeouts:
  PushSurfacePolicy / ArrayAppendAny
  SetSurfacePolicy / MapStoreI64
  SetSurfacePolicy / MapStoreAny

DeleteSurfacePolicy / MapDeleteAny:
  hako mirror retired = 1
  live Rust route preserved = 1
  direct closeout materialized = 0
  counts as hako direct closeout = false
```

## Result

```text
write_scalar_i64_routes_closeout = 1
write_scalar_i64_routes_scoped_closeout_materialized = 1
scoped_direct_closeout_contract_count = 3
delete_surface_hako_mirror_retired = 1
delete_surface_direct_closeout_materialized = 0
delete_surface_live_rust_route_preserved = 1

scalar_known_transport_axis_closeout = 0
runtime_mutation_authority = 0
publication_execution = 0
source_selfhost_claim = 0

decision:
  SelectScalarKnownTransportCloseoutRerunAfterWriteCloseout

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-TRANSPORT-CLOSEOUT-RERUN-002
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_scalar_i64_routes_closeout_rerun_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-write-scalar-i64-routes-closeout-rerun-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_scalar_known_write_scalar_i64_routes_closeout_rerun.py
```

## Non-Claims

```text
delete_surface_direct_closeout_materialized = 0
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
