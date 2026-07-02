# 2076 - MIRBUILDER-DOMAIN-OBJECT-ID-EXPLICIT-SEMANTIC-RESOURCE-DOMAIN-DECLARATION-BASIS-001

## Token

```text
MIRBUILDER-DOMAIN-OBJECT-ID-EXPLICIT-SEMANTIC-RESOURCE-DOMAIN-DECLARATION-BASIS-001
```

## Purpose

Define the authority requirements for turning type-identity rows into
resource-domain registry rows.

This card does not assign a domain subaxis to any type.

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
manual_subaxis_selection = 0
manual_type_to_subaxis_assignment = 0
return_type_string_to_subaxis_mapping = 0
source_path_as_policy_authority = 0
observed_subaxis_set_as_policy_authority = 0
row_count_as_proof = 0
owner_name_as_proof = 0
shape_signature_as_proof = 0
route_membership_alone_as_proof = 0
self_signed_taxonomy = 0
```
