# 2092 - MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-RERUN-005

## Token

```text
MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-RERUN-005
```

## Purpose

Consume MissingProjectionPolicy V4 and BASIS-010 after carrier/type authority
exhaustion. This card records the post-TypeTransport inventory and selects a
selector basis. It does not select a projection policy cluster.

## Post-Type Inventory

```text
input_candidate_count = 1004
input_cluster_count = 78

TypeTransportMissing:
  treated_as = ParkedExhausted
  silently_deleted = false

remaining blockers:
  remaining_blocker_cluster_count = 5
  remaining_blocker_candidate_count = 185
  remaining_blocker_classes =
    NoExactOrFixtureMappedOwnerEdge
    MissingShapeSignatureClusterAxis

type-only clusters:
  type_only_cluster_count = 73
  type_only_candidate_count = 819
  type_only_clusters_are_directly_selectable = false
```

## Result

```text
decision:
  SelectPostTypeExhaustionSelectionBasis

reason_token:
  PostTypeTransportExhaustionSelectorBasisRequired

selected_next_card:
  MIRBUILDER-MISSING-PROJECTION-POLICY-POST-TYPE-EXHAUSTION-SELECTION-BASIS-001
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-missing-projection-policy-cluster-resolution-rerun-005-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_missing_projection_policy_cluster_resolution_rerun_005.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_missing_projection_policy_cluster_resolution_rerun_005_guard.sh
```

## Non-Claims

```text
new_projection_policy_selected = 0
source_selfhost_claim = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
manual_lane_selection = 0
manual_family_selection = 0
manual_shape_selection = 0
manual_axis_selection = 0
row_count_as_proof = 0
cluster_size_as_proof = 0
source_path_as_authority = 0
owner_name_as_proof = 0
historical_preference_as_proof = 0
```
