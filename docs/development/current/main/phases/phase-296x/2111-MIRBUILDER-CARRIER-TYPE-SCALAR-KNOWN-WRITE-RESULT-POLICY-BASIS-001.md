# 2111 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-RESULT-POLICY-BASIS-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-RESULT-POLICY-BASIS-001
```

## Purpose

Define the basis-only WriteResultPolicy boundary for the remaining
`WriteScalarI64Routes` surface after 2110 materialized the CollectionLen scoped
closeout.

This card classifies Push/Delete/Set sub-surfaces and declares the mutate plus
mixed return/publication boundary. It does not materialize a write direct
closeout and does not close `ScalarKnownTransportAxis`.

## Policy

```text
policy_id = WriteResultPolicyV1
target_surface_id = WriteScalarI64Routes
route_kind_set = ArrayAppendAny, MapDeleteAny, MapStoreI64, MapStoreAny

PushSurfacePolicy:
  routes = ArrayAppendAny
  normalized_result_class = ScalarI64Result
  publication_class = NoPublication
  mutation_class = MutatesReceiverOrContainer

DeleteSurfacePolicy:
  routes = MapDeleteAny
  normalized_result_class = ScalarI64Result
  publication_class = NonePublication
  mutation_class = MutatesReceiverOrContainer

SetSurfacePolicy:
  routes = MapStoreI64, MapStoreAny
  normalized_result_class = NoneResult
  publication_class = NonePublication
  mutation_class = MutatesReceiverOrContainer
  MapStoreI64.typed_scalar_write = 1
  MapStoreAny.typed_scalar_write = 0
```

## Boundary

```text
observed_return_shape = ScalarI64OrNoneMixed
observed_publication_policy = MixedNoPublicationAndNone
mixed_state_is_not_direct_closeout_contract = true
effect_class = mutate
direct_closeout_requires_rerun = true
```

## Result

```text
write_result_policy_basis = 1
write_surface_policy_boundary_defined = 1
mutate_effect_boundary_declared = 1
write_subsurface_classification_defined = 1
push_surface_policy_defined = 1
delete_surface_policy_defined = 1
set_surface_policy_defined = 1
mixed_return_publication_policy_declared = 1
basis_only = 1
rerun_required_before_direct_closeout = 1

write_direct_closeout_materialized = 0
write_result_policy_ready = 0
write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0

decision:
  SelectWriteResultPolicyRerun

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-RESULT-POLICY-RERUN-001
```

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_result_policy_basis_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-write-result-policy-basis-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_scalar_known_write_result_policy_basis.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_result_policy_basis_guard.sh
```

## Non-Claims

```text
write_direct_closeout_materialized = 0
write_result_policy_ready = 0
write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0
component_specific_card_selection = 0
concrete_carrier_type_axis_selection = 0
hako_adoption = 0
source_selfhost_claim = 0
new_route_authority = 0
behavior_change = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
new_python_semantic_projector = 0
manual_axis_selection = 0
manual_carrier_selection = 0
row_count_as_proof = 0
source_path_as_authority = 0
owner_name_as_proof = 0
route_membership_alone_as_proof = 0
```
