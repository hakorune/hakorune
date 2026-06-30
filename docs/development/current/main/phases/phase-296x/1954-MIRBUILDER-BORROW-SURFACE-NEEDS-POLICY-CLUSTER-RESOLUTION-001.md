# 1954 - MIRBUILDER-BORROW-SURFACE-NEEDS-POLICY-CLUSTER-RESOLUTION-001

## Token

```text
MIRBUILDER-BORROW-SURFACE-NEEDS-POLICY-CLUSTER-RESOLUTION-001
```

## Purpose

Cluster the remaining `BorrowSurfaceNeedsPolicy` rows before selecting any
borrow replacement policy.

This card classifies the 112 unknown borrow surfaces by borrow kind, return
shape, receiver, source module, and owner-edge confidence. It does not select
`MutLease`, `OwnedReadSnapshotProjection`, or `ExplicitMutationApiOnly` for new
surfaces.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-borrow-surface-needs-policy-cluster-resolution-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_borrow_surface_needs_policy_cluster_resolution.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_borrow_surface_needs_policy_cluster_resolution_guard.sh
```

## Acceptance

```text
borrow_surface_needs_policy_count = 112
returned_read_borrow_count = 109
returned_mutable_borrow_count = 3
owner_edge_confidence_none_count = 112
selection_eligible_cluster_count = 0
decision = SelectBorrowSurfaceOwnerEdgeConfidenceRepair
reason_token = BorrowSurfaceOwnerEdgeConfidenceMissingForAllCandidates
selected_next_card =
  MIRBUILDER-BORROW-SURFACE-OWNER-EDGE-CONFIDENCE-REPAIR-001
borrow_policy_selected = 0
mut_lease_selected = 0
owned_read_snapshot_selected_for_new_surface = 0
explicit_mutation_api_selected_for_new_surface = 0
manual_borrow_policy_selection = 0
manual_owner_edge_selection = 0
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
  SelectBorrowSurfaceOwnerEdgeConfidenceRepair

reason_token:
  BorrowSurfaceOwnerEdgeConfidenceMissingForAllCandidates

selected_next_card:
  MIRBUILDER-BORROW-SURFACE-OWNER-EDGE-CONFIDENCE-REPAIR-001
```

All current borrow-policy candidates lack owner-edge confidence, so selecting a
borrow replacement policy would be manual. Repair the owner-edge mapping first.

## Non-Claims

```text
no borrow policy selected
no MutLease selected
no Hako generation
no native source seed
no HakoAdopted decision
no Source Selfhost claim
```
