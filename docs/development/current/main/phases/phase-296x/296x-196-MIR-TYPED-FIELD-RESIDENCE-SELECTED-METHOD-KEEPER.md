---
Status: Current
Date: 2026-05-28
Scope: implement the first selected-method MIR typed-field residence keeper.
Blocker: MIR-TYPED-FIELD-RESIDENCE-SELECTED-METHOD-KEEPER-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-195-MIR-TYPED-FIELD-RESIDENCE-SELECTED-METHOD-PLAN.md
---

# 296x-196 MIR Typed Field Residence Selected Method Keeper

## Purpose

Implement the first narrow MIR typed-field residence keeper for
`HakoAllocPageModel.acquire_usize/1`, using row195's plan as the only input.
This row must stay selected-method only.

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
