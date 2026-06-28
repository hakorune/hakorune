---
Status: Active
Decision: accepted
Date: 2026-06-29
Scope: Re-run the VariableContext route matrix after the owned read snapshot
  projection repair.
Related:
  - docs/development/current/main/phases/phase-296x/1786-MIRBUILDER-VARIABLE-CONTEXT-OWNED-READ-SNAPSHOT-PROJECTION-001.md
  - docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-route-matrix-rerun-v0.json
  - tools/checks/rust_lifecycle_variable_context_route_matrix_rerun_guard.sh
---

# MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-RERUN-001

## Goal

Consume the `OwnedReadSnapshotProjection` evidence and re-classify the
VariableContext route matrix without manual family selection. The previous
blocked row was:

```text
VariableContext_immutable_borrow_only
  state = Denied
  reason = ReturnedReadBorrow
  replacement_policy = OwnedReadSnapshotProjection
```

After 1786, the replacement is executable as a native owned snapshot, so the
matrix can derive a new candidate surface.

```text
docs_only_closeout = forbidden
code_or_guard_delta_required = 1
```

## Rerun Result

```text
input_state:
  candidate_pool_state = Blocked
  repaired_row = VariableContext_immutable_borrow_only
  repaired_reason = ReturnedReadBorrow

repair_evidence:
  owned_read_snapshot_projection = green
  raw_variable_map_alias_emitted = 0
  variable_map_mut_selected = 0

rerun_state:
  candidate_pool_state = CandidateEligible
  selected_surface = VariableContextNativeSurfaceOwnedReadSnapshotV1
```

## Selected Surface

```text
included:
  VariableContext_simple_map_only
  VariableContext_snapshot_restore_only
  VariableContext_carrier_snapshot_only
  VariableContext_explicit_carrier_snapshot_only
  VariableContext_immutable_borrow_repaired_as_owned_snapshot

excluded:
  VariableContext_mutable_returned_borrow
```

## Acceptance

```text
input_projection = MIRBUILDER-VARIABLE-CONTEXT-OWNED-READ-SNAPSHOT-PROJECTION-001
projection_result = OwnedReadSnapshotProjection
route_matrix_rerun = 1
candidate_pool_state = CandidateEligible
selected_surface_id = VariableContextNativeSurfaceOwnedReadSnapshotV1
full_variable_context_claim = 0
raw_variable_map_alias_selected = 0
variable_map_mut_selected = 0
manual_family_selection = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
source_selfhost_claim = 0
next_action = VARIABLE-CONTEXT-OWNED-READ-SNAPSHOT-HAKO-ADOPTION-DECISION-001
```

## Non-Claims

```text
full VariableContext HakoAdopted = 0
returned mutable borrow repair = 0
BorrowView implementation = 0
Rust deletion = 0
Source Selfhost = 0
runtime fallback = 0
new backend route = 0
new ABI = 0
```

## Closeout

```text
output_contract=rust-lifecycle-variable-context-route-matrix-rerun-v0
candidate_pool_state=CandidateEligible
selected_surface_id=VariableContextNativeSurfaceOwnedReadSnapshotV1
owned_read_snapshot_projection=green
raw_variable_map_alias_selected=0
variable_map_mut_selected=0
full_variable_context_claim=0
manual_family_selection=0
runtime_fallback=0
new_backend_route=0
new_abi=0
new_python_semantic_projector=0
source_selfhost_claim=0
next_action=VARIABLE-CONTEXT-OWNED-READ-SNAPSHOT-HAKO-ADOPTION-DECISION-001
summary=ok
```
