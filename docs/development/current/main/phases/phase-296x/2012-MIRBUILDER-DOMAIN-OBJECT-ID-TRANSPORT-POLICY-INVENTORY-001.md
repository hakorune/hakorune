# 2012 - MIRBUILDER-DOMAIN-OBJECT-ID-TRANSPORT-POLICY-INVENTORY-001

## Token

```text
MIRBUILDER-DOMAIN-OBJECT-ID-TRANSPORT-POLICY-INVENTORY-001
```

## Purpose

Classify the 116 `DomainObjectOrIdTransportAxis` rows into smaller
machine-derived domain transport subaxes.

This card is an inventory/resolver. It does not select a domain object layout
policy by count and does not materialize Hako.

## Result

```text
domain_object_id_input_count = 116

domain_subaxis_counts:
  IdScalarDomainTransportAxis = 31
  PlanRecipeDomainTransportAxis = 48
  MirDomainTransportAxis = 19
  AstNodeDomainTransportAxis = 14
  ContextOrSpanDomainTransportAxis = 3
  OtherDomainObjectTransportAxis = 1

decision:
  SelectIdScalarDomainTransportPolicy

selected_next_card:
  MIRBUILDER-ID-SCALAR-DOMAIN-TRANSPORT-POLICY-001
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-domain-object-id-transport-policy-inventory-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_domain_object_id_transport_policy_inventory.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_domain_object_id_transport_policy_inventory_guard.sh
```

## Non-Claims

```text
manual_family_selection = 0
manual_shape_selection = 0
manual_axis_selection = 0
manual_carrier_selection = 0
domain_object_count_as_proof = 0
cluster_size_as_proof = 0
coverage_percentage_as_proof = 0
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
