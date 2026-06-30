# 1880 - MIRBUILDER-PROJECTION-POLICY-EXISTING-DECISION-FILTER-001

## Token

```text
MIRBUILDER-PROJECTION-POLICY-EXISTING-DECISION-FILTER-001
```

## Purpose

Prevent the projection-policy priority resolver from selecting a policy card
that already landed.

The previous priority resolution selected
`MIRBUILDER-LOOP-COND-BC-CARRIER-SYNC-PROJECTION-POLICY-001`, but that card
already exists and decides `KeepParentOwner`. This card teaches the resolver to
read the source-selfhost family guard manifest and exclude eligible clusters
whose proposed next card is already present.

## Result

```text
eligible_cluster_count = 42
excluded_existing_decision_cluster_count = 5
selectable_cluster_count = 37

selected_next_card =
  MIRBUILDER-CARRIER-FEATURE-PROJECTION-POLICY-001
```

## Acceptance

```text
existing_decision_filter_enabled = 1
excluded_existing_decision_cluster_count = 5
selectable_cluster_count = 37
cluster_size_as_proof = 0
manual_family_selection = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
family_name_based_policy = 0
```

## Non-Claims

```text
no projection policy definition
no Hako generation
no HakoAdopted decision
no native source seed materialization
no Source Selfhost claim
```
