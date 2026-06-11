---
Status: Landed
Date: 2026-06-12
Scope: inventory whether the typed-object exact-slot lane has enough storage and route facts to open a NativeDirect pilot.
Blocker: TYPED-OBJECT-EXACT-SLOT-NATIVEDIRECT-READINESS-INVENTORY-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-651-HAKO-MIMALLOC-TYPED-OBJECT-EXACT-SLOT-ABI-SPLIT.md
  - docs/development/current/main/design/typed-object-exact-slot-abi-ssot.md
  - docs/development/current/main/design/pinned-typed-object-arena-ssot.md
  - docs/development/current/main/design/representation-direct-storage-substrate-ssot.md
---

# 296x-652 Typed Object Exact Slot NativeDirect Readiness Inventory

## Purpose

Inventory the current exact-slot lane against the NativeDirect preconditions
without opening lowering.

The current route truth stays on `exact_helper_bridge`. This row only records
whether the MIR-side direct-state facts are present and whether the storage
substrate facts required by `TYPEDOBJ-ABI-004` are still missing.

## Evidence

```text
output_contract=typed-object-exact-slot-nativedirect-readiness-inventory-v0
input_contract=typed-object-exact-slot-direct-helper-measurement-v0
workload_id=representative-object-lifecycle-small-block-v0
candidate_representation=NativeDirect
storage_substrate=PinnedTypedObjectArena
fallback_boundary=explicit_materialized_view_handle
typed_object_direct_state_plan_count=9
typed_object_direct_state_field_count=79
typed_object_direct_state_selected_count=5
typed_object_direct_state_selected_field_count=33
typed_object_native_direct_candidate_count=5
typed_object_native_direct_ready=0
typed_object_native_direct_open=0
typed_object_direct_load_store_open=0
typed_object_native_direct_storage_substrate=PinnedTypedObjectArena
typed_object_native_direct_fallback_boundary=explicit_materialized_view_handle
typed_object_native_direct_selected_next=typed_object_exact_slot_nativedirect_guard_surface
typed_object_exact_helper_call_count=726
typed_object_exact_lowering_forms=exact_helper_bridge
typed_object_exact_internal_dispatch_count=0
typed_object_exact_silent_fallback_count=0
summary=ok
```

## Decision

NativeDirect is still closed for this lane.

The exact-slot route already has the helper-backed bridge in place, but the
pinned arena and lease facts needed for a helper-free hot region are not opened
here. This keeps `TYPEDOBJ-ABI-004` as the next seam without pretending the
pilot is ready.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_typed_object_exact_slot_nativedirect_readiness_inventory_guard.sh
```
