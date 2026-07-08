# 2146 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-DIRECT-CLOSEOUT-CONTRACT-BASIS-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-DIRECT-CLOSEOUT-CONTRACT-BASIS-001
```

## Purpose

Define the basis-only direct closeout contract for the adopted
`SetSurfacePolicy / MapStoreAny` Write surface.

This card defines contract vocabulary only. It does not materialize direct
closeout, does not open runtime Any write authority, and does not close
`WriteScalarI64Routes` or `ScalarKnownTransportAxis`.

## Contract

```text
contract_id = WriteSetMapStoreAnyDirectCloseoutContract
source_kind = AnyWriteDirectCloseoutContract
surface_id = WriteScalarI64Routes
subsurface_id = SetSurfacePolicy/MapStoreAny
route_kind_set = MapStoreAny
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
hako_owner = write_set_mapstore_any_policy_classifier
runtime_mutation_authority = false
publication_execution = false
mapstore_any_included = true
any_write_boundary_declared = true
any_write_boundary_opened = false
```

## Result

```text
write_set_mapstore_any_direct_closeout_contract_basis = 1
write_set_mapstore_any_route_count = 1
any_write_boundary_declared = 1
any_write_boundary_opened = 0
direct_contract_materialized = 0
write_set_mapstore_any_direct_closeout_ready = 0
write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0
runtime_mutation_authority = 0
publication_execution = 0
source_selfhost_claim = 0

decision:
  SelectWriteSetMapStoreAnyDirectCloseoutRerun

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-DIRECT-CLOSEOUT-RERUN-001
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_set_mapstore_any_direct_closeout_contract_basis_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-write-set-mapstore-any-direct-closeout-contract-basis-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_scalar_known_write_set_mapstore_any_direct_closeout_contract_basis.py
```

## Non-Claims

```text
any_write_boundary_opened = 0
direct_contract_materialized = 0
write_set_mapstore_any_direct_closeout_ready = 0
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
