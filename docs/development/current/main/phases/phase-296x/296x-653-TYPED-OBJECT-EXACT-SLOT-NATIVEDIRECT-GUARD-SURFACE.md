---
Status: Landed
Date: 2026-06-12
Scope: define the typed-object exact-slot NativeDirect guard surface before any pilot opens.
Blocker: TYPED-OBJECT-EXACT-SLOT-NATIVEDIRECT-GUARD-SURFACE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-652-TYPED-OBJECT-EXACT-SLOT-NATIVEDIRECT-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-296x/296x-651-HAKO-MIMALLOC-TYPED-OBJECT-EXACT-SLOT-ABI-SPLIT.md
  - docs/development/current/main/design/typed-object-exact-slot-abi-ssot.md
  - docs/development/current/main/design/pinned-typed-object-arena-ssot.md
  - docs/development/current/main/design/representation-direct-storage-substrate-ssot.md
---

# 296x-653 Typed Object Exact Slot NativeDirect Guard Surface

## Purpose

Define the exact-slot NativeDirect guard surface without opening lowering.

Row 652 showed that the current exact-helper bridge is still the active route
truth, while the NativeDirect candidate is still closed. This row freezes the
facts that must remain true until a later pilot can actually open:

- exact-helper bridge stays the current lowering form
- pinned typed-object arena is still the required substrate
- explicit materialized-view fallback boundary stays visible
- `TYPEDOBJ-ABI-004` remains the next seam, not the current one

## Contract

```text
output_contract=typed-object-exact-slot-nativedirect-guard-surface-v0
input_contract=typed-object-exact-slot-nativedirect-readiness-inventory-v0
candidate_representation=NativeDirect
selected_route=hako.typed_object.slot_load_i64
selected_lowering_form=exact_helper_bridge
storage_substrate=PinnedTypedObjectArena
fallback_boundary=explicit_materialized_view_handle
typed_object_native_direct_ready=0
typed_object_native_direct_open=0
typed_object_direct_load_store_open=0
object_storage_pinned_required=1
field_address_stable_required=1
object_generation_required=1
slot_layout_stable_required=1
handle_generation_validation_required=1
lease_region_required=1
lease_barrier_policy_required=1
silent_fallback_allowed=0
helper_load_writeback_substitution_allowed=0
raw_runtime_vec_pointer_exposure_allowed=0
by_name_hako_alloc_special_case_allowed=0
selected_next=typed_object_exact_slot_nativedirect_pilot_selection
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

The guard surface is a gate, not a pilot.

It keeps the helper-backed exact slot route visible while explicitly rejecting
any attempt to treat the current direct-state metadata as proof that NativeDirect
is ready. The pilot only opens after the pinned-arena and lease facts are proven
in the actual runtime/storage substrate row.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_typed_object_exact_slot_nativedirect_guard_surface_guard.sh
```
