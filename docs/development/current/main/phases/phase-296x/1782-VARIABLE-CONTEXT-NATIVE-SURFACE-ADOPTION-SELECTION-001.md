---
Status: Active
Decision: accepted
Date: 2026-06-28
Scope: Select a native Hako adoption surface for VariableContext without
  claiming full VariableContext or reopening returned borrow routes.
Related:
  - docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md
  - docs/development/current/main/phases/phase-296x/1781-SOURCE-SELFHOST-BLOCKED-RECOVERY-DIAGNOSTIC-001.md
  - docs/development/current/main/phases/phase-296x/1774-MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-CLOSEOUT-001.md
  - docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-native-surface-adoption-selection-v0.json
  - tools/checks/rust_lifecycle_variable_context_native_surface_adoption_selection_guard.sh
---

# VARIABLE-CONTEXT-NATIVE-SURFACE-ADOPTION-SELECTION-001

## Goal

Turn the blocked source-selfhost recovery line into a bounded next owner. The
full `VariableContext` family remains parked by `ReturnedReadBorrow`, but the
route matrix already has several selected `DerivedMainline` scopes that do not
require a returned borrow. This row selects that smaller native surface as the
next machine-derived adoption candidate.

```text
docs_only_closeout = forbidden
code_or_guard_delta_required = 1
```

## Selected Surface

```text
surface_id:
  VariableContextNativeSurfaceNoReturnedBorrowV1

included_scopes:
  VariableContext_simple_map_only
  VariableContext_snapshot_restore_only
  VariableContext_carrier_snapshot_only
  VariableContext_explicit_carrier_snapshot_only

excluded_scopes:
  VariableContext_immutable_borrow_only
  VariableContext_mutable_returned_borrow
```

## Design Decision

```text
full_variable_context_claim = 0
returned_borrow_selected = 0
native_surface_candidate_state = CandidateEligible
next_action =
  VARIABLE-CONTEXT-NATIVE-SURFACE-HAKO-ADOPTION-DECISION-001
```

The design intentionally does not solve Rust returned references first. It
keeps `variable_map()` and `variable_map_mut()` out of the selected native
surface, while allowing already-selected owned and snapshot routes to move
toward native Hako authority.

## Authority

```text
lang/generated/rust_derived/hakorune_mir_builder/family_routes.json
docs/development/current/main/design/fixtures/rust-lifecycle/
  variable-context-route-matrix-closeout-v0.json
docs/development/current/main/design/fixtures/rust-lifecycle/
  source-selfhost-blocked-recovery-diagnostic-v0.json
tools/checks/rust_lifecycle_variable_context_native_surface_adoption_selection_guard.sh
```

## Acceptance

```text
source_selfhost_candidate_pool_state = Blocked
full_variable_context_family_state = Parked
full_variable_context_claim = 0
native_surface_candidate_state = CandidateEligible
included_scopes are all DerivedMainline
included_scopes all selected_on_mainline = true
denied returned-borrow route remains denied
replacement_policy = OwnedReadSnapshotProjection
manual_family_selection = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
source_selfhost_claim = 0
```

## Non-Claims

```text
full VariableContext HakoAdopted = 0
returned read borrow repair = 0
returned mutable borrow repair = 0
BorrowView implementation = 0
Rust reference parity = 0
Source Selfhost = 0
Rust deletion = 0
runtime fallback = 0
new backend route = 0
new ABI = 0
```

## Closeout

```text
output_contract=rust-lifecycle-variable-context-native-surface-adoption-selection-v0
source_selfhost_candidate_pool_state=Blocked
full_variable_context_family_state=Parked
native_surface_candidate_state=CandidateEligible
included_scope_count=4
excluded_returned_borrow_count=2
next_action=VARIABLE-CONTEXT-NATIVE-SURFACE-HAKO-ADOPTION-DECISION-001
manual_family_selection=0
runtime_fallback=0
new_backend_route=0
new_abi=0
new_python_semantic_projector=0
source_selfhost_claim=0
summary=ok
```
