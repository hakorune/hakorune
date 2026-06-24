---
Status: Landed
Date: 2026-05-29
Scope: freeze the selected-method typed-field direct-op guard surface before implementation.
Blocker: MIR-TYPED-FIELD-DIRECT-OP-GUARD-SURFACE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-218-MIR-TYPED-FIELD-DIRECT-OP-NET-INVENTORY.md
  - docs/development/current/main/phases/phase-296x/296x-217-POST-EXACT-SLOT-DIRECT-HELPER-OWNER-REFRESH.md
---

# 296x-219 MIR Typed-Field Direct-Op Guard Surface

## Purpose

Freeze the exact selected-method surface before implementing typed-field
direct-op lowering.

This row is still inventory-only. It records which selected method, helper
symbols, fields, and semantic guards the implementation row must preserve.

## Evidence

```text
output_contract=mir-typed-field-direct-op-guard-surface-v0
input_contract=mir-typed-field-direct-op-net-inventory-v0
workload_id=representative-object-lifecycle-small-block-v0
selected_method=HakoAllocPageModel.acquire_usize/1
candidate_field_get_count=13
candidate_field_set_count=8
candidate_total=21
projected_net_helper_call_delta=21
candidate_i64_count=2
candidate_u64_count=0
candidate_usize_count=17
candidate_handle_count=2
unsigned_set_nonnegative_guard_count=8
unsigned_set_const_nonnegative_count=0
unsigned_set_const_negative_reject_count=0
set_status_trap_count=8
helper_free_direct_op_required=1
slot_constant_required=1
typed_object_plan_required=1
weak_field_rejected=1
unsupported_storage_rejected=1
fallback_silent_success=0
residence_transform_open=0
direct_op_transform_open=0
implementation_open=0
projected_symbol_0=nyash.object.exact_slot_get_handle_hii
projected_symbol_0_count=2
projected_symbol_1=nyash.object.exact_slot_get_i64_hii
projected_symbol_1_count=2
projected_symbol_2=nyash.object.exact_slot_get_u64_hii
projected_symbol_2_count=9
projected_symbol_3=nyash.object.exact_slot_set_u64_hiu
projected_symbol_3_count=8
candidate_field_0=HakoAllocPageModel.reject_count.usize
candidate_field_0_count=6
candidate_field_1=HakoAllocPageModel.free_top.usize
candidate_field_1_count=2
candidate_field_2=HakoAllocPageModel.used.usize
candidate_field_2_count=2
selected_next=mir_typed_field_direct_op_selected_method_keeper
by_name_special_case=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Implementation Boundary

The next row may implement a selected-method direct-op keeper for
`HakoAllocPageModel.acquire_usize/1` only.

Required constraints:

```text
target_method=HakoAllocPageModel.acquire_usize/1
helper_free_direct_op_required=1
slot_constant_required=1
typed_object_plan_required=1
unsigned_set_nonnegative_guard_count=8
set_status_trap_count=8
fallback_silent_success=0
by_name_special_case=0
```

Do not reopen broad typed-field residence in the implementation row. The
previous residence attempts remain rejected until a separate positive
load/writeback net delta is proven.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_mir_typed_field_direct_op_guard_surface_guard.sh
```
