# 2093 - MIRBUILDER-MISSING-PROJECTION-POLICY-POST-TYPE-EXHAUSTION-SELECTION-BASIS-001

## Token

```text
MIRBUILDER-MISSING-PROJECTION-POLICY-POST-TYPE-EXHAUSTION-SELECTION-BASIS-001
```

## Purpose

Define the MissingProjectionPolicy selector after TypeTransport exhaustion.
This basis does not select a projection policy. It selects the post-Type
selection rerun.

## Selector

```text
MissingProjectionPolicyPostTypeExhaustionSelectorV1

basis_selects_projection_policy = false
type_transport_missing_is_parked_not_deleted = true
selection_requires_exactly_one_machine_derived_lane_or_card = true
if_zero_or_multiple_keep_stopped = true
```

## Candidate Lanes

```text
ResidualOwnerEdgeAndShapeSignatureBlockerInventory:
  scope = remaining blocker clusters
  candidate_cluster_count = 5
  candidate_row_count = 185
  selection_eligible = false

TypeOnlyProjectionPolicySelectorBasis:
  scope = type-only clusters after TypeTransport parked
  candidate_cluster_count = 73
  candidate_row_count = 819
  selection_eligible = false

ProjectionDescriptorOverlayFreshnessRerun:
  scope = projection descriptor overlay freshness
  selection_eligible = false

KeepStopped:
  selection_eligible = false
```

## Result

```text
selection_eligible_lane_count = 0

decision:
  SelectPostTypeExhaustionSelectionRerun

reason_token:
  MissingProjectionPolicyPostTypeExhaustionSelectorBasisDefined

selected_next_card:
  MIRBUILDER-MISSING-PROJECTION-POLICY-POST-TYPE-EXHAUSTION-SELECTION-RERUN-001
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-missing-projection-policy-post-type-exhaustion-selection-basis-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_missing_projection_policy_post_type_exhaustion_selection_basis.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_missing_projection_policy_post_type_exhaustion_selection_basis_guard.sh
```

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
