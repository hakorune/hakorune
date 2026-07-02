# 2089 - MIRBUILDER-CARRIER-TYPE-PARENT-POLICY-LANE-EVIDENCE-SOURCE-DISCOVERY-BASIS-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-PARENT-POLICY-LANE-EVIDENCE-SOURCE-DISCOVERY-BASIS-001
```

## Purpose

Define non-self-signed evidence-source authority for all four carrier/type
parent policy lanes before selecting any parent policy candidate.

This avoids selecting `ResultCarrierPolicyCandidate` by historical contract
presence, row count, or hardcoded priority.

## Previous State

```text
latest:
  MIRBUILDER-CARRIER-TYPE-TRANSPORT-PARENT-POLICY-LANE-PRIORITY-RERUN-001

previous_reason_token:
  NoCarrierTypeParentPolicyLaneMechanicalCandidate

candidate_parent_policy_lane_count = 4
selection_eligible_parent_policy_lane_count = 0
historical_result_contract_present = 1
historical_result_contract_as_direct_selection_proof = 0
```

## Allowed Evidence Source Kinds

```text
CurrentReusablePolicyContract
CurrentVerifierContractCompatibility
StableParentPolicyDependencyRoot
PriorClosedPolicyContinuationContract
CrossLanePolicyHandoffContract
```

Every accepted source must have a stable source/contract/dependency ID, a
stable proof hash, and a join to a current parent policy lane.

## Forbidden Evidence Source Kinds

```text
RowCount
ReturnTypeCount
HistoricalPreference
ResultHistoryAlone
OwnerNameInference
SourcePathOrModuleInference
RouteMembershipAlone
LexicalOrder
HardcodedParentPolicyPriority
SelfSignedFixture
```

## Result

```text
candidate_parent_policy_lane_count = 4
allowed_source_kind_count = 5
accepted_parent_policy_evidence_source_count = 0
parent_policy_candidate_selection = 0

decision:
  SelectParentPolicyLaneEvidenceSourceDiscoveryInventory

reason_token:
  ParentPolicyLaneEvidenceSourceDiscoveryBasisDefined

selected_parent_policy_candidate:
  null

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-PARENT-POLICY-LANE-EVIDENCE-SOURCE-DISCOVERY-INVENTORY-001
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-parent-policy-lane-evidence-source-discovery-basis-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_parent_policy_lane_evidence_source_discovery_basis.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_type_parent_policy_lane_evidence_source_discovery_basis_guard.sh
```

## Non-Claims

```text
source_selfhost_claim = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
parent_policy_candidate_selection = 0
direct_parent_policy_candidate_selection = 0
result_history_as_direct_selection_proof = 0
```
