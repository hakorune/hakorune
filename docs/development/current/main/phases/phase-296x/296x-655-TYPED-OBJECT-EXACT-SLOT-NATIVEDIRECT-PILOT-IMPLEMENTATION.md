---
Status: Landed
Date: 2026-06-12
Scope: add the typed-object exact-slot NativeDirect lowering seam while keeping the pilot closed.
Blocker: TYPED-OBJECT-EXACT-SLOT-NATIVEDIRECT-PILOT-IMPLEMENTATION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-654-TYPED-OBJECT-EXACT-SLOT-NATIVEDIRECT-PILOT-SELECTION.md
  - docs/development/current/main/phases/phase-296x/296x-653-TYPED-OBJECT-EXACT-SLOT-NATIVEDIRECT-GUARD-SURFACE.md
  - docs/development/current/main/design/typed-object-exact-slot-abi-ssot.md
  - src/llvm_py/instructions/field_access_helpers_typed.py
---

# 296x-655 Typed Object Exact Slot NativeDirect Pilot Implementation

## Purpose

Add the first typed-object exact-slot NativeDirect lowering seam.

This row wires a `native_direct` route decision into the exact-slot typed-object
lowering helper so selected field get/set paths can use direct payload loads and
stores when the route decision explicitly requests it. The pilot remains closed
because the planner still defaults to the exact-helper bridge, and the route
decision producer has not been changed.

## Contract

```text
output_contract=typed-object-exact-slot-nativedirect-pilot-implementation-v0
input_contract=typed-object-exact-slot-nativedirect-pilot-selection-v0
selected_owner=llvm_field_access_typed_object_exact_slot_nativedirect_pilot_implementation
selected_owner_file=src/llvm_py/instructions/field_access_helpers_typed.py
selected_backend=typed_object_exact_slot_nativedirect
selected_route=hako.typed_object.slot_load_i64
selected_lowering_form=native_direct
storage_substrate=PinnedTypedObjectArena
fallback_boundary=explicit_materialized_view_handle
required_facts=object_storage_pinned_required|field_address_stable_required|object_generation_required|slot_layout_stable_required|handle_generation_validation_required|lease_region_required|lease_barrier_policy_required
implementation_open=1
pilot_open=0
optimization_open=0
llvm_lowering_open=1
native_direct_open=0
direct_load_store_open=1
route_decision_native_direct_supported=1
helper_bridge_default_unchanged=1
helper_bridge_fallback_removed=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
selected_next=typed_object_exact_slot_nativedirect_pilot_route_selection
summary=ok
```

## Decision

The lowering helper now recognizes a `native_direct` route decision for the
typed-object exact-slot lane and emits direct payload load/store operations for
that decision. The default exact-helper bridge remains the route truth until the
planner and the remaining pinned-arena / lease facts prove that the pilot can be
opened safely.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_typed_object_exact_slot_nativedirect_pilot_implementation_guard.sh
```
