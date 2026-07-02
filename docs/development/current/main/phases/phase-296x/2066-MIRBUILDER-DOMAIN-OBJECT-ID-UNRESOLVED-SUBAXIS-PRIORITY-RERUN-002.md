# 2066 - MIRBUILDER-DOMAIN-OBJECT-ID-UNRESOLVED-SUBAXIS-PRIORITY-RERUN-002

## Token

```text
MIRBUILDER-DOMAIN-OBJECT-ID-UNRESOLVED-SUBAXIS-PRIORITY-RERUN-002
```

## Purpose

Consume `DomainObjectIdSubaxisMechanicalSelectorV1` and rerun unresolved
non-ID DomainObject/Id subaxis priority selection.

This card evaluates the proof tuple defined by 2065. It does not select a
subaxis by row count, owner name, source path, route membership, or
implementation convenience.

## Result

```text
candidate_subaxis_count = 5
guard_clean_candidate_count = 5
proof_tuple_complete_candidate_count = 0
selection_eligible_subaxis_count = 0

decision:
  KeepStopped

reason_token:
  NoExactlyOneDomainObjectIdSubaxisMechanicalCandidate

selected_next_card:
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
```

## Interpretation

All five subaxes are guard-clean for opening a policy-basis card, but none has
typed dependency-root authority or prior closed-lane continuation authority.
Therefore no subaxis is machine-selected.

The lane is back at design consultation. The next step must define stronger
typed dependency evidence, define a valid closed-lane continuation source, or
return to the wider route selector.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-domain-object-id-unresolved-subaxis-priority-rerun-002-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_domain_object_id_unresolved_subaxis_priority_rerun_002.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_domain_object_id_unresolved_subaxis_priority_rerun_002_guard.sh
```

## Non-Claims

```text
manual_family_selection = 0
manual_shape_selection = 0
manual_axis_selection = 0
manual_carrier_selection = 0
manual_subaxis_selection = 0
hardcoded_subaxis_priority = 0
row_count_as_proof = 0
domain_object_count_as_proof = 0
cluster_size_as_proof = 0
coverage_percentage_as_proof = 0
owner_name_as_proof = 0
source_path_as_authority = 0
route_membership_alone_as_proof = 0
convenience_as_proof = 0
generated_artifact_as_native_edit_authority = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```
