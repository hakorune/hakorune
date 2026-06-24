---
Status: Landed
Date: 2026-05-29
Scope: define pinned typed-object arena storage substrate before DirectSlotLease implementation.
Blocker: PINNED-TYPED-OBJECT-ARENA-SSOT-296X-001
Related:
  - docs/development/current/main/design/pinned-typed-object-arena-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-306-TYPED-OBJECT-DIRECT-SLOT-LEASE-FEASIBILITY.md
---

# 296x-307 Pinned Typed Object Arena SSOT

## Purpose

Define the pinned typed-object arena required before `DirectSlotLease` or
`NativeDirect` typed-object lowering can be implemented.

This is a docs-only row. It does not implement a runtime arena and does not
change typed-object helper ABI or LLVM field lowering.

## Contract

```text
output_contract=pinned-typed-object-arena-ssot-v0
input_contract=typed-object-direct-slot-lease-feasibility-v0
selected_design_owner=pinned_typed_object_arena
selected_reason=current_vec_refcell_store_cannot_support_direct_slot_lease
default_backend_unchanged=1
existing_helper_abi_unchanged=1
pinned_arena_backend_default=0
object_storage_pinned_required=1
field_address_stable_required=1
object_generation_required=1
slot_layout_stable_required=1
handle_generation_validation_required=1
lease_region_required=1
lease_barrier_policy_required=1
raw_runtime_vec_pointer_exposure_allowed=0
silent_fallback_after_lease_selection_allowed=0
direct_lowering_before_arena_guard_allowed=0
by_name_hako_alloc_special_case_allowed=0
first_implementation_boundary=pinned_typed_object_arena_storage_pilot
implementation_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

The next implementation boundary is storage-only:

```text
PINNED-TYPED-OBJECT-ARENA-STORAGE-PILOT-296X-001
```

That row may add a pinned typed-object storage backend and generation/slot
stability smokes. It must keep direct lowering, DirectSlotLease emission,
provider activation, allocator replacement, hooks, globals, and winner claims
closed.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_pinned_typed_object_arena_ssot_guard.sh
```
