# 2084 - MIRBUILDER-CARRIER-TYPE-TRANSPORT-COMPONENT-EVIDENCE-SOURCE-DISCOVERY-BASIS-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-TRANSPORT-COMPONENT-EVIDENCE-SOURCE-DISCOVERY-BASIS-001
```

## Purpose

Define the final carrier/type-internal authority basis for discovering
non-self-signed component evidence sources after 2083 found no root component
requirement.

This card does not select a concrete carrier/type axis or a component-specific
card.

## Result

```text
component_requirement_count = 7
accepted_component_evidence_source_count = 0
component_specific_card_selection = 0
concrete_carrier_type_axis_selection = 0

decision:
  SelectCarrierTypeComponentEvidenceSourceDiscoveryInventory

reason:
  CarrierTypeComponentEvidenceSourceDiscoveryBasisDefined

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-TRANSPORT-COMPONENT-EVIDENCE-SOURCE-DISCOVERY-INVENTORY-001
```

## Allowed Source Kinds

```text
StableComponentPolicyContract
ExplicitBoundaryDeclaration
StableCrossLaneHandoffContract
CollectionOverlapContract
TypedDirectCloseoutContract
```

## Forbidden Source Kinds

```text
ReturnTypeStringMapping
SourcePathOrModuleInference
OwnerNameInference
ShapeSignatureInference
RouteMembershipAlone
ObservedSubaxisSet
RowCount
LexicalOrder
ApparentSimplicity
SelfSignedFixture
```

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_carrier_type_transport_component_evidence_source_discovery_basis_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-transport-component-evidence-source-discovery-basis-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_transport_component_evidence_source_discovery_basis.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_type_transport_component_evidence_source_discovery_basis_guard.sh
```

## Non-Claims

```text
source_selfhost_claim = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
accepted_typed_dependency_edge_materialized = 0
component_specific_card_selection = 0
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
self_signed_component_authority = 0
```
