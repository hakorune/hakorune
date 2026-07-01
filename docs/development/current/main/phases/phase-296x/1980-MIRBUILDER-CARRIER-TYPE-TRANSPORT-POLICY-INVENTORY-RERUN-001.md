# 1980 - MIRBUILDER-CARRIER-TYPE-TRANSPORT-POLICY-INVENTORY-RERUN-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-TRANSPORT-POLICY-INVENTORY-RERUN-001
```

## Purpose

Re-inventory the `bridge_gap::carrier_type_transport_only` cluster selected by
`MIRBUILDER-BRIDGE-BLOCKED-GAP-CLUSTER-RESOLUTION-001`.

This card is a resolver. It does not select a family, emit Hako, materialize a
native seed, or weaken strict converter rules. Its job is to determine whether
the 23 single-axis carrier/type transport gaps can be routed to exactly one
machine-derived policy lane.

## Input Authority

```text
bridge gap cluster resolution:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-bridge-blocked-gap-cluster-resolution-v0.json

strict candidate selection:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-strict-converter-emission-native-seed-candidate-selection-v0.json

native-owner seed rerun:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-007-v0.json

precedent inventory:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-transport-policy-inventory-v0.json
```

## Resolution Rule

```text
1. Consume only candidates in bridge_gap::carrier_type_transport_only.
2. Reject mixed borrow+carrier gaps; those remain deferred.
3. Partition candidates by machine-derived carrier/type evidence:
   ResultCarrierNeedsVerifier
   ReturnedIteratorNeedsPolicy
   MissingTypeTransport
   ConstructorCarrier
   KnownOptionCarrier
   KnownVecCarrier
   GenericCarrierPolicyCandidate
4. If exactly one evidence-backed lane is eligible, select that policy lane.
5. If evidence is insufficient to derive a lane, select a shape/evidence
   inventory task rather than choosing by owner name or cluster size.
6. If lanes remain ambiguous, keep the Source Selfhost design stop active.
```

## Expected Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-transport-policy-inventory-rerun-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_transport_policy_inventory_rerun.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_type_transport_policy_inventory_rerun_guard.sh
```

## Result

```text
carrier_type_transport_only_count = 23
mixed_borrow_carrier_type_transport_count = 1
transport_notes_missing_count = 4
eligible_policy_lane_count = 4

decision:
  SelectCarrierTypeTransportEvidenceInventory

reason_token:
  CarrierTypeTransportEvidenceRequiresInventoryBeforePolicyLane

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-TRANSPORT-EVIDENCE-INVENTORY-001
```

The 23 carrier/type-only candidates do not form exactly one policy lane. Four
candidate rows also lack `transport_notes`, so selecting Result / Option / Vec
/ GenericCarrier policy directly would be evidence-thin. The next card must
inventory transport evidence before selecting a carrier/type transport policy.

## Acceptance

```text
bridge_gap_cluster_resolution_consumed = 1
selected_cluster_id = bridge_gap::carrier_type_transport_only
carrier_type_transport_only_count = 23
mixed_borrow_carrier_type_transport_count = 1
mixed_gap_deferred = 1

every_carrier_type_transport_only_candidate_classified_once = 1
classification_uses_machine_evidence = 1
owner_name_as_transport_policy = 0
cluster_size_as_proof = 0
manual_carrier_selection = 0

decision =
  SelectCarrierTypeTransportEvidenceInventory
```

## Non-Claims

```text
manual_family_selection = 0
manual_shape_selection = 0
manual_axis_selection = 0
manual_carrier_selection = 0
cluster_size_as_proof = 0
coverage_percentage_as_proof = 0
generated_artifact_as_native_edit_authority = 0
hako_generation = 0
hako_adopted_decision = 0
native_seed_materialization = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```
