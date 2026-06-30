# 1943 - MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-OWNER-CLUSTER-RERUN-001

## Token

```text
MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-OWNER-CLUSTER-RERUN-001
```

## Purpose

Rerun the `OtherMissingProjectionPolicyCluster` partition after applying the
file-scoped owner-edge confidence repair.

This card verifies that owner-edge confidence is no longer the active blocker
for the 185 Other rows. The rerun shows that every row still lacks a
`shape_signature`, so the next concrete task is a shape-signature inventory.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-missing-projection-policy-other-owner-cluster-rerun-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_missing_projection_policy_other_owner_cluster_rerun.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_missing_projection_policy_other_owner_cluster_rerun_guard.sh
```

## Input Authority

```text
source report:
  mirbuilder-crate-wide-unconverted-surface-report-v0.json

owner-edge confidence repair:
  mirbuilder-missing-projection-policy-other-owner-edge-confidence-repair-v0.json

current blocker:
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
```

## Acceptance

```text
source_report_consumed = 1
owner_edge_confidence_repair_consumed = 1
input_other_owner_cluster_count = 185
all_other_owner_cluster_items_partitioned_exactly_once = 1
owner_edge_confidence_repair_applied = 1
owner_edge_confidence = FileScoped:185
shape_signature = unknown_shape:185
selection_eligible_subcluster_count = 0
shape_signature_gap_selected = 1
manual_family_selection = 0
cluster_size_as_proof = 0
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
input_other_owner_cluster_count = 185
subcluster_count = 123
owner_edge_confidence_counts:
  FileScoped = 185
shape_signature_counts:
  unknown_shape = 185

decision:
  SelectShapeSignatureInventory

selected_next_card:
  MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-SHAPE-SIGNATURE-INVENTORY-001
```

The file-scoped owner edges are diagnostic routing evidence only. They are not
projection proof and do not make any row a HakoAdopted candidate.

## Stop Conditions

Stop for consultation if the next step requires:

```text
manual shape assignment
shape policy inferred from family names
new Hako syntax
runtime fallback
new ABI or backend route
VM/interpreter as semantic owner
Source Selfhost claim
```

## Non-Claims

```text
no Hako generation
no native source seed
no HakoAdopted decision
no Source Selfhost claim
```
