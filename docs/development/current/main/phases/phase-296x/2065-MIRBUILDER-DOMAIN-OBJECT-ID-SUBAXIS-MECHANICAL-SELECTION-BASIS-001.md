# 2065 - MIRBUILDER-DOMAIN-OBJECT-ID-SUBAXIS-MECHANICAL-SELECTION-BASIS-001

## Token

```text
MIRBUILDER-DOMAIN-OBJECT-ID-SUBAXIS-MECHANICAL-SELECTION-BASIS-001
```

## Purpose

Define the machine-checkable selector basis for unresolved non-ID
DomainObject/Id subaxes after 2064 kept Source Selfhost stopped.

This card does not select PlanRecipe, MIR, AST, Context/Span, or Other. It
only defines the proof tuple that a follow-up rerun must use before local
mechanical selection may choose exactly one subaxis.

## Local Authority

```text
local_selection_authority = LocalMechanicalSelectorAuthorityV1
worker_inventory = consumed
worker_inventory_scope = read_only_current_fixtures_cards_ledgers
```

Local authority is used here to formalize the consultation result as a
selector basis. It is not used to select a semantic subaxis.

## Selector Basis

```text
selector_rule:
  DomainObjectIdSubaxisMechanicalSelectorV1

select only if:
  exactly_one(candidate where proof_tuple_complete == true)

proof_tuple_complete requires:
  guard_clean_authority
  AND (
    dependency_root_authority
    OR prior_closed_lane_continuation_authority
  )
```

Allowed proof axes:

```text
dependency_root_authority:
  typed dependency edges prove this subaxis is the unique root needed before
  at least one other unresolved subaxis can become policy-ready.

prior_closed_lane_continuation_authority:
  fixture-proven closed-lane consumption links this subaxis to an already
  closed lane by source_id, owner_edge, shape_signature, or semantic resource
  continuity.

guard_clean_authority:
  the subaxis can open a policy-basis card without requiring native seed
  materialization, Hako generation, HakoAdopted decision, Source Selfhost
  claim, runtime fallback, new backend route, new ABI, new Python
  SemanticProjector, or runner semantic ownership.
```

Forbidden interpretations:

```text
PlanRecipe > MIR > AST > Context/Span > Other
largest row count
owner name
source path
route membership alone
implementation convenience
```

## Result

```text
candidate_subaxis_count = 5
selection_eligible_subaxis_count = 0

decision:
  SelectDomainObjectIdSubaxisPriorityRerun

reason_token:
  DomainObjectIdSubaxisMechanicalSelectorBasisDefined

selected_next_card:
  MIRBUILDER-DOMAIN-OBJECT-ID-UNRESOLVED-SUBAXIS-PRIORITY-RERUN-002
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-domain-object-id-subaxis-mechanical-selection-basis-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_domain_object_id_subaxis_mechanical_selection_basis.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_domain_object_id_subaxis_mechanical_selection_basis_guard.sh
```

## Recovery

If the follow-up rerun cannot prove exactly one guard-clean candidate:

```text
reason_token:
  NoExactlyOneDomainObjectIdSubaxisMechanicalCandidate

selected_next_card:
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
```

If multiple subaxes satisfy the proof tuple:

```text
reason_token:
  MultipleDomainObjectIdSubaxisMechanicalCandidates
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
