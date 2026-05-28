---
Status: Current
Date: 2026-05-28
Scope: choose the implementation owner for the selected-method ArraySlot direct op keeper.
Blocker: SELECTED-METHOD-ARRAY-SLOT-DIRECT-OP-OWNER-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-209-MIR-ARRAY-SLOT-RESIDENCE-SELECTED-METHOD-GUARD-SURFACE.md
---

# 296x-210 Selected Method Array Slot Direct Op Owner Selection

## Purpose

Choose the narrow implementation owner for erasing the selected
`ArrayBox.get` / `ArrayBox.set` helper pair in
`HakoAllocPageModel.acquire_usize/1`.

The row209 guard surface proved that the get result flows through copy carriers
into the later set call. Therefore the owner must see the block instruction
sequence. Individual `ArrayBox.get` / `ArrayBox.set` lowering cannot safely erase
both helper calls by itself.

## Decision

```text
Decision: accepted

output_contract=selected-method-array-slot-direct-op-owner-selection-v0
input_contract=mir-array-slot-residence-selected-method-guard-surface-v0
selected_method=HakoAllocPageModel.acquire_usize/1
selected_block=45
selected_owner=llvm_block_lower_array_slot_direct_op_fusion
implementation_owner_file=src/llvm_py/builders/block_lower.py
helper_owner_file=src/llvm_py/instructions/array_slot_direct_op.py
runtime_export_owner=crates/nyash_kernel/src/plugin/array_direct_slot_op.rs
runtime_mod_owner=crates/nyash_kernel/src/plugin/mod.rs
selected_reason=same_block_get_copy_set_pattern_requires_block_sequence_owner
planned_fused_runtime_symbol=nyash.array.slot_load_store_i64_hihi
planned_erased_get_set_helper_calls=2
planned_added_fused_helper_calls=1
planned_net_helper_call_delta=1
generic_array_residence_open=0
by_name_hako_alloc_special_case=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Rejected Owners

```text
boxcall_runtime_data_individual_get_set_lowering:
  rejected because it cannot erase a get call that was already lowered when
  the later set call is seen.

generic_mir_array_residence_transform:
  rejected because it is too broad before the selected-method keeper proves the
  direct-op seam.

hako_alloc_by_name_source_rewrite:
  rejected because it would push a lowering workaround into source.
```

## Next

```text
row211:
  selected_method_array_slot_direct_op_keeper

Goal:
  add one fused runtime helper and a narrow block-lower pattern that replaces
  the selected same-block get/copy/set helper pair with one direct-slot op.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_selected_method_array_slot_direct_op_owner_selection_guard.sh
```
