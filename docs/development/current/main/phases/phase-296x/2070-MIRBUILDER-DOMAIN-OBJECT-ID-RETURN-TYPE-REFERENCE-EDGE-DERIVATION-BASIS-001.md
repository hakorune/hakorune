# 2070 - MIRBUILDER-DOMAIN-OBJECT-ID-RETURN-TYPE-REFERENCE-EDGE-DERIVATION-BASIS-001

## Token

```text
MIRBUILDER-DOMAIN-OBJECT-ID-RETURN-TYPE-REFERENCE-EDGE-DERIVATION-BASIS-001
```

## Purpose

Define when a `ReturnTypeFieldReference` may become a typed dependency edge.

## Result

```text
return_type_reference_count = 85
distinct_return_type_count = 44
resource_taxonomy_entry_count = 0
edge_ready_return_type_count = 0

decision:
  SelectReturnTypeResourceTaxonomyInventory

selected_next_card:
  MIRBUILDER-DOMAIN-OBJECT-ID-RETURN-TYPE-RESOURCE-TAXONOMY-INVENTORY-001
```

## Interpretation

`return_type` is concrete evidence, but it is not a dependency edge by itself.
Return type names must not be mapped to subaxes by prefix, string contains,
row count, owner name, source path, route membership, or lexical ordering.

A return type can produce an edge only after a typed resource taxonomy declares
the prerequisite subaxis and the dependent source row supplies a concrete source
reference.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-domain-object-id-return-type-reference-edge-derivation-basis-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_domain_object_id_return_type_reference_edge_derivation_basis.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_domain_object_id_return_type_reference_edge_derivation_basis_guard.sh
```

## Non-Claims

```text
return_type_reference_is_dependency_edge_by_itself = 0
return_type_name_to_subaxis_map_allowed = 0
hardcoded_return_type_priority_allowed = 0
manual_subaxis_selection = 0
hardcoded_subaxis_priority = 0
row_count_as_proof = 0
owner_name_as_proof = 0
source_path_as_authority = 0
route_membership_alone_as_proof = 0
source_selfhost_claim = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```
