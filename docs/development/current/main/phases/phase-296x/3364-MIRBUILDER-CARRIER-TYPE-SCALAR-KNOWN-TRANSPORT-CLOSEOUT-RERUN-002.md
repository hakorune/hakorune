# 3364 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-TRANSPORT-CLOSEOUT-RERUN-002

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-TRANSPORT-CLOSEOUT-RERUN-002
```

## Purpose

Rerun ScalarKnown transport closeout after `WriteScalarI64Routes` materialized
its scoped closeout.

This card closes the scoped `ScalarKnownTransportAxis` evidence set. It also
refreshes the Rust ScalarKnown contract boundary so Collection and Write no
longer remain candidate surfaces after their scoped closeouts.

## Accepted Surfaces

```text
MapLoadScalarI64Routes:
  MapLoadScalarI64ScalarKnownTypedDirectCloseoutContract

StringScalarI64Routes:
  StringSearchScalarI64TypedDirectCloseoutContract

CollectionScalarI64Routes:
  CollectionLenScalarI64TypedDirectCloseoutContract

WriteScalarI64Routes:
  WriteScalarI64RoutesScopedCloseout
```

## Result

```text
scalar_known_transport_axis_closeout = 1
accepted_scalar_known_surface_count = 4
uncovered_scalar_known_surface_count = 0
write_scalar_i64_routes_closeout = 1
rust_boundary_status_refreshed = 1

fastpath_connected_closeout = 0
hako_runtime_route_authority = 0
rust_fastpath_rewired = 0
source_selfhost_claim = 0

decision:
  SelectFastpathConnectedCloseoutBasis

selected_next_card:
  MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-BASIS-001
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_carrier_type_scalar_known_transport_closeout_rerun_002_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-transport-closeout-rerun-002-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_scalar_known_transport_closeout_rerun_002.py
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
