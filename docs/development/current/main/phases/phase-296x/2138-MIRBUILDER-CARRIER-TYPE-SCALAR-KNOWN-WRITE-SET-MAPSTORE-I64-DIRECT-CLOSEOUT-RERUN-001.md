# 2138 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-DIRECT-CLOSEOUT-RERUN-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-DIRECT-CLOSEOUT-RERUN-001
```

## Purpose

Rerun the adopted MapStoreI64 typed direct closeout contract basis and
materialize the scoped `SetSurfacePolicy / MapStoreI64` direct closeout.

This closes only the typed scalar Set scoped surface. It does not include
`MapStoreAny`, does not open the Any write boundary, does not close
`WriteScalarI64Routes`, and does not close `ScalarKnownTransportAxis`.

## Materialized Contract

```text
contract_id = WriteSetMapStoreI64TypedDirectCloseoutContract
surface_id = WriteScalarI64Routes
subsurface_id = SetSurfacePolicy/MapStoreI64
routes = MapStoreI64
proof_or_policy_source = SetSurfacePolicy, TypedScalarWriteBeforeAnyWrite
core_method_op = MapSet
core_method_lowering_tier = ColdFallback
result_class = NoneResult
return_shape = None
value_demand = WriteAny
write_value_boundary = ScalarI64
publication_policy = NonePublication
effect_class = mutate
mutation_class = MutatesReceiverOrContainer
runtime_mutation_authority = false
publication_execution = false
mapstore_any_included = false
any_write_boundary_opened = false
```

## Result

```text
write_set_mapstore_i64_direct_closeout_materialized = 1
accepted_scoped_closeout_count = 6
remaining_write_scoped_surface_count = 1
remaining_write_scoped_surface = SetSurfacePolicy/MapStoreAny
mapstore_any_deferred = 1
any_write_boundary_opened = 0

write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0
runtime_mutation_authority = 0
publication_execution = 0
source_selfhost_claim = 0

decision:
  SelectWriteSetMapStoreAnyBoundarySelection

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-BOUNDARY-SELECTION-001
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_set_mapstore_i64_direct_closeout_rerun_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-write-set-mapstore-i64-direct-closeout-rerun-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_scalar_known_write_set_mapstore_i64_direct_closeout_rerun.py
```

## Non-Claims

```text
any_write_boundary_opened = 0
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
