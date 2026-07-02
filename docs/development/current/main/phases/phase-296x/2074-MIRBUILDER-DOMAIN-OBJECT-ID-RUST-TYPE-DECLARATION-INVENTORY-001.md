# 2074 - MIRBUILDER-DOMAIN-OBJECT-ID-RUST-TYPE-DECLARATION-INVENTORY-001

## Token

```text
MIRBUILDER-DOMAIN-OBJECT-ID-RUST-TYPE-DECLARATION-INVENTORY-001
```

## Purpose

Materialize a read-only Rust type declaration inventory for unresolved non-ID
DomainObject/Id return types. This card produces type identity evidence only.

## Result

```text
return_type_reference_count = 85
distinct_return_type_count = 44
declared_resource_domain_subaxis_ready_count = 0
registry_ready_row_count = 0
accepted_typed_dependency_edge_count = 0

decision:
  SelectStableTypeResourceRegistryAuthorityRerun

selected_next_card:
  MIRBUILDER-DOMAIN-OBJECT-ID-STABLE-TYPE-RESOURCE-REGISTRY-AUTHORITY-RERUN-001
```

## Interpretation

Rust declarations can provide stable `type_decl_ref` and declaration hashes.
They do not declare a resource domain subaxis. The next rerun must decide
whether type identity rows are sufficient for registry authority or whether a
separate explicit semantic resource-domain declaration is required.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-domain-object-id-rust-type-declaration-inventory-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_domain_object_id_rust_type_declaration_inventory.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_domain_object_id_rust_type_declaration_inventory_guard.sh
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
return_type_string_as_policy_authority = 0
source_path_as_policy_authority = 0
observed_domain_subaxis_set_as_policy_authority = 0
row_count_as_proof = 0
owner_name_as_proof = 0
shape_signature_as_proof = 0
route_membership_alone_as_proof = 0
```
