# 2094 - MIRBUILDER-MISSING-PROJECTION-POLICY-POST-TYPE-EXHAUSTION-SELECTION-RERUN-001

## Token

```text
MIRBUILDER-MISSING-PROJECTION-POLICY-POST-TYPE-EXHAUSTION-SELECTION-RERUN-001
```

## Purpose

Apply `MissingProjectionPolicyPostTypeExhaustionSelectorV1` after
TypeTransport exhaustion. This rerun does not select a projection policy or a
post-Type lane because no candidate has machine-derived selection authority.

## Result

```text
candidate_lane_count = 4
selection_eligible_lane_count = 0

remaining_blocker_cluster_count = 5
remaining_blocker_candidate_count = 185

type_only_cluster_count = 73
type_only_candidate_count = 819

new_projection_policy_selected = 0
```

## Decision

```text
decision:
  KeepStopped

reason_token:
  NoMachineDerivedMissingProjectionPolicyRerun005Lane

selected_lane:
  null

selected_projection_policy_cluster:
  null

selected_next_card:
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-missing-projection-policy-post-type-exhaustion-selection-rerun-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_missing_projection_policy_post_type_exhaustion_selection_rerun.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_missing_projection_policy_post_type_exhaustion_selection_rerun_guard.sh
```

## Consultation Point

The next design question is whether to open a residual owner-edge/shape blocker
inventory, a type-only cluster selector basis, an overlay freshness rerun, or a
different wider selector. This rerun intentionally does not choose among them.

## Non-Claims

```text
new_projection_policy_selected = 0
basis_010_exactly_one_wider_lane_as_projection_policy_proof = 0
type_transport_exhausted_as_projection_policy_proof = 0
type_only_cluster_direct_selection = 0
owner_edge_repair_as_projection_policy_proof = 0
shape_signature_inventory_as_projection_policy_proof = 0
row_count_as_proof = 0
cluster_size_as_proof = 0
source_path_as_authority = 0
owner_name_as_proof = 0
historical_preference_as_proof = 0
source_selfhost_claim = 0
```
