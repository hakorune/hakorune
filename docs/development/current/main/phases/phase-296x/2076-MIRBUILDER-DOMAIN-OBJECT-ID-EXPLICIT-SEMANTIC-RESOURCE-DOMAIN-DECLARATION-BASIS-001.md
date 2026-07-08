# 2076 - MIRBUILDER-DOMAIN-OBJECT-ID-EXPLICIT-SEMANTIC-RESOURCE-DOMAIN-DECLARATION-BASIS-001

## Token

```text
MIRBUILDER-DOMAIN-OBJECT-ID-EXPLICIT-SEMANTIC-RESOURCE-DOMAIN-DECLARATION-BASIS-001
```

## Purpose

Define the authority requirements for turning type-identity rows into
resource-domain registry rows.

This card does not assign a domain subaxis to any type.

This is the consultation-approved basis-only move after 2064. It creates the
input vocabulary for dependency-root authority, but it does not prove any
dependency root and does not select among the five unresolved non-ID
DomainObject/Id subaxes.

## Result

```text
candidate_registry_row_count = 42
resource_domain_declaration_ready_count = 0
registry_ready_row_count = 0
accepted_typed_dependency_edge_count = 0

decision:
  SelectSemanticResourceDomainDeclarationInventory

selected_next_card:
  MIRBUILDER-DOMAIN-OBJECT-ID-SEMANTIC-RESOURCE-DOMAIN-DECLARATION-INVENTORY-001
```

## Interpretation

Type identity is stable evidence, but it is not resource-domain authority.
The next card must inventory explicit semantic declarations without inferring
from type names, source paths, observed subaxis sets, owner names, or counts.

Positive claim scope:

```text
explicit_semantic_resource_domain_declaration_basis = 1
resource_domain_declaration_requirements_defined = 1
dependency_root_authority_input_defined = 1
subaxis_selection_ready_for_inventory = 1
```

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_domain_object_id_lane_guard.sh \
  explicit_semantic_resource_domain_declaration_basis
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-domain-object-id-explicit-semantic-resource-domain-declaration-basis-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_domain_object_id_explicit_semantic_resource_domain_declaration_basis.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_domain_object_id_lane_guard.sh
```

## Non-Claims

```text
source_selfhost_claim = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
accepted_typed_dependency_edge_materialized = 0
selected_domain_subaxis = null
dependency_root_authority_proven = 0
prior_closed_lane_continuation_authority_proven = 0
manual_subaxis_selection = 0
manual_axis_selection = 0
manual_family_selection = 0
manual_shape_selection = 0
manual_carrier_selection = 0
manual_type_to_subaxis_assignment = 0
return_type_string_to_subaxis_mapping = 0
source_path_as_policy_authority = 0
observed_subaxis_set_as_policy_authority = 0
row_count_as_proof = 0
domain_object_count_as_proof = 0
cluster_size_as_proof = 0
coverage_percentage_as_proof = 0
owner_name_as_proof = 0
shape_signature_as_proof = 0
route_membership_alone_as_proof = 0
implementation_convenience_as_proof = 0
self_signed_taxonomy = 0
generated_artifact_as_native_edit_authority = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```
