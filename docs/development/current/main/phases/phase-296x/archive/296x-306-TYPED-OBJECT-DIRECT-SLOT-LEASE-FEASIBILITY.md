---
Status: Landed
Date: 2026-05-29
Scope: check whether current typed-object storage can support DirectSlotLease without a pinned storage rewrite.
Blocker: TYPED-OBJECT-DIRECT-SLOT-LEASE-FEASIBILITY-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-305-REPRESENTATION-DIRECT-STORAGE-SUBSTRATE-SSOT.md
  - docs/development/current/main/design/representation-direct-storage-substrate-ssot.md
---

# 296x-306 Typed Object Direct Slot Lease Feasibility

## Purpose

Check whether the current typed-object runtime store can support
`DirectSlotLease` without a storage rewrite.

This row does not implement runtime storage. It answers the storage substrate
question raised by row305.

## Evidence

```text
output_contract=typed-object-direct-slot-lease-feasibility-v0
input_contract=representation-direct-storage-substrate-ssot-v0
workload_id=representative-object-lifecycle-small-block-v0
current_store_kind=safe_mutex_or_single_thread_refcell_vec
single_thread_exact_backend_exists=1
object_storage_container=Vec<TypedSlotObject>
field_storage_container=Vec<TypedSlot>
object_generation_available=0
object_storage_pinned=0
field_address_stable=0
vec_reallocation_possible=1
borrow_lifetime_representable_in_llvm=0
direct_slot_lease_feasible_without_storage_change=0
raw_runtime_vec_pointer_exposure_allowed=0
required_runtime_storage_change=pinned_typed_object_arena
selected_next=pinned_typed_object_arena_ssot
implementation_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
direct_slot_lease_feasible_without_storage_change=0
selected_next=pinned_typed_object_arena_ssot
implementation_open=0
```

Current typed-object storage uses `Vec<TypedSlotObject>` with `Vec<TypedSlot>`
fields behind `Mutex` or `RefCell`. It does not provide generation checks,
stable object/field addresses, or a borrow lifetime that LLVM can represent.

The next row must define a pinned typed-object arena before any DirectSlotLease
implementation.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_typed_object_direct_slot_lease_feasibility_guard.sh
```
