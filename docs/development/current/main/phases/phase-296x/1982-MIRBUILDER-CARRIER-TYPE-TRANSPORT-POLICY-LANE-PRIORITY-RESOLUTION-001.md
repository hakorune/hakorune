# 1982 - MIRBUILDER-CARRIER-TYPE-TRANSPORT-POLICY-LANE-PRIORITY-RESOLUTION-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-TRANSPORT-POLICY-LANE-PRIORITY-RESOLUTION-001
```

## Purpose

Select the next carrier/type transport policy lane after evidence inventory.

The input has five normalized lane buckets, so this card uses the established
carrier/type inventory lane order instead of selecting by owner name, candidate
count, or taste.

## Stable Lane Priority

```text
1. ResultCarrierVerifierPolicyCandidate
2. OptionCarrierPolicyCandidate
3. VecOrArrayCarrierPolicyCandidate
4. GenericCarrierPolicyCandidate
```

`KnownTypeTransportNoCarrierPolicy` is excluded from policy-lane selection
because it is a no-policy-needed closeout bucket, not a carrier policy lane.

## Result

```text
input_candidate_count = 23
policy_lane_count = 5
eligible_policy_lane_count = 4
known_type_transport_no_policy_count = 2

selected_policy_lane:
  ResultCarrierVerifierPolicyCandidate

selected_policy_lane_candidate_count = 3

selected_next_card:
  MIRBUILDER-RESULT-CARRIER-VERIFIER-POLICY-001
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-transport-policy-lane-priority-resolution-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_transport_policy_lane_priority_resolution.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_type_transport_policy_lane_priority_resolution_guard.sh
```

## Non-Claims

```text
manual_family_selection = 0
manual_shape_selection = 0
manual_axis_selection = 0
manual_carrier_selection = 0
owner_name_as_transport_policy = 0
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
