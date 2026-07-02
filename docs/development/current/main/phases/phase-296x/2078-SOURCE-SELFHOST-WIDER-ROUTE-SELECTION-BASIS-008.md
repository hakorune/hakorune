# 2078 - SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-008

## Token

```text
SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-008
```

## Purpose

Park the DomainObject/Id lane after non-hardcoded authority exhaustion and
select the next wider lane by fixture evidence.

## Result

```text
domain_object_id_lane_parked = 1
domain_object_id_subaxis_selection_eligible = 0
candidate_lane_count = 6
selection_eligible_lane_count = 1

decision:
  SelectCarrierTypeTransportRemainingAxisPriorityBasis

reason:
  DomainObjectIdAuthorityExhaustedReturnToNearestUnexhaustedParentLane

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-PRIORITY-BASIS-001
```

## Interpretation

DomainObject/Id cannot safely select PlanRecipe, MIR, AST, Context/Span, or
Other because explicit semantic resource-domain declaration sources and stable
closed-resource manifests are both absent. The lane is parked until a new
non-self-signed authority source appears.

The selected next card is the parent carrier/type remaining-axis priority
basis. This card does not select a concrete carrier/type axis.

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_domain_object_id_lane_guard.sh \
  post_domain_object_id_exhaustion_wider_selection
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    source-selfhost-wider-route-selection-basis-008-v0.json

tool:
  tools/rust_lifecycle/
    source_selfhost_wider_route_selection_basis_008.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_domain_object_id_lane_guard.sh
```

## Reentry Conditions

```text
new_explicit_semantic_resource_domain_declaration_source
new_stable_closed_resource_manifest
new_non_self_signed_resource_taxonomy_authority
```

## Non-Claims

```text
source_selfhost_claim = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
accepted_typed_dependency_edge_materialized = 0
manual_subaxis_selection = 0
manual_type_to_subaxis_assignment = 0
return_type_string_to_subaxis_mapping = 0
source_path_as_policy_authority = 0
observed_domain_subaxis_set_as_policy_authority = 0
row_count_as_proof = 0
owner_name_as_proof = 0
shape_signature_as_proof = 0
route_membership_alone_as_proof = 0
self_signed_taxonomy = 0
```
