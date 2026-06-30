# 1938 - MIRBUILDER-MISSING-PROJECTION-POLICY-CONTEXT-SURFACE-JOIN-001

## Token

```text
MIRBUILDER-MISSING-PROJECTION-POLICY-CONTEXT-SURFACE-JOIN-001
```

## Purpose

Join the remaining `ContextRegistryCluster` MissingProjectionPolicy surface by
context family and operation role.

This card exists because the earlier `ContextRegistry` projection-policy card
kept one constructor surface parent-owned, while the crate-wide unconverted
surface report still exposes a wider context registry surface:

```text
previous policy:
  MIRBUILDER-CONTEXT-REGISTRY-PROJECTION-POLICY-001
  decision = KeepParentOwner
  source_count = 1

crate-wide unconverted surface:
  MissingProjectionPolicy = 1384
  ContextRegistryCluster = 114
```

The next move is not to claim full context selfhost and not to select a context
family by hand. The next move is to join the remaining context surfaces against
the existing native/adjacent context evidence and preserve the gaps as
machine-readable blockers.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-missing-projection-policy-context-surface-join-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_missing_projection_policy_context_surface_join.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_missing_projection_policy_context_surface_join_guard.sh
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
  mirbuilder-context-registry-projection-policy-v0.json

current blocker:
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
```

## Required Classification

The fixture must partition `ContextRegistryCluster` rows without selecting an
owner.

Minimum join axes:

```text
context_surface
operation_role
shape_signature
borrow_axis
type_transport_axis
return_family
verifier_or_oracle_state
public_or_private_surface
native_authority_hint
```

Expected context surfaces:

```text
binding_context
metadata_context
type_context
core_context
scope_context
compilation_context
aggregate_context
unknown_context_surface
```

## Decision Rule

```text
if exactly one context surface subcluster is evidence-backed and selectable:
  decision = SelectProjectionPolicySubcluster
  selected_next_card = <CONTEXT-SURFACE>-PROJECTION-POLICY-001

elif exactly one context surface lacks native authority evidence:
  decision = SelectNativeAuthorityEvidenceRepair
  selected_next_card = <CONTEXT-SURFACE>-NATIVE-AUTHORITY-EVIDENCE-REPAIR-001

elif exactly one context surface needs type transport policy:
  decision = SelectTypeTransportPolicy
  selected_next_card = <CONTEXT-SURFACE>-TYPE-TRANSPORT-POLICY-001

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
input_context_registry_cluster_count = 114
all_context_registry_items_joined_exactly_once = 1
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
input_context_registry_cluster_count = 114
subcluster_count = 43
selection_eligible_subcluster_count = 22
decision = KeepStopped
reason = AmbiguousContextSurfaceProjectionSubclusters
```

## Follow-Up Queue

If this card does not produce exactly one executable owner, continue with the
remaining broad owner clusters in this order:

```text
1. MIRBUILDER-MISSING-PROJECTION-POLICY-CALL-EMIT-SSA-CLUSTER-001
2. MIRBUILDER-CRATE-WIDE-SURFACE-REPORT-OWNER-CLUSTER-FIELD-001
```

## Stop Conditions

Stop for consultation if this join would require:

```text
manual family selection
full context selfhost claim
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
