---
Status: Landed
Date: 2026-05-29
Scope: define the lowering guard surface for the selected DirectSlotLease method before codegen changes.
Blocker: DIRECT-SLOT-LEASE-LOWERING-GUARD-SURFACE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-313-DIRECT-SLOT-LEASE-GUARD-SURFACE.md
  - docs/development/current/main/phases/phase-296x/296x-316-DIRECT-SLOT-LEASE-SELECTED-METHOD-INVENTORY.md
---

# 296x-317 Direct Slot Lease Lowering Guard Surface

## Purpose

Freeze the selected-method lowering guard before any compiler codegen change.

Row316 proved the selected method has positive helper-delta if a lease-backed
representation can be consumed without inserting C ABI helper load/writeback
calls. This row does not implement that lowering. It defines the exact
preconditions and fail-fast surface for the next implementation row.

## Contract

```text
output_contract=direct-slot-lease-lowering-guard-surface-v0
input_contract=direct-slot-lease-selected-method-inventory-v0
selected_owner=compiler_direct_slot_lease_lowering_guard
selected_method=HakoAllocPageModel.acquire_usize/1
selected_storage_backend=pinned_arena_exact
selected_storage_classes=i64|u64|handle
current_representation=ExactSlotObject
candidate_representation=NativeDirectViaDirectSlotLease
planned_erased_helper_ops=21
planned_added_helper_ops=0
planned_net_helper_delta=21
planned_net_helper_delta_positive=1
prior_resident_scalar_net_helper_call_delta=0
lease_acquire_c_abi_helper_count_required=0
materialization_helper_count_required=0
selected_method_only=1
receiver_exact_plan_required=1
slot_constant_required=1
storage_class_exact_required=1
pinned_arena_exact_backend_required=1
lease_token_runtime_available_required=1
addressable_slot_bridge_required=1
unknown_call_barrier_policy=no_plan
observer_barrier_policy=no_plan_or_explicit_materialization
escape_barrier_policy=no_plan_or_explicit_materialization
unsupported_storage_policy=keep_existing_helper_before_selection
selected_plan_silent_fallback_allowed=0
default_backend_exact_lease_emission=0
default_helper_abi_unchanged=1
new_c_abi_helper_symbols=0
raw_runtime_vec_pointer_exposure=0
by_name_hako_alloc_special_case=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
implementation_open=0
optimization_open=0
selected_next=direct_slot_lease_lowering_pilot
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Guard Rules

The next row may implement only a selected-method lowering pilot.

The pilot must fail the row if any selected field operation silently falls back
after lease selection. Fallback is allowed only before selection, when a field
does not satisfy the exact receiver/slot/storage/backend facts.

The pilot must not add public C ABI helper symbols. If a backend bridge cannot
express the selected slot access without adding helper calls, the row must stop
and select a bridge design row instead of pretending the plan is a keeper.

## Non-Goals

```text
generic typed-field residence retry
generic CSE
whole-program scalar replacement
raw runtime Vec pointer exposure
ArraySlot direct lowering
result capsule ValueAggregate retry
provider activation
allocator replacement
hooks
global allocator
winner claim
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_slot_lease_lowering_guard_surface_guard.sh
```
