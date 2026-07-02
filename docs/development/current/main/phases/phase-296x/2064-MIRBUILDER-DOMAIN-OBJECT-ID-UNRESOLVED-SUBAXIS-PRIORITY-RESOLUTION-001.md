# 2064 - MIRBUILDER-DOMAIN-OBJECT-ID-UNRESOLVED-SUBAXIS-PRIORITY-RESOLUTION-001

## Token

```text
MIRBUILDER-DOMAIN-OBJECT-ID-UNRESOLVED-SUBAXIS-PRIORITY-RESOLUTION-001
```

## Purpose

Consume the 85 unresolved non-ID DomainObject/Id rows from rerun 002 and
determine whether exactly one subaxis can be selected by machine-derived
authority.

This resolver does not select a subaxis by row count, owner name, route
membership, source path, or implementation convenience.

## Local Authority

```text
local_selection_authority = LocalMechanicalSelectorAuthorityV1
worker_inventory = consumed
worker_inventory_scope = read_only_current_fixtures_cards_ledgers
```

Local authority is used only to confirm that no machine-derived subaxis
priority exists. It is not used to choose PlanRecipe, MIR, AST, Context/Span,
or Other.

## Result

```text
unresolved_non_id_domain_row_count = 85
candidate_subaxis_count = 5
selection_eligible_subaxis_count = 0

domain_subaxis_counts:
  PlanRecipeDomainTransportAxis = 48
  MirDomainTransportAxis = 19
  AstNodeDomainTransportAxis = 14
  ContextOrSpanDomainTransportAxis = 3
  OtherDomainObjectTransportAxis = 1

decision:
  KeepStopped

reason_token:
  NoMachineDerivedDomainObjectIdUnresolvedSubaxisPriority

selected_next_card:
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
```

## Recovery

```text
recovery = DesignConsultationRequired

question =
  Which non-ID DomainObject/Id subaxis may define the next policy basis without
  using row count, owner name, route membership, or convenience as proof?
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-domain-object-id-unresolved-subaxis-priority-resolution-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_domain_object_id_unresolved_subaxis_priority_resolution.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_domain_object_id_unresolved_subaxis_priority_resolution_guard.sh
```

## Non-Claims

```text
manual_family_selection = 0
manual_shape_selection = 0
manual_axis_selection = 0
manual_carrier_selection = 0
manual_subaxis_selection = 0
row_count_as_proof = 0
owner_name_as_proof = 0
domain_object_count_as_proof = 0
cluster_size_as_proof = 0
coverage_percentage_as_proof = 0
route_membership_alone_as_proof = 0
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
