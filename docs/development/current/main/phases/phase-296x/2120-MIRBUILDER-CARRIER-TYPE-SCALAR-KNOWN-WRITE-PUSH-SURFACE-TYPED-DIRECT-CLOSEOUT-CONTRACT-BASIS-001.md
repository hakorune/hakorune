# 2120 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-PUSH-SURFACE-TYPED-DIRECT-CLOSEOUT-CONTRACT-BASIS-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-PUSH-SURFACE-TYPED-DIRECT-CLOSEOUT-CONTRACT-BASIS-001
```

## Purpose

Define the basis-only typed direct closeout contract for the adopted
`PushSurfacePolicy / ArrayAppendAny` Write surface.

This card defines contract vocabulary only. It does not materialize direct
closeout, does not include Delete/Set Write sub-surfaces, and does not close
`WriteScalarI64Routes` or `ScalarKnownTransportAxis`.

## Contract

```text
contract_id = WritePushSurfaceTypedDirectCloseoutContract
surface_id = WriteScalarI64Routes
subsurface_id = PushSurfacePolicy
route_kind_set = ArrayAppendAny
proof_or_policy_source = PushSurfacePolicy
core_method_op = ArrayPush
core_method_lowering_tier = ColdFallback
result_class = ScalarI64Result
return_shape = ScalarI64
value_demand = WriteAny
publication_policy = NoPublication
effect_class = mutate
mutation_class = MutatesReceiverOrContainer
hako_owner = write_push_surface_policy_classifier
runtime_mutation_authority = false
delete_surface_policy_included = false
set_surface_policy_included = false
```

## Result

```text
write_push_surface_typed_direct_closeout_contract_basis = 1
write_push_route_count = 1
direct_contract_materialized = 0
write_push_direct_closeout_ready = 0
write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0
runtime_mutation_authority = 0
source_selfhost_claim = 0

decision:
  SelectWritePushSurfaceDirectCloseoutRerun

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-PUSH-SURFACE-DIRECT-CLOSEOUT-RERUN-001
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_push_surface_typed_direct_closeout_contract_basis_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-write-push-surface-typed-direct-closeout-contract-basis-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_scalar_known_write_push_surface_typed_direct_closeout_contract_basis.py
```

## Non-Claims

```text
direct_contract_materialized = 0
write_push_direct_closeout_ready = 0
write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0
source_selfhost_claim = 0
hako_generation = 0
new_route_authority = 0
behavior_change = 0
runtime_mutation_authority = 0
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
