# 1921 - MIRBUILDER-PROJECTION-POLICY-CLUSTER-ID-AXIS-STABILITY-REPAIR-001

## Token

```text
MIRBUILDER-PROJECTION-POLICY-CLUSTER-ID-AXIS-STABILITY-REPAIR-001
```

## Purpose

Repair projection-policy cluster identity before opening the next
`RecipeTreeMatcherCluster` projection policy card.

The crate-wide missing-projection cluster resolver previously emitted the same
`cluster_id` for sibling axis variants of one source cluster. That made a
landed narrow policy or decomposition capable of hiding blocked sibling
clusters that still need type-transport or borrow repair.

This card makes `cluster_id` axis-qualified and unique while preserving
`legacy_cluster_id` for historical decomposition filters.

## Output

```text
updated tools:
  tools/rust_lifecycle/
    mirbuilder_crate_wide_missing_projection_policy_cluster_resolution.py
    mirbuilder_projection_policy_cluster_priority_resolution.py

updated fixtures:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-crate-wide-missing-projection-policy-cluster-resolution-v0.json
    mirbuilder-projection-policy-cluster-priority-resolution-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_projection_policy_cluster_id_axis_stability_repair_guard.sh
```

## Contract

```text
cluster_id:
  includes stable deny reason, shape, owner confidence, source cluster,
  borrow axis, control-flow axis, type axis, call axis, and verifier axis

legacy_cluster_id:
  preserves the pre-repair source-cluster id for historical decomposition
  filters only
```

## Decision

```text
kind = RepairProjectionPolicyClusterIdentity

next_card =
  MIRBUILDER-RECIPE-TREE-MATCHER-PROJECTION-POLICY-001
```

## Acceptance

```text
duplicate_cluster_id_count = 0
legacy_cluster_id_preserved = 1
priority_selected_cluster_id_is_axis_qualified = 1
manual_family_selection = 0
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

## Non-Claims

```text
no projection policy selected by this repair
no Hako emitted
no HakoAdopted decision
no native source seed materialization
no Source Selfhost claim
```
