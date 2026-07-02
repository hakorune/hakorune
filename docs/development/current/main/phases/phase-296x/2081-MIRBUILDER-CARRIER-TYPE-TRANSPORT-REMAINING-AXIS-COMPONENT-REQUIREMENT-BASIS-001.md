# 2081 - MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-BASIS-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-BASIS-001
```

## Purpose

Define component requirements for the five remaining carrier/type axes after
the priority rerun found no proof-complete axis.

This card does not select a concrete carrier/type axis.

## Result

```text
candidate_axis_count = 5
component_requirement_count = 7
root_component_requirement_count = 0
component_specific_card_selection_eligible_count = 0
concrete_carrier_type_axis_selection = 0

decision:
  SelectCarrierTypeRemainingAxisComponentRequirementInventory

reason:
  CarrierTypeRemainingAxisComponentRequirementsDefined

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-INVENTORY-001
```

## Component Requirements

```text
ProductTupleTransportAxis:
  TupleFieldDomainBoundaryPolicy
  TupleElementTransportPolicy

CollectionCarrierTransportAxis:
  CollectionPolicyOverlapResolution
  CollectionElementCarrierPolicy

IteratorOrBorrowTypeTransportAxis:
  IteratorBorrowBoundaryRoutingPolicy

OpaqueTypeTransportAxis:
  OpaqueTypeBoundaryDeclaration

ScalarKnownTransportAxis:
  ScalarKnownCloseoutAuthority
```

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_carrier_type_transport_remaining_axis_component_requirement_basis_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-transport-remaining-axis-component-requirement-basis-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_transport_remaining_axis_component_requirement_basis.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_type_transport_remaining_axis_component_requirement_basis_guard.sh
```

## Non-Claims

```text
source_selfhost_claim = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
accepted_typed_dependency_edge_materialized = 0
concrete_carrier_type_axis_selection = 0
manual_axis_selection = 0
manual_carrier_selection = 0
hardcoded_carrier_axis_priority = 0
row_count_as_proof = 0
source_path_as_authority = 0
owner_name_as_proof = 0
route_membership_alone_as_proof = 0
return_type_string_mapping_as_proof = 0
observed_subaxis_set_as_proof = 0
```
