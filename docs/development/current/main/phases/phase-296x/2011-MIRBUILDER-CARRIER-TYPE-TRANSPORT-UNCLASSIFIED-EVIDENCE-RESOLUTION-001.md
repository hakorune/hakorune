# 2011 - MIRBUILDER-CARRIER-TYPE-TRANSPORT-UNCLASSIFIED-EVIDENCE-RESOLUTION-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-TRANSPORT-UNCLASSIFIED-EVIDENCE-RESOLUTION-001
```

## Purpose

Resolve the 130 carrier/type transport evidence rows left unclassified by
`MIRBUILDER-CARRIER-TYPE-TRANSPORT-EVIDENCE-INVENTORY-RERUN-002`.

This card names evidence axes. It does not select a policy by return-type
count, cluster size, or manual carrier preference.

## Result

```text
unclassified_input_count = 130
resolved_axis_count = 6

axis_counts:
  DomainObjectOrIdTransportAxis = 116
  ProductTupleTransportAxis = 9
  CollectionCarrierTransportAxis = 2
  IteratorOrBorrowTypeTransportAxis = 1
  ScalarKnownTransportAxis = 1
  OpaqueTypeTransportAxis = 1

decision:
  SelectDomainObjectIdTransportPolicyInventory

selected_next_card:
  MIRBUILDER-DOMAIN-OBJECT-ID-TRANSPORT-POLICY-INVENTORY-001
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-transport-unclassified-evidence-resolution-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_transport_unclassified_evidence_resolution.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_type_transport_unclassified_evidence_resolution_guard.sh
```

## Non-Claims

```text
manual_family_selection = 0
manual_shape_selection = 0
manual_axis_selection = 0
manual_carrier_selection = 0
return_type_count_as_proof = 0
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
