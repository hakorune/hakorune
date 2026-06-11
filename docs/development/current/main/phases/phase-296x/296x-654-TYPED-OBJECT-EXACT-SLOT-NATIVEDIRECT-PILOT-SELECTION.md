---
Status: Landed
Date: 2026-06-12
Scope: select the first typed-object exact-slot NativeDirect pilot owner without opening lowering.
Blocker: TYPED-OBJECT-EXACT-SLOT-NATIVEDIRECT-PILOT-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-653-TYPED-OBJECT-EXACT-SLOT-NATIVEDIRECT-GUARD-SURFACE.md
  - docs/development/current/main/phases/phase-296x/296x-651-HAKO-MIMALLOC-TYPED-OBJECT-EXACT-SLOT-ABI-SPLIT.md
  - docs/development/current/main/design/typed-object-exact-slot-abi-ssot.md
  - docs/development/current/main/design/pinned-typed-object-arena-ssot.md
  - docs/development/current/main/design/representation-direct-storage-substrate-ssot.md
---

# 296x-654 Typed Object Exact Slot NativeDirect Pilot Selection

## Purpose

Select the first typed-object exact-slot NativeDirect pilot owner while keeping
the pilot closed.

Row 652 proved the exact-helper bridge is still the current route truth.
Row 653 froze the pinned-arena and lease prerequisites. This row chooses the
owner for the first real NativeDirect pilot so that a later implementation row
can open only after the storage and lease facts are proven.

## Contract

```text
output_contract=typed-object-exact-slot-nativedirect-pilot-selection-v0
input_contract=typed-object-exact-slot-nativedirect-guard-surface-v0
candidate_representation=NativeDirect
selected_owner=llvm_field_access_typed_object_exact_slot_nativedirect_pilot_selection
selected_owner_file=src/llvm_py/instructions/field_access_helpers_typed.py
selected_backend=typed_object_exact_slot_nativedirect
selected_route=hako.typed_object.slot_load_i64
selected_lowering_form=exact_helper_bridge
storage_substrate=PinnedTypedObjectArena
fallback_boundary=explicit_materialized_view_handle
required_facts=object_storage_pinned_required|field_address_stable_required|object_generation_required|slot_layout_stable_required|handle_generation_validation_required|lease_region_required|lease_barrier_policy_required
pilot_open=0
implementation_open=0
optimization_open=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
selected_next=typed_object_exact_slot_nativedirect_pilot_implementation
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

The pilot owner is fixed, but the pilot is still closed.

This row exists so the next implementation seam is unambiguous once the pinned
arena and lease facts are available. It does not change route selection, helper
semantics, or LLVM lowering.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_typed_object_exact_slot_nativedirect_pilot_selection_guard.sh
```
