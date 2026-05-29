---
Status: Landed
Date: 2026-05-29
Scope: select the narrow owner for selected-method DirectSlot NativeDirect lowering implementation.
Blocker: DIRECT-SLOT-NATIVEDIRECT-LOWERING-OWNER-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-334-DIRECT-SLOT-NATIVEDIRECT-LOWERING-GUARD-SURFACE.md
  - src/llvm_py/instructions/field_access.py
---

# 296x-335 Direct Slot NativeDirect Lowering Owner Selection

## Purpose

Select the implementation owner for the first DirectSlot NativeDirect lowering
attempt.

This row does not implement lowering. It chooses the smallest owner so the next
row can make one focused implementation attempt without changing MIRBuilder,
`.hako` source, or runtime helper semantics.

## Contract

```text
output_contract=direct-slot-nativedirect-lowering-owner-selection-v0
input_contract=direct-slot-nativedirect-lowering-guard-surface-v0
selected_owner=llvm_field_access_direct_slot_nativedirect_selected_method_hook
selected_owner_file=src/llvm_py/instructions/field_access.py
selected_runtime_layout_owner=crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs
selected_method=HakoAllocPageModel.acquire_usize/1
selected_backend=direct_slot_exact
selected_reason=field_access_py_already_owns_exact_typed_object_field_lowering_and_can_remain_a_thin_plan_consumer
mirbuilder_changes_allowed=0
hako_source_changes_allowed=0
runtime_helper_semantics_changes_allowed=0
generic_direct_slot_rewrite_allowed=0
selected_method_only=1
direct_slot_exact_only=1
default_backend_emission=0
fallback_boundary=explicit_materialized_view_handle
silent_fallback_allowed=0
helper_load_writeback_substitution_allowed=0
by_name_hako_alloc_special_case=0
implementation_open=0
optimization_open=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
selected_next=direct_slot_nativedirect_lowering_selected_method_pilot
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Rejected Owners

```text
rejected_owner=mirbuilder_direct_slot_rewrite
reason=semantic facts already exist and this row needs a lowerer-only pilot

rejected_owner=runtime_helper_fast_lane
reason=the micro-helper lane is closed and NativeDirect must remove helper calls

rejected_owner=hako_alloc_source_rewrite
reason=hako_alloc remains the semantic owner and should not carry compiler workaround shape

rejected_owner=generic_field_access_rewrite
reason=first pilot must be selected-method only with silent fallback forbidden
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_slot_nativedirect_lowering_owner_selection_guard.sh
```
