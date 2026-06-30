# 1956 - MIRBUILDER-BORROW-SURFACE-POLICY-CLUSTER-RERUN-001

## Token

```text
MIRBUILDER-BORROW-SURFACE-POLICY-CLUSTER-RERUN-001
```

## Purpose

Rerun borrow-surface policy cluster selection after owner-edge confidence was
repaired.

This card selects the next borrow policy cluster by evidence quality. It does
not select the replacement policy itself.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-borrow-surface-policy-cluster-rerun-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_borrow_surface_policy_cluster_rerun.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_borrow_surface_policy_cluster_rerun_guard.sh
```

## Acceptance

```text
selection_eligible_cluster_count = 61
returned_mutable_borrow_cluster_count = 3
returned_read_borrow_cluster_count = 58
selected_cluster.borrow_kind = ReturnedMutableBorrow
selected_cluster.return_shape = mutable_ref
selected_cluster.receiver_axis = mutable_receiver
decision = SelectBorrowProjectionPolicyCluster
selected_next_card =
  MIRBUILDER-BORROW-SURFACE-RETURNED-MUTABLE-BORROW-POLICY-001
borrow_policy_cluster_selected = 1
borrow_policy_selected = 0
mut_lease_selected = 0
owned_read_snapshot_selected_for_new_surface = 0
explicit_mutation_api_selected_for_new_surface = 0
manual_borrow_policy_selection = 0
cluster_size_as_proof = 0
strict_rules_changed = 0
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

## Result

```text
decision:
  SelectBorrowProjectionPolicyCluster

reason_token:
  HighestRiskBorrowPolicyClusterSelected

selected_next_card:
  MIRBUILDER-BORROW-SURFACE-RETURNED-MUTABLE-BORROW-POLICY-001
```

The selected cluster is the highest-risk returned mutable borrow shape. The
next card must choose a policy for that cluster or defer it with a stable
reason.

## Non-Claims

```text
no borrow policy selected
no MutLease selected
no Hako generation
no native source seed
no HakoAdopted decision
no Source Selfhost claim
```
