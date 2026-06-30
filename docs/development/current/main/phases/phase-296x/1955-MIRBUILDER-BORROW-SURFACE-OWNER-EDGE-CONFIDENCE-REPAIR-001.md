# 1955 - MIRBUILDER-BORROW-SURFACE-OWNER-EDGE-CONFIDENCE-REPAIR-001

## Token

```text
MIRBUILDER-BORROW-SURFACE-OWNER-EDGE-CONFIDENCE-REPAIR-001
```

## Purpose

Repair owner-edge confidence for borrow-policy candidates before selecting any
borrow replacement policy.

The repair is file/module scoped. It maps each borrow cluster to a deterministic
`mirbuilder::borrow_surface::<source_module>` owner edge using source-path
evidence. It does not infer borrow semantics, select `MutLease`, or emit Hako.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-borrow-surface-owner-edge-confidence-repair-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_borrow_surface_owner_edge_confidence_repair.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_borrow_surface_owner_edge_confidence_repair_guard.sh
```

## Acceptance

```text
input_borrow_surface_candidate_count = 112
repaired_candidate_count = 112
repaired_cluster_count = 61
file_scoped_owner_edge_count = 46
selection_eligible_for_borrow_policy_count = 61
old_owner_edge_confidence = None
repaired_owner_edge_confidence = FileScoped
decision = SelectBorrowSurfacePolicyClusterRerun
selected_next_card = MIRBUILDER-BORROW-SURFACE-POLICY-CLUSTER-RERUN-001
manual_owner_edge_selection = 0
borrow_policy_selected = 0
mut_lease_selected = 0
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
  SelectBorrowSurfacePolicyClusterRerun

reason_token:
  BorrowSurfaceOwnerEdgeConfidenceRepaired

selected_next_card:
  MIRBUILDER-BORROW-SURFACE-POLICY-CLUSTER-RERUN-001
```

## Non-Claims

```text
no borrow policy selected
no MutLease selected
no Hako generation
no native source seed
no HakoAdopted decision
no Source Selfhost claim
```
