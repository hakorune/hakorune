---
Status: Current
Date: 2026-05-28
Scope: fix the selected-method ArraySlotResidence guard surface before implementation.
Blocker: MIR-ARRAY-SLOT-RESIDENCE-SELECTED-METHOD-GUARD-SURFACE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-208-MIR-ARRAY-SLOT-RESIDENCE-INVENTORY.md
---

# 296x-209 MIR Array Slot Residence Selected Method Guard Surface

## Purpose

Freeze the exact selected-method pattern before implementing ArraySlotResidence.
The goal is to avoid a broad generic residence transform and keep row210
implementation focused on one block-local ArrayBox get/set pair.

## Guard Surface

```text
output_contract=mir-array-slot-residence-selected-method-guard-surface-v0
input_contract=mir-array-slot-residence-inventory-v0
selected_method=HakoAllocPageModel.acquire_usize/1
selected_reason=explicit_hot_context
selected_block=45
array_get_call_count=1
array_set_call_count=1
same_block_get_set_pair=1
set_uses_get_result=1
planned_transform_kind=selected_method_same_block_array_get_set_direct_slot_op
planned_erased_get_set_helper_calls=2
planned_added_guard_helper_calls=1
planned_added_writeback_helper_calls=0
planned_net_helper_call_delta=1
implementation_surface_supported=1
generic_array_residence_open=0
by_name_hako_alloc_special_case=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
mir_array_slot_residence_selected_method_guard_surface=accepted
selected_method=HakoAllocPageModel.acquire_usize/1
implementation_surface_supported=1
implementation_owner=selected_method_same_block_array_get_set_direct_slot_op
generic_array_residence_open=0
by_name_hako_alloc_special_case=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Next

```text
row210:
  selected_method_array_slot_direct_op_owner_selection

Goal:
  choose the implementation owner. The guard surface shows a supported pattern,
  but a one-pass LLVM boxcall replacement cannot erase both helper calls by
  itself because the ArrayBox.get result flows through copy carriers into the
  later ArrayBox.set.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_mir_array_slot_residence_selected_method_guard_surface_guard.sh
```
