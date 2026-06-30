# 1939 - MIRBUILDER-MISSING-PROJECTION-POLICY-CALL-EMIT-SSA-CLUSTER-001

## Token

```text
MIRBUILDER-MISSING-PROJECTION-POLICY-CALL-EMIT-SSA-CLUSTER-001
```

## Purpose

Partition the remaining CallLowering, EmissionSsaPhi, and
StatementValueConstruction MissingProjectionPolicy surfaces.

This card exists because these three broad owner clusters already have narrow
decisions, but the crate-wide unconverted surface report still contains
uncovered rows:

```text
CallLoweringCluster = 88
EmissionSsaPhiCluster = 53
StatementValueConstructionCluster = 59
total = 200
```

The next move is not to reopen whole-cluster projection policies and not to
generate Hako. The next move is to partition remaining rows, mark prior narrow
decisions as covered, and expose unresolved type/verifier gaps as stable
machine-readable blockers.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-missing-projection-policy-call-emit-ssa-cluster-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_missing_projection_policy_call_emit_ssa_cluster.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_missing_projection_policy_call_emit_ssa_cluster_guard.sh
```

## Input Authority

```text
source report:
  mirbuilder-crate-wide-unconverted-surface-report-v0.json

cluster resolution:
  mirbuilder-crate-wide-missing-projection-policy-cluster-resolution-v0.json

priority result:
  mirbuilder-projection-policy-cluster-priority-resolution-v0.json

prior narrow decisions:
  mirbuilder-call-lowering-policy-subcluster-decomposition-v0.json
  mirbuilder-statement-value-construction-subcluster-decomposition-v0.json
  mirbuilder-emission-ssa-phi-projection-policy-v0.json

current blocker:
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
```

## Required Classification

The fixture must partition all selected rows without selecting an owner.

Minimum subcluster axes:

```text
source_cluster
surface_role
source_module
shape_signature
borrow_axis
type_transport_axis
return_family
verifier_or_oracle_state
public_or_private_surface
prior_narrow_decision_state
```

Prior narrow decisions must be consumed so already-handled rows do not reenter
candidate selection.

## Decision Rule

```text
if exactly one uncovered evidence-backed subcluster is selectable:
  decision = SelectProjectionPolicySubcluster
  selected_next_card = <SUBCLUSTER>-PROJECTION-POLICY-001

elif exactly one subcluster needs verifier/oracle repair:
  decision = SelectVerifierOrOracleRepair
  selected_next_card = <SUBCLUSTER>-VERIFIER-OR-ORACLE-REPAIR-001

elif exactly one subcluster needs type transport policy:
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
prior_narrow_decisions_consumed = 1
input_call_emit_ssa_cluster_count = 200
all_call_emit_ssa_items_partitioned_exactly_once = 1
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
input_call_emit_ssa_cluster_count = 200
subcluster_count = 61
selection_eligible_subcluster_count = 13
decision = KeepStopped
reason = AmbiguousCallEmitSsaProjectionSubclusters

prior narrow decision state:
  CoveredByCallLoweringSubclusterDecomposition = 12
  CoveredByEmissionSsaPhiProjectionPolicy = 13
  CoveredByStatementValueConstructionSubclusterDecomposition = 10
  UncoveredByPriorNarrowDecision = 165
```

## Follow-Up Queue

If this card does not produce exactly one executable owner, continue with:

```text
1. MIRBUILDER-CRATE-WIDE-SURFACE-REPORT-OWNER-CLUSTER-FIELD-001
```

## Stop Conditions

Stop for consultation if this partition would require:

```text
manual family selection
whole-cluster projection policy
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
