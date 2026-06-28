---
Status: Active
Decision: accepted
Date: 2026-06-29
Scope: Decide HakoAdopted state for the bounded VariableContext surface that
  includes the owned read snapshot replacement.
Related:
  - docs/development/current/main/phases/phase-296x/1787-MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-RERUN-001.md
  - docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-owned-read-snapshot-hako-adoption-decision-v0.json
  - tools/checks/rust_lifecycle_variable_context_owned_read_snapshot_hako_adoption_decision_guard.sh
---

# VARIABLE-CONTEXT-OWNED-READ-SNAPSHOT-HAKO-ADOPTION-DECISION-001

## Goal

Adopt the machine-derived VariableContext native surface that includes
`OwnedReadSnapshotProjection` for the formerly returned read borrow route.

This is a family-surface adoption decision, not a full VariableContext claim.
The mutable returned borrow remains denied.

```text
docs_only_closeout = forbidden
code_or_guard_delta_required = 1
```

## Decision

```text
target_family:
  hakorune_mir_builder::variable_context

target_surface:
  VariableContextNativeSurfaceOwnedReadSnapshotV1

decision:
  Adopt

reason:
  NativeSurfaceOwnerPresentAndOwnedReadSnapshotGreen
```

## Included Surface

```text
VariableContext_simple_map_only
VariableContext_snapshot_restore_only
VariableContext_carrier_snapshot_only
VariableContext_explicit_carrier_snapshot_only
VariableContext_immutable_borrow_repaired_as_owned_snapshot
```

## Excluded Surface

```text
VariableContext_mutable_returned_borrow:
  ReturnedMutableBorrow
```

## Acceptance

```text
route_matrix_rerun = MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-RERUN-001
selected_surface_id = VariableContextNativeSurfaceOwnedReadSnapshotV1
decision = Adopt
native_hako_source_owner_present = 1
native_behavior_guard_green = 1
owned_read_snapshot_projection_green = 1
generator_overwrite_guard = 1
rust_bootstrap_retained = 1
rust_oracle_retained = 1
full_variable_context_claim = 0
returned_mutable_borrow_selected = 0
raw_variable_map_alias_selected = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
```

## Non-Claims

```text
full VariableContext HakoAdopted = 0
BorrowView implementation = 0
returned mutable borrow repair = 0
Rust deletion = 0
Source Selfhost = 0
runtime fallback = 0
new backend route = 0
new ABI = 0
```

## Closeout

```text
output_contract=rust-lifecycle-variable-context-owned-read-snapshot-hako-adoption-decision-v0
surface_id=VariableContextNativeSurfaceOwnedReadSnapshotV1
decision=Adopt
included_scope_count=5
native_hako_source_owner_present=1
native_behavior_guard_green=1
owned_read_snapshot_projection_green=1
generator_overwrite_guard=1
rust_bootstrap_retained=1
rust_oracle_retained=1
full_variable_context_claim=0
returned_mutable_borrow_selected=0
raw_variable_map_alias_selected=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
```
