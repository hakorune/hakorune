---
Status: Landed
Date: 2026-05-29
Scope: select the next pinned arena boundary after generic helper backend pilot.
Blocker: PINNED-TYPED-OBJECT-ARENA-NEXT-LEASE-BOUNDARY-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-310-PINNED-TYPED-OBJECT-ARENA-BACKEND-PILOT.md
  - docs/development/current/main/design/pinned-typed-object-arena-ssot.md
---

# 296x-311 Pinned Typed Object Arena Next Lease Boundary Selection

## Purpose

Decide whether to proceed directly to `DirectSlotLease` guard planning or first
make `pinned_arena_exact` compatible with exact-slot helper fallback.

## Contract

```text
output_contract=pinned-typed-object-arena-next-lease-boundary-selection-v0
input_contract=pinned-typed-object-arena-backend-pilot-v0
selected_next=pinned_arena_exact_slot_helper_compatibility
selected_reason=direct_slot_lease_needs_existing_exact_slot_helper_fallback_to_work_with_pinned_backend
generic_helper_backend_smoke=ok
exact_slot_helper_with_pinned_backend_supported=0
direct_slot_lease_guard_ready=0
selected_owner=typed_object_store_exact_slot_helper_backend
allowed_scope=exact_slot_get_set_rmw_record_helpers_only
default_backend_unchanged=1
existing_helper_abi_unchanged=1
pinned_arena_backend_default=0
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

Select exact-slot helper compatibility before DirectSlotLease guard planning.

Reason:

- current exact `.hako` lowering already uses exact-slot helper fallback;
- `pinned_arena_exact` currently proves only generic helper roundtrips;
- a DirectSlotLease row still needs a correct fallback/materialization path;
- therefore exact-slot helper compatibility is the next storage-layer boundary.

## Next Row

```text
PINNED-TYPED-OBJECT-ARENA-EXACT-SLOT-HELPER-PILOT-296X-001
```

That row may route existing exact-slot helper functions through
`pinned_arena_exact`. It must not add new helper symbols, change LLVM lowering,
or emit DirectSlotLease / NativeDirect.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_pinned_typed_object_arena_next_lease_boundary_selection_guard.sh
```
