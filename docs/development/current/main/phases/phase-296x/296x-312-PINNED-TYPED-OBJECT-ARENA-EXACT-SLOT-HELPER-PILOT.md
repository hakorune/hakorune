---
Status: Landed
Date: 2026-05-29
Scope: route existing exact-slot helper fallback through pinned_arena_exact before DirectSlotLease planning.
Blocker: PINNED-TYPED-OBJECT-ARENA-EXACT-SLOT-HELPER-PILOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-311-PINNED-TYPED-OBJECT-ARENA-NEXT-LEASE-BOUNDARY-SELECTION.md
  - docs/development/current/main/design/pinned-typed-object-arena-ssot.md
---

# 296x-312 Pinned Typed Object Arena Exact Slot Helper Pilot

## Purpose

Route existing exact-slot helper fallback through `pinned_arena_exact`.

This row does not add new helper symbols, does not change LLVM lowering, and
does not emit DirectSlotLease or NativeDirect code.

## Contract

```text
output_contract=pinned-typed-object-arena-exact-slot-helper-pilot-v0
input_contract=pinned-typed-object-arena-next-lease-boundary-selection-v0
selected_owner=typed_object_store_exact_slot_helper_backend
selected_backend_name=pinned_arena_exact
exact_slot_helper_with_pinned_backend_supported=1
generic_helper_backend_smoke=ok
exact_slot_helper_smoke=ok
default_backend_unchanged=1
existing_helper_abi_unchanged=1
new_helper_symbol_count=0
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

The pinned backend can now serve existing exact-slot helper fallback. This makes
the storage substrate ready for a DirectSlotLease guard-surface row.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_pinned_typed_object_arena_exact_slot_helper_pilot_guard.sh
```
