---
Status: Landed
Date: 2026-05-29
Scope: close the selected DirectSlotLease lowering pilot as not implementable without a helper-free addressable slot bridge.
Blocker: DIRECT-SLOT-LEASE-LOWERING-PILOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-317-DIRECT-SLOT-LEASE-LOWERING-GUARD-SURFACE.md
  - src/llvm_py/instructions/field_access.py
  - crates/nyash_kernel/src/exports/typed_object_store.rs
  - crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs
---

# 296x-318 Direct Slot Lease Lowering Pilot Feasibility Closeout

## Purpose

Evaluate the row317 lowering pilot before making codegen changes.

The guard surface requires a helper-free addressable slot bridge. The current
implementation has a runtime-internal lease token, but LLVM lowering can only
reach typed-object storage through existing helper calls. Implementing the pilot
now would either add helper calls or expose runtime storage unsafely, so this row
closes the pilot and selects a bridge SSOT.

## Evidence

```text
output_contract=direct-slot-lease-lowering-pilot-feasibility-v0
input_contract=direct-slot-lease-lowering-guard-surface-v0
selected_method=HakoAllocPageModel.acquire_usize/1
selected_storage_backend=pinned_arena_exact
selected_storage_classes=i64|u64|handle
current_exact_lowering_owner=src/llvm_py/instructions/field_access.py
current_exact_lowering_path=exact_slot_helper_call
runtime_store_owner=crates/nyash_kernel/src/exports/typed_object_store.rs
runtime_arena_owner=crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs
runtime_lease_token_visibility=rust_internal_only
runtime_arena_location=thread_local_refcell_pinned_arena
addressable_slot_bridge_available=0
helper_free_bridge_possible_now=0
new_c_abi_helper_symbols_allowed=0
raw_runtime_vec_pointer_exposure_allowed=0
row317_selected_plan_silent_fallback_allowed=0
pilot_codegen_opened=0
pilot_implemented=0
rejection_reason=missing_helper_free_addressable_slot_bridge
selected_next=direct_slot_lease_addressable_slot_bridge_ssot
implementation_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

Do not implement DirectSlotLease lowering yet.

The next row must define the addressable slot bridge. It must choose how LLVM can
consume a selected slot without adding C ABI load/writeback helpers and without
exposing raw runtime `Vec` pointers.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_slot_lease_lowering_pilot_feasibility_closeout_guard.sh
```
