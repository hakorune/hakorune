---
Status: Landed
Date: 2026-05-28
Scope: build a selected-method field residence plan for HakoAllocPageModel.acquire_usize/1.
Blocker: MIR-TYPED-FIELD-RESIDENCE-SELECTED-METHOD-PLAN-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-194-MIR-TYPED-FIELD-RESIDENCE-INVENTORY.md
---

# 296x-195 MIR Typed Field Residence Selected Method Plan

## Purpose

Convert row194's inventory result into a selected-method residence plan for
`HakoAllocPageModel.acquire_usize/1`. This row remains observation/planning
only: it does not rewrite MIR and does not change runtime behavior.

## Selected Method

```text
selected_method=HakoAllocPageModel.acquire_usize/1
selection_reason=largest_dynamic_scalar_field_residence_surface
dynamic_eligible_estimate=9961472
```

## Plan Contract

```text
output_contract=mir-typed-field-residence-selected-method-plan-v0
input_contract=mir-typed-field-residence-inventory-v0
selected_method=HakoAllocPageModel.acquire_usize/1
scalar_field_count=...
readonly_field_count=...
writeback_field_count=...
rejected_handle_field_count=...
barrier_unknown_call_count=...
barrier_phi_count=...
barrier_return_count=...
helper_load_on_first_use_count=...
writeback_on_return_count=...
transform_open=0
by_name_special_case=0
summary=ok
```

## Acceptance

```text
selected_method_plan=accepted
selected_method=HakoAllocPageModel.acquire_usize/1
transform_open=0
writeback_field_count_positive=1
helper_load_on_first_use_count_positive=1
by_name_special_case=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```
