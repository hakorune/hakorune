# 2129 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-DIRECT-CLOSEOUT-RERUN-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-DIRECT-CLOSEOUT-RERUN-001
```

## Purpose

Rerun the adopted Delete typed direct closeout contract basis and materialize
the scoped `DeleteSurfacePolicy / MapDeleteAny` direct closeout.

This closes only the Delete Write sub-surface. It does not close
`WriteScalarI64Routes`, does not close `ScalarKnownTransportAxis`, and does not
open runtime mutation authority or publication execution.

## Materialized Contract

```text
contract_id = WriteDeleteSurfaceTypedDirectCloseoutContract
surface_id = WriteScalarI64Routes
subsurface_id = DeleteSurfacePolicy
routes = MapDeleteAny
proof_or_policy_source = DeleteSurfacePolicy
core_method_op = MapDelete
core_method_lowering_tier = ColdFallback
result_class = ScalarI64Result
return_shape = ScalarI64
value_demand = WriteAny
publication_policy = NonePublication
effect_class = mutate
mutation_class = MutatesReceiverOrContainer
runtime_mutation_authority = false
publication_execution = false
```

## Result

```text
write_delete_surface_direct_closeout_materialized = 1
accepted_scoped_closeout_count = 5
remaining_write_subsurface_count = 1
remaining_write_subsurfaces = SetSurfacePolicy

write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0
runtime_mutation_authority = 0
publication_execution = 0
source_selfhost_claim = 0

decision:
  SelectWriteRemainingSubsurfacePostDeleteCloseoutRerun

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-REMAINING-SUBSURFACE-POST-DELETE-CLOSEOUT-RERUN-001
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_delete_surface_direct_closeout_rerun_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-write-delete-surface-direct-closeout-rerun-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_scalar_known_write_delete_surface_direct_closeout_rerun.py
```

## Non-Claims

```text
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
