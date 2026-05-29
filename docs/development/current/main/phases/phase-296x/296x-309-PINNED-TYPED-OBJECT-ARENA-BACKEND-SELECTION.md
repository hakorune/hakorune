---
Status: Landed
Date: 2026-05-29
Scope: select how the pinned typed-object arena connects to runtime backend selection without opening DirectSlotLease lowering.
Blocker: PINNED-TYPED-OBJECT-ARENA-BACKEND-SELECTION-296X-001
Related:
  - docs/development/current/main/design/pinned-typed-object-arena-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-308-PINNED-TYPED-OBJECT-ARENA-STORAGE-PILOT.md
---

# 296x-309 Pinned Typed Object Arena Backend Selection

## Purpose

Select the smallest backend-selection boundary for the storage-only pinned arena.

This row does not implement the backend connection. It fixes the next code row
so it cannot drift into DirectSlotLease emission, LLVM lowering, or helper
rewrites.

## Contract

```text
output_contract=pinned-typed-object-arena-backend-selection-v0
input_contract=pinned-typed-object-arena-storage-pilot-v0
selected_owner=typed_object_store_backend_selection
selected_backend_name=pinned_arena_exact
selection_env=HAKO_TYPED_OBJECT_STORE
allowed_env_values=safe_mutex|single_thread_exact|pinned_arena_exact
default_backend_unchanged=1
existing_helper_abi_unchanged=1
pinned_arena_backend_default=0
allowed_code_owner=crates/nyash_kernel/src/exports/typed_object_store.rs
allowed_storage_owner=crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs
allowed_export_owner=crates/nyash_kernel/src/exports/typed_object.rs
selected_helper_scope=generic_typed_object_helpers_only
exact_slot_helper_rewrite_open=0
direct_slot_lease_emission_open=0
llvm_lowering_open=0
native_direct_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

The next row may connect the pinned arena as an explicit runtime backend:

```text
HAKO_TYPED_OBJECT_STORE=pinned_arena_exact
```

The implementation must keep default behavior on `safe_mutex`, preserve the
existing C ABI symbols, and prove only generic typed-object helper roundtrips
through the pinned arena. Exact-slot helpers and DirectSlotLease remain closed.

## Next Row

```text
PINNED-TYPED-OBJECT-ARENA-BACKEND-PILOT-296X-001
```

Acceptance for that row must include:

```text
default_backend_smoke=ok
pinned_arena_generic_helper_smoke=ok
invalid_backend_fail_fast=1
exact_slot_helper_rewrite_open=0
direct_slot_lease_emission_open=0
summary=ok
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_pinned_typed_object_arena_backend_selection_guard.sh
```
