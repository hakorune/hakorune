---
Status: Landed
Date: 2026-05-29
Scope: connect pinned_arena_exact as an explicit typed-object runtime backend for generic helper smokes only.
Blocker: PINNED-TYPED-OBJECT-ARENA-BACKEND-PILOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-309-PINNED-TYPED-OBJECT-ARENA-BACKEND-SELECTION.md
  - docs/development/current/main/design/pinned-typed-object-arena-ssot.md
---

# 296x-310 Pinned Typed Object Arena Backend Pilot

## Purpose

Connect `pinned_arena_exact` as an explicit `HAKO_TYPED_OBJECT_STORE` backend
for generic typed-object helper smokes.

This row keeps exact-slot helper rewriting, DirectSlotLease emission, and LLVM
NativeDirect lowering closed.

## Contract

```text
output_contract=pinned-typed-object-arena-backend-pilot-v0
input_contract=pinned-typed-object-arena-backend-selection-v0
selected_owner=typed_object_store_backend_selection
selected_backend_name=pinned_arena_exact
selection_env=HAKO_TYPED_OBJECT_STORE
allowed_env_values=safe_mutex|single_thread_exact|pinned_arena_exact
default_backend_unchanged=1
existing_helper_abi_unchanged=1
pinned_arena_backend_default=0
pinned_arena_generic_helper_smoke=ok
default_backend_smoke=ok
invalid_backend_fail_fast=1
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

The pinned arena can now be selected for generic typed-object helper roundtrips:

```bash
HAKO_TYPED_OBJECT_STORE=pinned_arena_exact
```

This only proves runtime storage connectivity. The next row must decide whether
to extend the pinned backend to exact-slot helpers or start DirectSlotLease guard
planning.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_pinned_typed_object_arena_backend_pilot_guard.sh
```
