---
Status: Landed
Date: 2026-05-29
Scope: select how explicit DirectSlot materialization creates a separate helper-compatible TypedSlot view handle.
Blocker: DIRECT-SLOT-MATERIALIZED-VIEW-HANDLE-POLICY-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-329-DIRECT-SLOT-BACKEND-HELPER-FALLBACK-ROUTING-SELECTION.md
  - crates/nyash_kernel/src/exports/typed_object_store.rs
---

# 296x-330 Direct Slot Materialized View Handle Policy Selection

## Purpose

Select the handle policy for explicit DirectSlot materialization.

The materialized view must be helper-compatible without making `TypedSlot` a
second source of truth for the DirectSlot object. Therefore the view gets a
separate handle and separate storage. Existing helpers may operate on that view
handle only after an explicit materialization boundary creates it.

## Contract

```text
output_contract=direct-slot-materialized-view-handle-policy-selection-v0
input_contract=direct-slot-backend-helper-fallback-routing-selection-v0
selected_owner=direct_slot_materialized_view_negative_handle_store
selected_reason=helper_compatibility_requires_a_separate_materialized_typed_slot_view_not_per_call_snapshot_routing
direct_handle_kind=tagged_stable_direct_slot_object_pointer
direct_handle_sign=positive_tagged
materialized_view_handle_kind=typed_slot_view_handle
materialized_view_handle_sign=negative_index_handle
materialized_view_storage=separate_thread_local_typed_slot_object_vec
materialized_view_source=explicit_direct_slot_snapshot
materialized_view_lifetime=until_store_reset_or_process_exit
existing_helper_route_to_direct_backend=0
existing_helper_route_to_materialized_view_handle=1
existing_helper_route_to_snapshot_per_call=0
direct_cell_primary_storage=1
typed_slot_role=materialized_view_not_primary_storage
view_writeback_to_direct_slot=0
dual_truth_allowed=0
implicit_sync_on_every_direct_write=0
new_c_abi_helper_symbols=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
implementation_open=0
optimization_open=0
selected_next=direct_slot_materialized_view_handle_pilot
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

Use two handle spaces inside `direct_slot_exact`:

```text
DirectSlot object handle:
  positive tagged stable pointer

Materialized view handle:
  negative typed-slot view index
```

Existing typed-object helpers stay closed for the positive DirectSlot handle.
They may route to the negative materialized view handle after an explicit
materialization call creates that view.

## Rejected

```text
rejected=helper_reads_positive_direct_slot_handle
reason=would_hide_materialization_and_make_fallback_path_look_like_primary_storage

rejected=materialized_view_writes_back_to_direct_slot
reason=would_reintroduce_dual_truth_before_a_writeback_policy_exists

rejected=per_call_snapshot_view
reason=work_explosion_and_hidden_materialization
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_slot_materialized_view_handle_policy_selection_guard.sh
```
