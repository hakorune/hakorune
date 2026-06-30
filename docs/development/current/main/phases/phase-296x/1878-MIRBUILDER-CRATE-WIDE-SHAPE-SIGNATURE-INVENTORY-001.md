# 1878 - MIRBUILDER-CRATE-WIDE-SHAPE-SIGNATURE-INVENTORY-001

## Token

```text
MIRBUILDER-CRATE-WIDE-SHAPE-SIGNATURE-INVENTORY-001
```

## Purpose

Repair the third blocking axis for crate-wide MissingProjectionPolicy
clusters. After owner-edge confidence and stable deny reason repairs, the
remaining selectable surfaces still need shape signatures before projection
policy clusters can be prioritized.

This card derives shape signatures from existing source-surface cluster axes.
It does not define projection policy or emit Hako.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_crate_wide_shape_signature_inventory.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-crate-wide-shape-signature-inventory-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_crate_wide_shape_signature_inventory_guard.sh
```

## Result

```text
input_candidate_count = 1396
shape_signature_count = 54
shape_signature_candidate_count = 1211
unknown_shape_candidate_count_after_inventory = 185

cluster_resolution:
  selection_eligible_cluster_count = 42
  decision = SelectProjectionPolicyClusterPriorityResolution
  next_card = MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001
```

## Boundary

```text
shape source:
  deepest existing subcluster axis
  else likely_owner_cluster

OtherMissingProjectionPolicyCluster:
  remains unknown_shape

shape signature:
  resolver axis only
```

## Acceptance

```text
shape_signature_inventory_defined = 1
shape_signature_candidate_count_after_inventory = 1211
unknown_shape_candidate_count_after_inventory = 185
manual_family_selection = 0
cluster_size_as_proof = 0
coverage_percentage_as_proof = 0
route_membership_alone_as_proof = 0
generated_artifact_as_edit_authority = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
family_name_based_policy = 0
```

## Non-Claims

```text
no Hako generation
no projection policy selection
no HakoAdopted decision
no native source seed materialization
no Source Selfhost claim
```
