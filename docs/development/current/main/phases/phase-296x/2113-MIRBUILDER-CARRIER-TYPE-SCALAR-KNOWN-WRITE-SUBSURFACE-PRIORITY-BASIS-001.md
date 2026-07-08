# 2113 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SUBSURFACE-PRIORITY-BASIS-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SUBSURFACE-PRIORITY-BASIS-001
```

## Purpose

Define the basis-only selector for the Write sub-surface priority rerun after
2112 rejected whole-surface `WriteScalarI64Routes` direct closeout.

This card does not choose Push, Delete, or Set. It only defines the proof axes a
rerun may use before selecting a sub-surface-specific contract basis.

## Candidate Sub-Surfaces

```text
PushSurfacePolicy:
  routes = ArrayAppendAny
  route_count_as_proof = false
  selection_eligible = false

DeleteSurfacePolicy:
  routes = MapDeleteAny
  route_count_as_proof = false
  selection_eligible = false

SetSurfacePolicy:
  routes = MapStoreI64, MapStoreAny
  route_count_as_proof = false
  selection_eligible = false
```

## Selector Rule

```text
rule:
  WriteSubsurfacePriorityMechanicalSelectorV1

select only if exactly one sub-surface has:
  scope_eligible
  stable_result_publication_contract
  mutation_semantics_policy_ready
  direct_contract_shape_ready
  typed_value_boundary_ready_or_not_required
```

`route_count`, lexical order, apparent simplicity, and similarity to accepted
read contracts are not priority authority.

## Result

```text
write_subsurface_priority_basis = 1
candidate_write_subsurface_count = 3
basis_selection_eligible_subsurface_count = 0
basis_selects_write_subsurface = 0

write_direct_closeout_materialized = 0
write_result_policy_ready = 0
write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0

decision:
  SelectWriteSubsurfacePriorityRerun

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SUBSURFACE-PRIORITY-RERUN-001
```

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_subsurface_priority_basis_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-write-subsurface-priority-basis-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_scalar_known_write_subsurface_priority_basis.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_subsurface_priority_basis_guard.sh
```

## Non-Claims

```text
basis_selects_write_subsurface = 0
write_direct_closeout_materialized = 0
write_result_policy_ready = 0
write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0
component_specific_direct_contract_materialized = 0
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
manual_subsurface_selection = 0
manual_axis_selection = 0
manual_carrier_selection = 0
route_count_as_proof = 0
source_path_as_authority = 0
owner_name_as_proof = 0
route_membership_alone_as_proof = 0
apparent_simplicity_as_proof = 0
accepted_read_contract_similarity_as_proof = 0
```
