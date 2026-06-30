# 1940 - MIRBUILDER-CRATE-WIDE-SURFACE-REPORT-OWNER-CLUSTER-FIELD-001

## Token

```text
MIRBUILDER-CRATE-WIDE-SURFACE-REPORT-OWNER-CLUSTER-FIELD-001
```

## Purpose

Audit the owner-cluster fields in the crate-wide unconverted surface report.

This card exists after the broad owner clusters were decomposed:

```text
JoinIRPlanCluster
JoinIRRouteVerifyCluster
ContextRegistryCluster
CallLoweringCluster / EmissionSsaPhiCluster / StatementValueConstructionCluster
```

The next move is not to select a family by hand. The next move is to prove the
report carries the required owner-cluster fields for every item, then identify
the remaining owner-cluster field-quality gap.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-crate-wide-surface-report-owner-cluster-field-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_crate_wide_surface_report_owner_cluster_field.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_crate_wide_surface_report_owner_cluster_field_guard.sh
```

## Input Authority

```text
source report:
  mirbuilder-crate-wide-unconverted-surface-report-v0.json

cluster resolution:
  mirbuilder-crate-wide-missing-projection-policy-cluster-resolution-v0.json

priority result:
  mirbuilder-projection-policy-cluster-priority-resolution-v0.json

current blocker:
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
```

## Required Field Audit

The fixture must verify that every reported item has these fields:

```text
source_id
source_path
symbol
classification
reason_token
known_owner_edge
owner_edge_confidence
likely_owner_cluster
```

The fixture must not treat cluster size as proof. It may use residual cluster
size only after confirming that required owner-cluster fields are present.

## Acceptance

```text
source_report_consumed = 1
projection_cluster_resolution_consumed = 1
projection_priority_consumed = 1
owner_cluster_field_audited = 1
reported_item_count = 1584
missing_projection_policy_count = 1384
field_gaps = 0
likely_owner_cluster_present_for_every_item = 1
owner_edge_confidence_present_for_every_item = 1
known_owner_edge_field_present_for_every_item = 1
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
required owner-cluster field gaps = 0

residual owner clusters:
  OtherMissingProjectionPolicyCluster = 185
  JoinIRRouteRegistryCluster = 37
  FastMemCluster = 19

decision = SelectOtherOwnerClusterDecomposition
selected_next_card =
  MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-OWNER-CLUSTER-001
```

`OtherMissingProjectionPolicyCluster` is selected because it is the remaining
owner-cluster field-quality gap: its rows have `known_owner_edge = ""` and
`owner_edge_confidence = None`, while the known clusters already have explicit
cluster names.

## Follow-Up Queue

```text
1. MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-OWNER-CLUSTER-001
2. MIRBUILDER-MISSING-PROJECTION-POLICY-JOINIR-ROUTE-REGISTRY-CLUSTER-001
3. MIRBUILDER-MISSING-PROJECTION-POLICY-FASTMEM-CLUSTER-001
```

## Stop Conditions

Stop for consultation if this audit would require:

```text
manual family selection
new owner-cluster taxonomy not derivable from source path / existing SSOT
new Hako syntax
new ABI or backend route
runtime fallback
VM/interpreter as semantic owner
Source Selfhost claim
```

## Non-Claims

```text
no Hako projection
no native source seed
no HakoAdopted decision
no Source Selfhost claim
```
