---
Status: Landed
Date: 2026-05-29
Scope: define the first DirectSlotLease guard surface now that pinned exact-slot fallback works.
Blocker: DIRECT-SLOT-LEASE-GUARD-SURFACE-296X-001
Related:
  - docs/development/current/main/design/direct-slot-lease-guard-surface-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-312-PINNED-TYPED-OBJECT-ARENA-EXACT-SLOT-HELPER-PILOT.md
---

# 296x-313 Direct Slot Lease Guard Surface

## Purpose

Define the first `DirectSlotLease` guard surface before any lease runtime API or
LLVM lowering implementation.

## Contract

```text
output_contract=direct-slot-lease-guard-surface-v0
input_contract=pinned-typed-object-arena-exact-slot-helper-pilot-v0
selected_owner=typed_object_direct_slot_lease_guard
selected_storage_backend=pinned_arena_exact
selected_storage_classes=i64|u64|handle
hako_alloc_policy_state_owner=unchanged
raw_memory_owner=capability_substrate_or_native_metal
representation_owner=compiler_direct_lowering
helper_path=fallback_materialization_debug
lease_token_runtime_smoke_open=1
helper_fallback_required=1
materialization_policy_required=1
barrier_policy_required=1
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
new_c_abi_helper_symbols=0
raw_runtime_vec_pointer_exposure=0
by_name_hako_alloc_special_case=0
silent_fallback_after_lease_selection=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

The next row may implement only a runtime-internal lease token pilot:

```text
DIRECT-SLOT-LEASE-RUNTIME-TOKEN-PILOT-296X-001
```

It must prove generation validation, storage-class validation, and stable slot
access against `pinned_arena_exact`. LLVM lowering and NativeDirect emission
remain closed.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_slot_lease_guard_surface_guard.sh
```
