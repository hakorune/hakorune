# 2004 - MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-V2

## Token

```text
MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-V2
```

## Purpose

Resolve the checkpoint-selected `MissingProjectionPolicy` blocker after the
fresh report and native-owner checkpoint.

This card does not select a new projection policy by count. It checks whether
the eligible projection-policy clusters are already covered by landed descriptor
decisions.

## Result

```text
input_candidate_count = 1384
selection_eligible_cluster_count = 41
excluded_existing_decision_cluster_count = 41
selectable_cluster_count = 0

decision:
  SelectProjectionDescriptorCoverageReclassification

reason_token:
  ProjectionPolicyClustersAlreadyLandedButReportStillMissing

selected_next_card:
  MIRBUILDER-PROJECTION-DESCRIPTOR-COVERAGE-RECLASSIFICATION-001
```

The next issue is not another projection-policy descriptor. It is that the
source-surface report still classifies rows as `MissingProjectionPolicy` after
the corresponding descriptor clusters have landed.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-missing-projection-policy-cluster-resolution-v2-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_missing_projection_policy_cluster_resolution_v2.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_missing_projection_policy_cluster_resolution_v2_guard.sh
```

## Non-Claims

```text
new_projection_policy_selected = 0
manual_family_selection = 0
manual_shape_selection = 0
manual_axis_selection = 0
candidate_count_as_proof = 0
cluster_size_as_proof = 0
coverage_percentage_as_proof = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
```
