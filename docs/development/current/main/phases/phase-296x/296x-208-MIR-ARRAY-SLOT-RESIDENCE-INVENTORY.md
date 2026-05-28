---
Status: Current
Date: 2026-05-28
Scope: inventory MIR ArraySlotResidence helper-call erasure before any transform.
Blocker: MIR-ARRAY-SLOT-RESIDENCE-INVENTORY-296X-001
Related:
  - docs/development/current/main/design/mir-array-slot-residence-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-207-MIR-ARRAY-SLOT-RESIDENCE-SSOT.md
---

# 296x-208 MIR Array Slot Residence Inventory

## Purpose

Inventory ArrayBox get/set helper erasure for the selected hot method before
any MIR transform. The selected method is fixed by dynamic object-lifecycle
context, not by the largest static candidate, because release fallback methods
can look larger while remaining inactive in the proof workload.

## Inventory Report

```text
output_contract=mir-array-slot-residence-inventory-v0
input_kind=mir_json
workload_id=representative-object-lifecycle-small-block-v0
candidate_function_count=9
selected_method=HakoAllocPageModel.acquire_usize/1
selected_reason=explicit_hot_context
eligible_array_get_count=1
eligible_array_set_count=1
erased_get_set_helper_calls=2
added_guard_helper_calls=1
added_writeback_helper_calls=0
net_helper_call_delta=1
barrier_unknown_call_count=1
barrier_escape_count=0
barrier_phi_count=1
barrier_storage_kind_count=1
transform_open=0
array_helper_abi_fallback=1
positive_net_helper_call_delta_required=1
positive_net_helper_call_delta=1
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
mir_array_slot_residence_inventory=accepted
selected_method=HakoAllocPageModel.acquire_usize/1
selected_method_reason=dynamic_hot_context_object_lifecycle_small_alloc
positive_net_helper_call_delta=1
transform_open=0
array_helper_abi_fallback=1
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Next

```text
row209:
  mir_array_slot_residence_selected_method_keeper

Goal:
  transform only HakoAllocPageModel.acquire_usize/1 if the implementation can
  preserve the explicit barriers and keep the helper ABI as fallback.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_mir_array_slot_residence_inventory_guard.sh
```
