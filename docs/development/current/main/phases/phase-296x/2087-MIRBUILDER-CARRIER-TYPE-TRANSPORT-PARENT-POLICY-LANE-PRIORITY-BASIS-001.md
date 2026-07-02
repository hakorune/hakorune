# 2087 - MIRBUILDER-CARRIER-TYPE-TRANSPORT-PARENT-POLICY-LANE-PRIORITY-BASIS-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-TRANSPORT-PARENT-POLICY-LANE-PRIORITY-BASIS-001
```

## Purpose

Define the mechanical selector basis for the deferred carrier/type parent
policy lanes after `SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-009` parked the
remaining-axis branch.

This card does not select Result, Option, SelfConstructor, or Collection. It
only defines what proof a rerun may use.

## Previous State

```text
source_selfhost_blocker =
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001

basis_009_decision =
  SelectCarrierTypeParentPolicyLanePriorityBasis

basis_009_reason_token =
  CarrierTypeRemainingLaneParkedReturnToParentPolicyLanePriority

carrier_type_remaining_lane_parked = 1
direct_parent_policy_candidate_selection = 0
```

## Candidate Parent Policy Lanes

```text
ResultCarrierPolicyCandidate
OptionCarrierPolicyCandidate
SelfConstructorTransportPolicyCandidate
CollectionCarrierPolicyCandidate
```

Counts are diagnostic only. Historical Result policy work is not direct
selection authority.

## Selector Rule

```text
rule:
  CarrierTypeParentPolicyLaneMechanicalSelectorV1

select only if exactly one parent policy lane has:
  scope_eligible
  guard_clean_authority
  evidence_inventory_completeness
  one of:
    dependency_root_authority
    prior_closed_policy_continuation_authority
    current_policy_contract_readiness
```

`guard_clean_authority` and `evidence_inventory_completeness` are required
filters, not priority signals.

## Result

```text
candidate_parent_policy_lane_count = 4
scope_eligible_parent_policy_lane_count = 4
basis_selection_eligible_parent_policy_lane_count = 0
basis_selects_concrete_parent_policy_candidate = 0

decision:
  SelectCarrierTypeParentPolicyLanePriorityRerun

reason_token:
  CarrierTypeParentPolicyLanePriorityBasisDefined

selected_parent_policy_candidate:
  null

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-TRANSPORT-PARENT-POLICY-LANE-PRIORITY-RERUN-001
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-transport-parent-policy-lane-priority-basis-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_transport_parent_policy_lane_priority_basis.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_type_transport_parent_policy_lane_priority_basis_guard.sh
```

## Forbidden

```text
direct Result / Option / SelfConstructor / Collection selection
row_count_as_proof
return_type_count_as_proof
owner_name_as_proof
source_path_as_authority
route_membership_alone_as_proof
historical_preference_as_proof
result_history_as_direct_selection_proof
hardcoded_parent_policy_priority
```

## Non-Claims

```text
source_selfhost_claim = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
accepted_typed_dependency_edge_materialized = 0
direct_parent_policy_candidate_selection = 0
```
