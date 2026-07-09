# 3376 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-INVENTORY-RERUN-004

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-INVENTORY-RERUN-004
```

## Purpose

Rerun the ScalarKnown fastpath-connected closeout inventory after
`MapLoadScalarI64Routes` started shadow-consuming a checked-in generated typed
`.hako` artifact from the live Rust fast path.

This card is inventory/rerun only. It records the newly connected MapLoad read
surface, keeps fastpath-connected closeout unclaimed, and selects the next
generated typed artifact basis for `StringScalarI64Routes`.

## Inventory Result

```text
connected_surface_row_count = 4

connected rows:
  WriteScalarI64Routes / SetSurfacePolicy / MapStoreI64
  WriteScalarI64Routes / SetSurfacePolicy / MapStoreAny
  WriteScalarI64Routes / PushSurfacePolicy / ArrayAppendAny
  MapLoadScalarI64Routes / MapLoadScalarI64

known_unconnected_surface_row_count = 2

remaining:
  StringScalarI64Routes
  CollectionScalarI64Routes
```

## Selection

```text
selection_rule:
  ReadSurfaceGeneratedArtifactMinimalityAfterMapLoadV1

selected_surface:
  StringScalarI64Routes

selected_next_card:
  MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-BASIS-STRING-SCALAR-I64-001
```

`StringScalarI64Routes` is selected after MapLoad because it keeps one receiver
domain, `ScalarI64` result, `NoPublication`, and read effect. Collection remains
after String because it mixes receiver/domain families.

Forbidden proof sources remain forbidden:

```text
route_count_as_proof = 0
manual_surface_selection = 0
owner_name_as_proof = 0
source_path_as_authority = 0
route_membership_alone_as_proof = 0
```

## Claims

```text
fastpath_connected_closeout_inventory_rerun_004 = 1
connected_surface_row_count = 4
known_unconnected_surface_row_count = 2
read_mapload_connection_complete = 1
read_surface_connection_complete = 0
selection_eligible_candidate_count = 1
```

## Non-Claims

```text
fastpath_connected_closeout = 0
hako_runtime_route_authority = 0
rust_fastpath_rewired = 0
route_selection_authority_switch = 0
backend_lowering_authority = 0
runtime_mutation_authority = 0
publication_execution = 0
build_rs_hako_compiler_invocation = 0
live_hako_authority = 0
caller_orientation_runtime_path = 0
new_backend_route = 0
new_abi = 0
runtime_fallback = 0
source_selfhost_claim = 0
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_scalar_known_fastpath_connected_closeout_inventory_rerun_004_guard.sh
```
