---
Status: Active
Decision: accepted
Date: 2026-06-29
Scope: Re-run the VariableContext route matrix after the explicit mutation API
  projection repair.
Related:
  - docs/development/current/main/phases/phase-296x/1791-MIRBUILDER-VARIABLE-CONTEXT-EXPLICIT-MUTATION-API-PROJECTION-001.md
  - docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-route-matrix-rerun-002-v0.json
  - tools/checks/rust_lifecycle_variable_context_route_matrix_rerun_002_guard.sh
---

# MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-RERUN-002

## Goal

Consume the `ExplicitMutationApiOnly` evidence and re-classify the
VariableContext route matrix without manual family selection. The previous
blocked row was:

```text
VariableContext_mutable_returned_borrow
  state = Blocked
  reason = ReturnedMutableBorrow
  replacement_policy = ExplicitMutationApiOnly
```

After 1791, the explicit mutation surface is executable as a bounded native
surface, so the matrix can derive a new candidate surface.

```text
docs_only_closeout = forbidden
code_or_guard_delta_required = 1
```

## Rerun Result

```text
input_state:
  candidate_pool_state = Blocked
  repaired_row = VariableContext_mutable_returned_borrow
  repaired_reason = ReturnedMutableBorrow

repair_evidence:
  explicit_mutation_projection = green
  replace_owned_map_native_api = 1
  raw_variable_map_mut_alias_emitted = 0
  variable_map_mut_selected = 0

rerun_state:
  candidate_pool_state = CandidateEligible
  selected_surface = VariableContextNativeSurfaceExplicitMutationApiOnlyV1
```

## Selected Surface

```text
included:
  VariableContext_simple_map_only
  VariableContext_snapshot_restore_only
  VariableContext_carrier_snapshot_only
  VariableContext_explicit_carrier_snapshot_only
  VariableContext_immutable_borrow_repaired_as_owned_snapshot
  VariableContext_mutable_returned_borrow_repaired_as_explicit_mutation

excluded:
  VariableContext_mutable_returned_borrow
```

## Acceptance

```text
input_projection = MIRBUILDER-VARIABLE-CONTEXT-EXPLICIT-MUTATION-API-PROJECTION-001
projection_result = ExplicitMutationApiOnly
route_matrix_rerun = 1
candidate_pool_state = CandidateEligible
selected_surface_id = VariableContextNativeSurfaceExplicitMutationApiOnlyV1
full_variable_context_claim = 0
raw_variable_map_alias_selected = 0
variable_map_mut_selected = 0
manual_family_selection = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
source_selfhost_claim = 0
next_action = VARIABLE-CONTEXT-EXPLICIT-MUTATION-API-HAKO-ADOPTION-DECISION-001
```

## Non-Claims

```text
full VariableContext HakoAdopted = 0
returned mutable borrow repair = 0
MutLease implementation = 0
Source Selfhost = 0
runtime fallback = 0
new backend route = 0
new ABI = 0
```

## Closeout

```text
output_contract=rust-lifecycle-variable-context-route-matrix-rerun-v1
candidate_pool_state=CandidateEligible
selected_surface_id=VariableContextNativeSurfaceExplicitMutationApiOnlyV1
explicit_mutation_projection=green
raw_variable_map_alias_selected=0
variable_map_mut_selected=0
full_variable_context_claim=0
manual_family_selection=0
runtime_fallback=0
new_backend_route=0
new_abi=0
new_python_semantic_projector=0
source_selfhost_claim=0
next_action=VARIABLE-CONTEXT-EXPLICIT-MUTATION-API-HAKO-ADOPTION-DECISION-001
summary=ok
```
