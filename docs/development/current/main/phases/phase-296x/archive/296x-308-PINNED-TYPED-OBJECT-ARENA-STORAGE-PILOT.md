---
Status: Landed
Date: 2026-05-29
Scope: implement the first storage-only pinned typed-object arena pilot without opening DirectSlotLease lowering.
Blocker: PINNED-TYPED-OBJECT-ARENA-STORAGE-PILOT-296X-001
Related:
  - docs/development/current/main/design/pinned-typed-object-arena-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-307-PINNED-TYPED-OBJECT-ARENA-SSOT.md
---

# 296x-308 Pinned Typed Object Arena Storage Pilot

## Purpose

Add the first storage-only `PinnedTypedObjectArena` substrate.

This row does not connect the arena to typed-object helper selection, does not
change the default store, and does not emit DirectSlotLease or NativeDirect
lowering.

## Contract

```text
output_contract=pinned-typed-object-arena-storage-pilot-v0
input_contract=pinned-typed-object-arena-ssot-v0
selected_owner=typed_object_runtime_storage
new_storage_box=typed_object_pinned_arena
default_backend_unchanged=1
existing_helper_abi_unchanged=1
pinned_arena_backend_default=0
pinned_object_allocation_smoke=ok
generation_validation_smoke=ok
slot_stability_smoke=ok
direct_lowering_open=0
direct_slot_lease_emission_open=0
native_direct_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

The storage substrate now has a small, test-covered pinned arena with:

- opaque negative handle encoding;
- generation validation;
- boxed object storage;
- boxed field storage;
- stable slot address under mutation.

The next row should decide how to connect this arena to the typed-object backend
selector without opening LLVM lowering.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_pinned_typed_object_arena_storage_pilot_guard.sh
```
