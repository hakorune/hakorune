---
Status: Current
Date: 2026-05-28
Scope: count net helper-call erasure before another typed-field residence implementation.
Blocker: MIR-TYPED-FIELD-RESIDENCE-ERASURE-FEASIBILITY-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-196-MIR-TYPED-FIELD-RESIDENCE-SELECTED-METHOD-KEEPER.md
  - tools/allocator/mir_typed_field_residence_erasure_feasibility.py
---

# 296x-197 MIR Typed Field Residence Erasure Feasibility

## Purpose

Stop the selected-method residence lane from guessing after row196 showed that
block-local residence only moved `field_set` helpers to writeback sites. This
row measures net helper-call erasure before another implementation attempt.

## Evidence

```text
output_contract=mir-typed-field-residence-erasure-feasibility-v0
input_contract=mir-typed-field-residence-selected-method-plan-v0
selected_method=HakoAllocPageModel.acquire_usize/1
block_count=12
scalar_field_get_count=11
scalar_field_set_count=8
set_replacement_count=8
writeback_required_count=8
duplicate_get_erasure_count=0
coalesced_set_erasure_count=0
net_helper_call_delta=0
block_local_residence_feasible=0
rejected_handle_field_count=2
barrier_policy=block_local_only
implementation_recommendation=do_not_implement_block_local_residence
next_diagnostic=cfg_residence_or_runtime_owner_selection
transform_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Interpretation

```text
Row195's would-erase estimate was too optimistic for block-local residence.
For acquire_usize/1, every scalar field set still requires one writeback and
there are no same-block duplicate scalar gets or coalescable repeated sets.
Therefore a second block-local keeper attempt is rejected.
```

## Next

```text
row198:
  cfg_residence_or_runtime_owner_selection

Goal:
  choose between a real CFG-aware residence design with PHI/writeback ownership
  and a different large owner. Do not edit compiler/runtime code until that
  owner is selected.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_mir_typed_field_residence_erasure_feasibility_guard.sh
```
