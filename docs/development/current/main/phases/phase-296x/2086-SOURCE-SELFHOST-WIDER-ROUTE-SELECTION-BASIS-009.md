# 2086 - SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-009

## Token

```text
SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-009
```

## Purpose

Park the carrier/type remaining-axis lane after component evidence source
authority exhaustion and select the next wider lane.

This card does not select a concrete carrier/type axis, component requirement,
or parent policy candidate.

## Result

```text
carrier_type_remaining_lane_parked = 1
component_authority_source_count = 0
candidate_lane_count = 6
selection_eligible_lane_count = 1

decision:
  SelectCarrierTypeParentPolicyLanePriorityBasis

reason:
  CarrierTypeRemainingLaneParkedReturnToParentPolicyLanePriority

selected_lane:
  CarrierTypeParentPolicyLanePriority

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-TRANSPORT-PARENT-POLICY-LANE-PRIORITY-BASIS-001
```

## Candidate Lanes

```text
UnconvertedSurfaceReportRerun:
  selection_eligible = false

NativeOwnerCheckpointRerun:
  selection_eligible = false

CarrierTypeParentPolicyLanePriority:
  selection_eligible = true

MissingProjectionPolicyNextLane:
  selection_eligible = false

BorrowSurfacePolicyLane:
  selection_eligible = false

GuardConsolidation:
  selection_eligible = false
```

## Guard

```text
tools/checks/rust_lifecycle_source_selfhost_wider_route_selection_basis_009_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    source-selfhost-wider-route-selection-basis-009-v0.json

tool:
  tools/rust_lifecycle/
    source_selfhost_wider_route_selection_basis_009.py

guard:
  tools/checks/
    rust_lifecycle_source_selfhost_wider_route_selection_basis_009_guard.sh
```

## Non-Claims

```text
source_selfhost_claim = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
component_specific_card_selection = 0
concrete_carrier_type_axis_selection = 0
direct_parent_policy_candidate_selection = 0
manual_lane_selection = 0
hardcoded_lane_priority = 0
row_count_as_proof = 0
source_path_as_authority = 0
owner_name_as_proof = 0
route_membership_alone_as_proof = 0
observed_subaxis_set_as_proof = 0
return_type_string_mapping_as_proof = 0
```
