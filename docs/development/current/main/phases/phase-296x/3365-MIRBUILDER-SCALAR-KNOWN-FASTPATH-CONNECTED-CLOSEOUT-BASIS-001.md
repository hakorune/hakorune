# 3365 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-BASIS-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-BASIS-001
```

## Purpose

Define the basis for ScalarKnown fastpath-connected closeout after the scoped
`ScalarKnownTransportAxis` closeout.

This is basis-only. It records that only `SetSurfacePolicy / MapStoreI64`
currently has a checked-in generated typed `.hako` artifact consumed at the
Rust fast-path decision point. It does not close the full fastpath-connected
surface.

## Basis

```text
required connection:
  checked-in generated typed .hako artifact
  consumed at Rust fast-path decision point as shadow evidence
  Rust route authority retained
  runtime .hako source text parsing forbidden

connected:
  WriteScalarI64Routes / SetSurfacePolicy / MapStoreI64

known unconnected:
  MapLoadScalarI64Routes
  StringScalarI64Routes
  CollectionScalarI64Routes
  WriteScalarI64Routes / PushSurfacePolicy
  WriteScalarI64Routes / SetSurfacePolicy / MapStoreAny
```

## Result

```text
fastpath_connected_closeout_basis = 1
scalar_known_transport_axis_closeout = 1
generated_typed_hako_artifact_shadow_consumed = 1
connected_surface_row_count = 1
known_unconnected_surface_row_count = 5

fastpath_connected_closeout = 0
hako_runtime_route_authority = 0
rust_fastpath_rewired = 0
source_selfhost_claim = 0

decision:
  SelectFastpathConnectedCloseoutInventoryRerun

selected_next_card:
  MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-INVENTORY-RERUN-001
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_scalar_known_fastpath_connected_closeout_basis_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-scalar-known-fastpath-connected-closeout-basis-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_scalar_known_fastpath_connected_closeout_basis.py
```

## Non-Claims

```text
fastpath_connected_closeout = 0
hako_runtime_route_authority = 0
rust_fastpath_rewired = 0
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
