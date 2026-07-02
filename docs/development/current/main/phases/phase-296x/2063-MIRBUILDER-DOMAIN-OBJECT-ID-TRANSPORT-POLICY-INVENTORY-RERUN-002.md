# 2063 - MIRBUILDER-DOMAIN-OBJECT-ID-TRANSPORT-POLICY-INVENTORY-RERUN-002

## Token

```text
MIRBUILDER-DOMAIN-OBJECT-ID-TRANSPORT-POLICY-INVENTORY-RERUN-002
```

## Purpose

Consume the 2062 `DomainObjectOrIdTransportAxis` selection and build a full
116-row DomainObject/Id source-id ledger.

This rerun must not repeat the historical 2012 behavior of selecting
`IdScalarDomainTransportAxis` just because it is present. The current 31 ID
scalar rows exactly overlap the previous ID scalar directability lane by
`source_id`, so this card first partitions them as an already processed closed
lane before any unresolved non-ID subaxis is considered.

## Worker Inventory

```text
worker_inventory = consumed
worker_inventory_scope = read_only_current_fixtures_cards_ledgers
```

Durable findings:

```text
DomainObjectOrIdTransportAxis = 116
IdScalarDomainTransportAxis = 31
id_scalar_source_id_overlap_with_previous_directability_rerun = 31
new_id_scalar_source_id_count = 0
unresolved_non_id_domain_row_count = 85

unresolved_non_id_domain_subaxis_counts:
  PlanRecipeDomainTransportAxis = 48
  MirDomainTransportAxis = 19
  AstNodeDomainTransportAxis = 14
  ContextOrSpanDomainTransportAxis = 3
  OtherDomainObjectTransportAxis = 1
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-domain-object-id-transport-policy-inventory-rerun-002-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_domain_object_id_transport_policy_inventory_rerun_002.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_domain_object_id_transport_policy_inventory_rerun_002_guard.sh
```

## Decision Rule

```text
if new_id_scalar_source_id_count > 0:
  selected_next_card =
    MIRBUILDER-ID-SCALAR-NEWLY-UNCOVERED-DOMAIN-TRANSPORT-RESOLUTION-001

elif unresolved_non_id_domain_row_count > 0:
  selected_next_card =
    MIRBUILDER-DOMAIN-OBJECT-ID-UNRESOLVED-SUBAXIS-PRIORITY-RESOLUTION-001

else:
  selected_next_card =
    SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
```

This rule is mechanical partitioning only. It does not choose PlanRecipe, MIR,
AST, Context/Span, or Other by row count.

## Result

```text
domain_object_id_input_count = 116
closed_id_scalar_row_count = 31
new_id_scalar_source_id_count = 0
unresolved_non_id_domain_row_count = 85

decision:
  SelectUnresolvedSubaxisPriorityResolution

reason_token:
  ClosedIdScalarLaneConsumedAndNonIdDomainRowsRemain

selected_next_card:
  MIRBUILDER-DOMAIN-OBJECT-ID-UNRESOLVED-SUBAXIS-PRIORITY-RESOLUTION-001
```

## Acceptance

```text
carrier_type_transport_unclassified_evidence_resolution_002_consumed = 1
domain_object_id_input_count = 116
full_source_id_ledger_present = 1

id_scalar_row_count = 31
previous_id_scalar_directability_row_count = 31
id_scalar_source_id_overlap_with_previous_directability_rerun = 31
new_id_scalar_source_id_count = 0
closed_id_scalar_lane_consumed = 1

unresolved_non_id_domain_row_count = 85
unresolved_non_id_subaxis_counts_present = 1

manual_subaxis_selection = 0
return_type_count_as_proof = 0
domain_object_count_as_proof = 0
cluster_size_as_proof = 0
coverage_percentage_as_proof = 0
route_membership_alone_as_proof = 0
```

## Next Task Order

```text
1. MIRBUILDER-DOMAIN-OBJECT-ID-TRANSPORT-POLICY-INVENTORY-RERUN-002
   Partition closed ID scalar rows from unresolved non-ID domain rows.

2. MIRBUILDER-DOMAIN-OBJECT-ID-UNRESOLVED-SUBAXIS-PRIORITY-RESOLUTION-001
   Select a non-ID subaxis only by machine-derived authority.

3. MIRBUILDER-DOMAIN-OBJECT-ID-SELECTED-SUBAXIS-POLICY-BASIS-001
   Conditional on exactly one selected subaxis.
```

## Non-Claims

```text
manual_family_selection = 0
manual_shape_selection = 0
manual_axis_selection = 0
manual_carrier_selection = 0
manual_subaxis_selection = 0
return_type_count_as_proof = 0
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
