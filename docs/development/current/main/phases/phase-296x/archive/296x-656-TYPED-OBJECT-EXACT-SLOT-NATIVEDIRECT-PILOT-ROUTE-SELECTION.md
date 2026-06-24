---
Status: Landed
Date: 2026-06-12
Scope: select the first typed-object exact-slot NativeDirect route decision when direct-state facts are ready.
Blocker: TYPED-OBJECT-EXACT-SLOT-NATIVEDIRECT-PILOT-ROUTE-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-655-TYPED-OBJECT-EXACT-SLOT-NATIVEDIRECT-PILOT-IMPLEMENTATION.md
  - docs/development/current/main/phases/phase-296x/296x-654-TYPED-OBJECT-EXACT-SLOT-NATIVEDIRECT-PILOT-SELECTION.md
  - docs/development/current/main/phases/phase-296x/296x-653-TYPED-OBJECT-EXACT-SLOT-NATIVEDIRECT-GUARD-SURFACE.md
  - docs/development/current/main/design/typed-object-exact-slot-abi-ssot.md
  - src/mir/route_decision.rs
---

# 296x-656 Typed Object Exact Slot NativeDirect Pilot Route Selection

## Purpose

Select the first typed-object exact-slot NativeDirect route decision when the
direct-state facts prove the selected field is ready for inline direct
load/store lowering.

Row 655 introduced the lowering seam. This row makes the MIR planner actually
choose `native_direct` for the ready exact-slot lane, while keeping the
semantic route fixed at `hako.typed_object.slot_load_i64`.

## Contract

```text
output_contract=typed-object-exact-slot-nativedirect-pilot-route-selection-v0
input_contract=typed-object-exact-slot-nativedirect-pilot-implementation-v0
candidate_representation=NativeDirect
selected_owner=mir_route_decision_typed_object_exact_slot_nativedirect_pilot_route_selection
selected_owner_file=src/mir/route_decision.rs
selected_backend=typed_object_exact_slot_nativedirect
selected_route=hako.typed_object.slot_load_i64
selected_lowering_form=native_direct
selected_bridge_symbol=none
storage_substrate=PinnedTypedObjectArena
fallback_boundary=explicit_materialized_view_handle
required_facts=object_storage_pinned_required|field_address_stable_required|object_generation_required|slot_layout_stable_required|handle_generation_validation_required|lease_region_required|lease_barrier_policy_required
native_direct_ready=1
pilot_open=1
implementation_open=1
optimization_open=1
llvm_lowering_open=1
native_direct_open=1
direct_load_store_open=1
route_decision_native_direct_supported=1
helper_bridge_default_unchanged=1
helper_bridge_fallback_removed=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
selected_next=typed_object_exact_slot_nativedirect_native_direct_smoke
summary=ok
```

## Decision

When the route planner sees a ready direct-state plan for the receiver field,
it now selects `native_direct` instead of the exact-helper bridge. The
semantic route remains the typed-object exact-slot route in the `hako.*`
namespace; only the lowering form changes.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_typed_object_exact_slot_nativedirect_pilot_route_selection_guard.sh
```
