# 1945 - MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-SHAPE-SIGNATURE-CLUSTER-RESOLUTION-001

## Token

```text
MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-SHAPE-SIGNATURE-CLUSTER-RESOLUTION-001
```

## Purpose

Resolve the next shape-signature cluster from the repaired Other owner rows.

The previous inventory produced 11 diagnostic `shape.other_*` candidates. This
card evaluates them by evidence quality, not by cluster size.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-missing-projection-policy-other-shape-signature-cluster-resolution-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_missing_projection_policy_other_shape_signature_cluster_resolution.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_missing_projection_policy_other_shape_signature_cluster_resolution_guard.sh
```

## Selection Rule

```text
required:
  type transport already Known
  no borrow or receiver policy gap
  no carrier policy gap

forbidden as proof:
  cluster size
  coverage percentage
  route membership alone
```

## Acceptance

```text
other_shape_signature_inventory_consumed = 1
input_shape_signature_count = 11
input_other_owner_cluster_count = 185
shape_clusters_evaluated_by_evidence_quality = 1
selection_eligible_shape_count = 1
selected_shape_signature = shape.other_unit_observer_surface
selected_candidate_count = 26
selected_subcluster_count = 17
cluster_size_as_proof = 0
manual_family_selection = 0
coverage_percentage_as_proof = 0
route_membership_alone_as_proof = 0
generated_artifact_as_edit_authority = 0
hako_generation = 0
hako_adopted_decision = 0
native_source_seed_materialization = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```

## Result

```text
eligible shape:
  shape.other_unit_observer_surface

decision:
  SelectOtherShapeSignatureProjectionPolicy

selected_next_card:
  MIRBUILDER-OTHER-UNIT-OBSERVER-SURFACE-PROJECTION-POLICY-001
```

This card does not define the projection policy. It only selects the next
policy owner by deterministic evidence quality.

## Stop Conditions

Stop for consultation if the next step requires:

```text
manual shape selection
cluster size as proof
new Hako syntax
runtime fallback
new ABI or backend route
VM/interpreter as semantic owner
Source Selfhost claim
```

## Non-Claims

```text
no projection policy materialized
no Hako generation
no native source seed
no HakoAdopted decision
no Source Selfhost claim
```
