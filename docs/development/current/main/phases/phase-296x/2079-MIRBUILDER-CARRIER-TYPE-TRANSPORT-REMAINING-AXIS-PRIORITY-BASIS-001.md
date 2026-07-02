# 2079 - MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-PRIORITY-BASIS-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-PRIORITY-BASIS-001
```

## Purpose

Define the selector basis for remaining carrier/type axes after
DomainObject/Id is parked. This card does not select a concrete axis.

## Result

```text
domain_object_id_lane_parked = 1
parked_axis_count = 1
deferred_parent_policy_lane_count = 4
candidate_axis_count = 5
basis_selection_eligible_axis_count = 0
basis_selects_concrete_axis = 0

decision:
  SelectCarrierTypeRemainingAxisPriorityRerun

reason:
  CarrierTypeRemainingAxisPriorityBasisDefined

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-PRIORITY-RERUN-001
```

## Candidate Scope

```text
parked:
  DomainObjectOrIdTransportAxis

candidate_axes:
  ProductTupleTransportAxis
  CollectionCarrierTransportAxis
  IteratorOrBorrowTypeTransportAxis
  OpaqueTypeTransportAxis
  ScalarKnownTransportAxis

deferred_parent_policy_lanes:
  ResultCarrierPolicyCandidate
  OptionCarrierPolicyCandidate
  SelfConstructorTransportPolicyCandidate
  CollectionCarrierPolicyCandidate
```

The deferred parent policy lanes are recorded as diagnostics, not selected in
this basis.

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_carrier_type_transport_remaining_axis_priority_basis_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-transport-remaining-axis-priority-basis-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_transport_remaining_axis_priority_basis.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_type_transport_remaining_axis_priority_basis_guard.sh
```

## Non-Claims

```text
source_selfhost_claim = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
accepted_typed_dependency_edge_materialized = 0
manual_axis_selection = 0
manual_carrier_selection = 0
hardcoded_carrier_axis_priority = 0
row_count_as_proof = 0
source_path_as_authority = 0
owner_name_as_proof = 0
route_membership_alone_as_proof = 0
observed_subaxis_set_as_proof = 0
return_type_string_mapping_as_proof = 0
```
