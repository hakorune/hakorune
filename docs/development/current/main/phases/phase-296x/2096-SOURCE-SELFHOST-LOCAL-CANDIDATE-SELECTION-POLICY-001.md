# 2096 - SOURCE-SELFHOST-LOCAL-CANDIDATE-SELECTION-POLICY-001

## Token

```text
SOURCE-SELFHOST-LOCAL-CANDIDATE-SELECTION-POLICY-001
```

## Purpose

Reduce unnecessary external design consultation by separating candidate
selection from authority-definition changes.

This policy does not select a semantic lane. It defines how Source Selfhost
uses read-only worker inventory for local mechanical candidate selection.

## Policy

```text
worker_inventory_first = 1
external_consultation_only_for_new_authority = 1
local_mechanical_selection_requires_exactly_one = 1
zero_or_multiple_keeps_stopped = 1
parked_lane_must_record_reentry_condition = 1
no_consultation_for_counting = 1
two_turn_same_missing_authority_stops_micro_basis = 1
```

## Local Candidate Selection Protocol

```text
1. spawn/read-only worker inventory
2. record candidate_set
3. record selector_rule
4. record allowed_proof_axes
5. record forbidden_proof_axes
6. record proof_tuple_per_candidate
7. record selection_eligible_count
8. select locally only if exactly one
9. KeepStopped locally if zero or multiple
```

## External Consultation Gate

Ask externally only if one is true:

```text
new_proof_axis_needed
existing_forbidden_axis_needs_reconsideration
new_authority_source_kind_introduced
source_selfhost_or_native_seed_or_hako_adopted_boundary_approached
selector_rule_semantics_change
local_worker_finds_fixture_card_contradiction
```

Do not ask externally only because:

```text
row_count_differs
cluster_count_differs
candidate_labels_look_important
one_option_feels_more_central
historical_card_exists
```

## KeepStopped Reentry Contract

Every `KeepStopped` or parked lane must record:

```text
park_reason_token
exact_blocking_counts
forbidden_axes_held_at_zero
new_evidence_that_allows_reentry
selected_next_card_or_design_stop_pointer
```

## Two-Turn Rule

If a lane produces two consecutive `KeepStopped` results with the same selector
family, forbidden axes, and missing authority class, do not open another
micro-basis in that same lane. Park the lane, open an explicit authority
registry basis, or keep stopped with a reentry contract.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    source-selfhost-local-candidate-selection-policy-v0.json

tool:
  tools/rust_lifecycle/
    source_selfhost_local_candidate_selection_policy.py

guard:
  tools/checks/
    rust_lifecycle_source_selfhost_local_candidate_selection_policy_guard.sh
```

## Non-Claims

```text
semantic_lane_selected = 0
projection_policy_selected = 0
source_selfhost_claim = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
manual_lane_selection = 0
row_count_as_proof = 0
cluster_size_as_proof = 0
owner_name_as_proof = 0
historical_preference_as_proof = 0
external_consultation_for_counting = 0
```
