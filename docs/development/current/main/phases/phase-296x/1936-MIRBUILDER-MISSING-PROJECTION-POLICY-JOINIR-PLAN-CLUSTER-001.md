# 1936 - MIRBUILDER-MISSING-PROJECTION-POLICY-JOINIR-PLAN-CLUSTER-001

## Token

```text
MIRBUILDER-MISSING-PROJECTION-POLICY-JOINIR-PLAN-CLUSTER-001
```

## Purpose

Decompose the remaining `JoinIRPlanCluster` MissingProjectionPolicy surface into
machine-derived subclusters.

This card exists because the projection-policy priority queue is exhausted:

```text
projection_policy_priority:
  decision = KeepStopped
  reason = NoEligibleProjectionPolicyCluster
  selectable_cluster_count = 0

crate_wide_unconverted_surface:
  MissingProjectionPolicy = 1384
  JoinIRPlanCluster = 623
```

The next move is not to select a family by hand and not to generate Hako. The
next move is to split the largest remaining owner cluster into smaller,
evidence-backed subclusters that can later produce exactly one next owner.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-missing-projection-policy-joinir-plan-cluster-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_missing_projection_policy_joinir_plan_cluster.py

guard:
  prefer source-selfhost family guard manifest if possible;
  add a row-specific guard only if the clustering contract cannot be covered
  by the reusable manifest guard.
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

## Required Classification

The fixture must partition `JoinIRPlanCluster` rows without selecting an owner.

Minimum subcluster axes:

```text
source_module
plan_feature_subcluster
joinir_plan_subcluster
shape_signature
borrow_axis
type_transport_axis
verifier_or_oracle_state
public_or_private_surface
cfg_test_surface
```

Expected high-level buckets:

```text
route_local_plan_descriptor
recipe_tree_matcher
plan_feature_helper
loop_plan_extractor
joinir_plan_lowering_surface
diagnostic_or_debug_helper
test_only_surface
unknown_or_needs_owner_edge_repair
```

## Decision Rule

```text
if exactly one evidence-backed subcluster is small and selectable:
  decision = SelectProjectionPolicySubcluster
  selected_next_card = <SUBCLUSTER>-PROJECTION-POLICY-001

elif exactly one subcluster needs owner-edge repair:
  decision = SelectOwnerEdgeRepair
  selected_next_card = <SUBCLUSTER>-OWNER-EDGE-REPAIR-001

elif exactly one subcluster needs shape inventory:
  decision = SelectShapeSignatureInventory
  selected_next_card = <SUBCLUSTER>-SHAPE-SIGNATURE-INVENTORY-001

else:
  decision = KeepStopped
  selected_next_card = SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
```

Cluster size may be used as a batching signal only. It is not proof.

## Acceptance

```text
source_report_consumed = 1
projection_priority_consumed = 1
input_joinir_plan_cluster_count = 623
all_joinir_plan_items_partitioned_exactly_once = 1
subcluster_ids_are_stable = 1
subcluster_reason_tokens_are_stable = 1
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

## Follow-Up Queue

If this card does not produce exactly one executable owner, continue with the
remaining broad owner clusters in this order:

```text
1. MIRBUILDER-MISSING-PROJECTION-POLICY-JOINIR-ROUTE-VERIFY-CLUSTER-001
2. MIRBUILDER-MISSING-PROJECTION-POLICY-CONTEXT-SURFACE-JOIN-001
3. MIRBUILDER-MISSING-PROJECTION-POLICY-CALL-EMIT-SSA-CLUSTER-001
4. MIRBUILDER-CRATE-WIDE-SURFACE-REPORT-OWNER-CLUSTER-FIELD-001
```

## Stop Conditions

Stop for consultation if this clustering would require:

```text
manual family selection
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
