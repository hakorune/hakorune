---
Status: Landed
Date: 2026-05-28
Scope: implement the first selected-method MIR typed-field residence keeper.
Blocker: MIR-TYPED-FIELD-RESIDENCE-SELECTED-METHOD-KEEPER-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-195-MIR-TYPED-FIELD-RESIDENCE-SELECTED-METHOD-PLAN.md
---

# 296x-196 MIR Typed Field Residence Selected Method Keeper

## Purpose

Attempt the first narrow MIR typed-field residence keeper for
`HakoAllocPageModel.acquire_usize/1`, using row195's plan as the only input.
This row stayed selected-method only and rejected the block-local
implementation shape before landing code.

## Implementation Boundary

```text
selected_method=HakoAllocPageModel.acquire_usize/1
residence_kind=method_receiver_cache_writeback
init_policy=helper_load_on_first_use
writeback_policy=writeback_on_return
readonly_fields=block_size,decommitted,retired
writeback_fields=alloc_count,free_top,peak_used,reject_count,requested_bytes,used
rejected_handle_fields=block_used,free
```

## Required Guardrails

```text
- Do not specialize by hako_alloc source names outside the selected method gate.
- Do not transform handle fields.
- Do not cross unknown-call, PHI, or return barriers without explicit load /
  writeback accounting.
- Keep typed-object helper ABI fallback.
- Keep provider activation, allocator replacement, hooks, globals, and winner
  claims closed.
```

## Acceptance Draft

```text
output_contract=mir-typed-field-residence-selected-method-keeper-v0
selected_method=HakoAllocPageModel.acquire_usize/1
erased_field_get_count=...
erased_field_set_count=...
helper_load_count=...
writeback_count=...
rejected_handle_field_count=2
semantic_summary=ok
body_measurement_required=1
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Outcome

```text
output_contract=mir-typed-field-residence-selected-method-attempt-v0
selected_method=HakoAllocPageModel.acquire_usize/1
attempted_shape=block_local_residence_with_block_end_writeback
semantic_summary=ok
ir_observation=field_set_helpers_moved_to_resident_field_set_writebacks
erased_helper_call_count=0
body_measurement_sample_ns=245000000
keeper_effect=no_effect
implementation_landed=0
rollback_required=0
next_diagnostic=mir_typed_field_residence_erasure_feasibility
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Interpretation:

```text
The selected-method block-local residence shape did not erase helper calls. It
only delayed scalar field_set helpers into block-end resident writebacks. Since
`acquire_usize/1` has no duplicate scalar field get or repeated scalar field set
inside a block, this is not a keeper. Row197 must count net helper-call erasure
before any second residence implementation attempt.
```
