# 2009 - MIRBUILDER-CARRIER-TYPE-TRANSPORT-POLICY-INVENTORY-RERUN-002

## Token

```text
MIRBUILDER-CARRIER-TYPE-TRANSPORT-POLICY-INVENTORY-RERUN-002
```

## Purpose

Inventory carrier/type transport lanes for the remaining projection clusters
blocked by `TypeTransportMissing`.

This card consumes `MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-V3`
and does not choose a carrier policy by return-type count. Multiple transport
lanes are present, so the next step is evidence inventory.

## Result

```text
type_transport_missing_cluster_count = 76
type_transport_missing_item_count = 944
eligible_policy_lane_count = 4

policy_lane_candidate_counts:
  ResultCarrierPolicyCandidate = 557
  OptionCarrierPolicyCandidate = 166
  SelfConstructorTransportPolicyCandidate = 56
  CollectionCarrierPolicyCandidate = 35
  CarrierTypeTransportEvidenceInventoryRequired = 130

decision:
  SelectCarrierTypeTransportEvidenceInventoryRerun002

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-TRANSPORT-EVIDENCE-INVENTORY-RERUN-002
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-transport-policy-inventory-rerun-002-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_transport_policy_inventory_rerun_002.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_type_transport_policy_inventory_rerun_002_guard.sh
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
