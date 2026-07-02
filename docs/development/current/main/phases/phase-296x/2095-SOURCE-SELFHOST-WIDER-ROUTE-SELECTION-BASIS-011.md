# 2095 - SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-011

## Token

```text
SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-011
```

## Purpose

Park the MissingProjectionPolicy post-TypeTransport lane after no
machine-derived post-Type lane is eligible. This wider selector keeps Source
Selfhost stopped and does not select a projection policy, freshness rerun,
borrow lane, native checkpoint, or guard consolidation.

## Previous State

```text
previous_decision = KeepStopped
previous_reason_token = NoMachineDerivedMissingProjectionPolicyRerun005Lane

candidate_lane_count = 4
selection_eligible_lane_count = 0

remaining_blocker_cluster_count = 5
remaining_blocker_candidate_count = 185

type_only_cluster_count = 73
type_only_candidate_count = 819
```

## Parked Lane

```text
MissingProjectionPolicyPostTypeTransportLane:
  parked = true
  park_reason_token = NoMachineDerivedMissingProjectionPolicyRerun005Lane
  projection_policy_selected = 0
```

## Candidate Lanes

```text
NativeOwnerCheckpointRerun:
  selection_eligible = false

UnconvertedSurfaceReportRerun:
  selection_eligible = false

BorrowSurfacePolicyLane:
  selection_eligible = false

GuardConsolidation:
  selection_eligible = false
```

## Decision

```text
decision:
  KeepStopped

reason_token:
  NoMachineDerivedPostMissingProjectionPolicyWiderLane

selected_next_card:
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    source-selfhost-wider-route-selection-basis-011-v0.json

tool:
  tools/rust_lifecycle/
    source_selfhost_wider_route_selection_basis_011.py

guard:
  tools/checks/
    rust_lifecycle_source_selfhost_wider_route_selection_basis_011_guard.sh
```

## Non-Claims

```text
projection_policy_selected = 0
source_selfhost_claim = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
row_count_as_proof = 0
cluster_size_as_proof = 0
source_path_as_authority = 0
owner_name_as_proof = 0
historical_preference_as_proof = 0
basis_010_exactly_one_wider_lane_as_projection_policy_proof = 0
type_transport_exhausted_as_projection_policy_proof = 0
type_only_cluster_direct_selection = 0
owner_edge_repair_as_projection_policy_proof = 0
shape_signature_inventory_as_projection_policy_proof = 0
residual_blocker_count_as_root_proof = 0
type_only_cluster_count_as_root_proof = 0
freshness_rerun_as_semantic_priority = 0
```
