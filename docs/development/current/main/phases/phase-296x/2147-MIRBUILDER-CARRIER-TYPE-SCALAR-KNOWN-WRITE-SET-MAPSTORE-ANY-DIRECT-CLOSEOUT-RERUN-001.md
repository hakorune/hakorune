# 2147 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-DIRECT-CLOSEOUT-RERUN-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-DIRECT-CLOSEOUT-RERUN-001
```

## Purpose

Rerun the adopted MapStoreAny direct closeout contract basis and materialize the
scoped `SetSurfacePolicy / MapStoreAny` direct closeout.

This closes only the MapStoreAny scoped Write surface. It keeps the Any write
boundary as declared metadata only, does not open runtime Any write authority,
and does not yet close `WriteScalarI64Routes` or `ScalarKnownTransportAxis`.

## Materialized Contract

```text
contract_id = WriteSetMapStoreAnyDirectCloseoutContract
surface_id = WriteScalarI64Routes
subsurface_id = SetSurfacePolicy/MapStoreAny
routes = MapStoreAny
proof_or_policy_source = SetSurfacePolicy, AnyWriteBoundaryDeclared
core_method_op = MapSet
core_method_lowering_tier = ColdFallback
result_class = NoneResult
return_shape = None
value_demand = WriteAny
write_value_boundary = Any
publication_policy = NonePublication
effect_class = mutate
mutation_class = MutatesReceiverOrContainer
runtime_mutation_authority = false
publication_execution = false
mapstore_any_included = true
any_write_boundary_declared = true
any_write_boundary_opened = false
```

## Result

```text
write_set_mapstore_any_direct_closeout_materialized = 1
accepted_scoped_closeout_count = 7
any_write_boundary_declared = 1
any_write_boundary_opened = 0

write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0
runtime_mutation_authority = 0
publication_execution = 0
source_selfhost_claim = 0

decision:
  SelectWriteScalarI64RoutesCloseoutBasis

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SCALAR-I64-ROUTES-CLOSEOUT-BASIS-001
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_set_mapstore_any_direct_closeout_rerun_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-write-set-mapstore-any-direct-closeout-rerun-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_scalar_known_write_set_mapstore_any_direct_closeout_rerun.py
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
