---
Status: Landed
Date: 2026-05-29
Scope: select compiler-side DirectSlotLease plan inventory before any LLVM lowering change.
Blocker: DIRECT-SLOT-LEASE-COMPILER-PLAN-INVENTORY-SELECTION-296X-001
Related:
  - docs/development/current/main/design/direct-slot-lease-guard-surface-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-314-DIRECT-SLOT-LEASE-RUNTIME-TOKEN-PILOT.md
---

# 296x-315 Direct Slot Lease Compiler Plan Inventory Selection

## Purpose

Select the first compiler-side `DirectSlotLease` plan inventory row.

Runtime can now validate a lease token, but LLVM lowering is still closed. The
next step is an inventory that counts whether a selected method has a positive
helper-call delta after lease acquire/materialization boundaries.

## Contract

```text
output_contract=direct-slot-lease-compiler-plan-inventory-selection-v0
input_contract=direct-slot-lease-runtime-token-pilot-v0
selected_owner=compiler_direct_slot_lease_plan_inventory
selected_method=HakoAllocPageModel.acquire_usize/1
selected_reason=prior_resident_scalar_plan_had_21_candidate_ops_but_helper_load_writeback_zero_net
selected_storage_backend=pinned_arena_exact
selected_storage_classes=i64|u64|handle
inventory_only=1
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
runtime_token_available=1
helper_fallback_available=1
materialization_policy_required=1
positive_net_helper_delta_required=1
unknown_barrier_policy_fail_fast=1
default_backend_unchanged=1
existing_helper_abi_unchanged=1
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

Select `HakoAllocPageModel.acquire_usize/1` for the first lease plan inventory.

Reason:

- it was the selected typed-object ResidentScalar method;
- prior evidence found many candidate field ops but helper load/writeback made
  the plan zero-net;
- DirectSlotLease specifically exists to avoid that helper-backed zero-net.

The next row must count:

```text
candidate_exact_slot_get_count
candidate_exact_slot_set_count
lease_acquire_count
materialization_helper_count
barrier_count
planned_erased_helper_ops
planned_added_helper_ops
planned_net_helper_delta
```

No code generation changes are allowed until the inventory proves positive net.

## Next Row

```text
DIRECT-SLOT-LEASE-SELECTED-METHOD-INVENTORY-296X-001
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_slot_lease_compiler_plan_inventory_selection_guard.sh
```
