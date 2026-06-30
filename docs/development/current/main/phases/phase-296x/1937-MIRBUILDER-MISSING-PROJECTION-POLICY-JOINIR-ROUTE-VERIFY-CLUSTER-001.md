# 1937 - MIRBUILDER-MISSING-PROJECTION-POLICY-JOINIR-ROUTE-VERIFY-CLUSTER-001

## Token

```text
MIRBUILDER-MISSING-PROJECTION-POLICY-JOINIR-ROUTE-VERIFY-CLUSTER-001
```

## Purpose

Decompose the remaining `JoinIRRouteVerifyCluster` MissingProjectionPolicy
surface into machine-derived subclusters.

This card exists because the previous broad `JoinIRRouteVerify` policy kept the
evidence-quality slice parent-owned, while the crate-wide unconverted surface
report still exposes a larger route-verify surface:

```text
previous policy:
  MIRBUILDER-JOIN-I-R-ROUTE-VERIFY-PROJECTION-POLICY-001
  decision = KeepParentOwner
  source_count = 53

crate-wide unconverted surface:
  MissingProjectionPolicy = 1384
  JoinIRRouteVerifyCluster = 206
```

The next move is not to reopen the whole route-verify owner and not to generate
Hako. The next move is to split the remaining route-verify surface by role,
return transport, borrow shape, and source module so a later card can derive a
narrow owner without manual family selection.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-missing-projection-policy-joinir-route-verify-cluster-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_missing_projection_policy_joinir_route_verify_cluster.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_missing_projection_policy_joinir_route_verify_cluster_guard.sh
```

## Input Authority

```text
source report:
  mirbuilder-crate-wide-unconverted-surface-report-v0.json

cluster resolution:
  mirbuilder-crate-wide-missing-projection-policy-cluster-resolution-v0.json

priority result:
  mirbuilder-projection-policy-cluster-priority-resolution-v0.json

previous parent-owned policy:
  mirbuilder-join-i-r-route-verify-projection-policy-v0.json

current blocker:
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
```

## Required Classification

The fixture must partition `JoinIRRouteVerifyCluster` rows without selecting an
owner.

Minimum subcluster axes:

```text
route_verify_role
source_module
shape_signature
borrow_axis
type_transport_axis
return_family
receiver_axis
verifier_or_oracle_state
public_or_private_surface
cfg_test_surface
```

Expected route-verify roles:

```text
edgecfg_compose_or_verify
facts_or_recognizer
recipe_index_or_ref
joinir_routing_or_trace
joinir_merge_coordinator
joinir_merge_contract
joinir_merge_rewriter
joinir_merge_helper
verify_diagnostic
verify_observability
verify_predicate_or_guard
diagnostic_or_debug_helper
test_only_surface
route_verify_other
```

## Decision Rule

```text
if exactly one evidence-backed subcluster is small and selectable:
  decision = SelectProjectionPolicySubcluster
  selected_next_card = <SUBCLUSTER>-PROJECTION-POLICY-001

elif exactly one subcluster needs verifier/oracle repair:
  decision = SelectVerifierOrOracleRepair
  selected_next_card = <SUBCLUSTER>-VERIFIER-OR-ORACLE-REPAIR-001

elif exactly one subcluster needs type-transport policy:
  decision = SelectTypeTransportPolicy
  selected_next_card = <SUBCLUSTER>-TYPE-TRANSPORT-POLICY-001

else:
  decision = KeepStopped
  selected_next_card = SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
```

Cluster size may be used as a batching signal only. It is not proof.

## Acceptance

```text
source_report_consumed = 1
projection_priority_consumed = 1
previous_parent_owned_policy_consumed = 1
input_joinir_route_verify_cluster_count = 206
all_joinir_route_verify_items_partitioned_exactly_once = 1
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

## Result

```text
input_joinir_route_verify_cluster_count = 206
subcluster_count = 81
selection_eligible_subcluster_count = 42
decision = KeepStopped
reason = AmbiguousJoinIRRouteVerifyProjectionSubclusters
```

## Follow-Up Queue

If this card does not produce exactly one executable owner, continue with the
remaining broad owner clusters in this order:

```text
1. MIRBUILDER-MISSING-PROJECTION-POLICY-CONTEXT-SURFACE-JOIN-001
2. MIRBUILDER-MISSING-PROJECTION-POLICY-CALL-EMIT-SSA-CLUSTER-001
3. MIRBUILDER-CRATE-WIDE-SURFACE-REPORT-OWNER-CLUSTER-FIELD-001
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
