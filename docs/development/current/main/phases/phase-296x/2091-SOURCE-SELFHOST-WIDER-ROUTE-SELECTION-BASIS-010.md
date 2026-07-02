# 2091 - SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-010

## Token

```text
SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-010
```

## Purpose

Park the carrier/type parent policy lane after evidence-source authority
exhaustion and select the next wider lane.

This card does not select Result, Option, SelfConstructor, Collection, or a
carrier/type axis.

## Parked Lanes

```text
DomainObjectIdLane:
  ExplicitSemanticResourceDomainDeclarationSourceMissing

CarrierTypeRemainingAxisLane:
  NoCarrierTypeComponentEvidenceSourceAuthority

CarrierTypeParentPolicyLane:
  NoCarrierTypeParentPolicyLaneEvidenceSourceAuthority
```

## Candidate Lanes

```text
UnconvertedSurfaceReportRerun:
  selection_eligible = false

NativeOwnerCheckpointRerun:
  selection_eligible = false

MissingProjectionPolicyNextLane:
  selection_eligible = true

BorrowSurfacePolicyLane:
  selection_eligible = false

GuardConsolidation:
  selection_eligible = false
```

## Result

```text
carrier_type_parent_policy_lane_parked = 1
candidate_lane_count = 5
selection_eligible_lane_count = 1

decision:
  SelectMissingProjectionPolicyClusterResolutionRerun

reason_token:
  CarrierTypeParentPolicyLaneExhaustedReturnToMissingProjectionPolicy

selected_lane:
  MissingProjectionPolicyNextLane

selected_next_card:
  MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-RERUN-005
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    source-selfhost-wider-route-selection-basis-010-v0.json

tool:
  tools/rust_lifecycle/
    source_selfhost_wider_route_selection_basis_010.py

guard:
  tools/checks/
    rust_lifecycle_source_selfhost_wider_route_selection_basis_010_guard.sh
```

## Non-Claims

```text
source_selfhost_claim = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
parent_policy_candidate_selection = 0
direct_parent_policy_candidate_selection = 0
result_history_as_direct_selection_proof = 0
manual_lane_selection = 0
hardcoded_lane_priority = 0
```
