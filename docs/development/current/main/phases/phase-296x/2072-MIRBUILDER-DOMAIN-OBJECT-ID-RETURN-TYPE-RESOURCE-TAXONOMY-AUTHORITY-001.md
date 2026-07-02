# 2072 - MIRBUILDER-DOMAIN-OBJECT-ID-RETURN-TYPE-RESOURCE-TAXONOMY-AUTHORITY-001

## Token

```text
MIRBUILDER-DOMAIN-OBJECT-ID-RETURN-TYPE-RESOURCE-TAXONOMY-AUTHORITY-001
```

## Purpose

Define `ReturnTypeResourceTaxonomyAuthorityV1` without allowing self-signed
taxonomy or return-type string policy.

## Result

```text
taxonomy_entry_count = 0
resolved_type_decl_ref_count = 0
resource_taxonomy_join_ready_count = 0
edge_ready_return_type_count = 0
accepted_typed_dependency_edge_count = 0

decision:
  KeepStopped

reason_token:
  ReturnTypeResourceTaxonomyAuthorityEntriesMissing
```

## Interpretation

The authority rule is defined, but no independent stable type/resource registry
is available. Therefore this card cannot produce taxonomy rows, return-type
reference joins, or accepted typed dependency edges.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-domain-object-id-return-type-resource-taxonomy-authority-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_domain_object_id_return_type_resource_taxonomy_authority.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_domain_object_id_return_type_resource_taxonomy_authority_guard.sh
```

## Non-Claims

```text
return_type_string_to_subaxis_mapping = 0
return_type_string_as_policy_authority = 0
observed_domain_subaxis_set_as_proof = 0
self_signed_taxonomy = 0
accepted_typed_dependency_edge_materialized = 0
manual_subaxis_selection = 0
hardcoded_subaxis_priority = 0
row_count_as_proof = 0
owner_name_as_proof = 0
shape_signature_as_proof = 0
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
