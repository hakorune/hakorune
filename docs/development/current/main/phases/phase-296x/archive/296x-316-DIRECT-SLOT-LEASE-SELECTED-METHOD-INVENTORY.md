---
Status: Landed
Date: 2026-05-29
Scope: inventory HakoAllocPageModel.acquire_usize/1 DirectSlotLease helper-delta before lowering.
Blocker: DIRECT-SLOT-LEASE-SELECTED-METHOD-INVENTORY-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-315-DIRECT-SLOT-LEASE-COMPILER-PLAN-INVENTORY-SELECTION.md
  - tools/allocator/direct_slot_lease_selected_method_inventory.py
---

# 296x-316 Direct Slot Lease Selected Method Inventory

## Purpose

Inventory `HakoAllocPageModel.acquire_usize/1` under the DirectSlotLease plan
before opening any lowering implementation.

This row reuses the row302 ResidentScalar selected-method plan and row304
zero-net closeout as inputs. It does not change compiler lowering.

## Evidence

```text
output_contract=direct-slot-lease-selected-method-inventory-v0
input_contract=direct-slot-lease-compiler-plan-inventory-selection-v0
selected_method=HakoAllocPageModel.acquire_usize/1
selected_storage_backend=pinned_arena_exact
selected_storage_classes=i64|u64|handle
candidate_exact_slot_get_count=13
candidate_exact_slot_set_count=8
candidate_exact_slot_helper_count=21
resident_field_key_count=11
lease_acquire_count=11
lease_acquire_c_abi_helper_count=0
materialization_helper_count=0
planned_erased_helper_ops=21
planned_added_helper_ops=0
planned_net_helper_delta=21
planned_net_helper_delta_positive=1
prior_resident_scalar_inserted_helper_load_count=11
prior_resident_scalar_inserted_helper_writeback_count=8
prior_resident_scalar_net_helper_call_delta=0
barrier_policy=guard_surface_required_before_lowering
lowering_open=0
native_direct_open=0
direct_load_store_open=0
selected_next=direct_slot_lease_lowering_guard_surface
implementation_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

The selected method has a positive DirectSlotLease helper-delta inventory.

The next row may define a lowering guard surface. It must still keep code
generation closed until materialization/barrier policy and exact fallback are
explicit.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_slot_lease_selected_method_inventory_guard.sh
```
