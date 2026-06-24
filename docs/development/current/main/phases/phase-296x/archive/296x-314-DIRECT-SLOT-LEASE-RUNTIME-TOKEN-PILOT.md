---
Status: Landed
Date: 2026-05-29
Scope: implement runtime-internal DirectSlotLease token pilot while keeping LLVM lowering closed.
Blocker: DIRECT-SLOT-LEASE-RUNTIME-TOKEN-PILOT-296X-001
Related:
  - docs/development/current/main/design/direct-slot-lease-guard-surface-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-313-DIRECT-SLOT-LEASE-GUARD-SURFACE.md
---

# 296x-314 Direct Slot Lease Runtime Token Pilot

## Purpose

Implement a runtime-internal `DirectSlotLeaseToken` for the pinned typed-object
arena.

This row proves token validation and storage-class-specific read/write behavior
inside the runtime storage module. It does not expose C ABI helpers and does not
change LLVM lowering.

## Contract

```text
output_contract=direct-slot-lease-runtime-token-pilot-v0
input_contract=direct-slot-lease-guard-surface-v0
selected_owner=typed_object_direct_slot_lease_runtime_token
selected_storage_backend=pinned_arena_exact
selected_storage_classes=i64|u64|handle
lease_token_struct=1
lease_validate_i64_u64_handle=1
lease_read_write_smoke=ok
wrong_storage_reject_smoke=ok
existing_helper_abi_unchanged=1
default_backend_unchanged=1
new_c_abi_helper_symbols=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

The runtime can now represent a bounded direct slot lease token for i64, u64,
and handle slots in the pinned arena. The next row must select the compiler-side
lease plan/inventory surface before any lowering change.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_slot_lease_runtime_token_pilot_guard.sh
```
