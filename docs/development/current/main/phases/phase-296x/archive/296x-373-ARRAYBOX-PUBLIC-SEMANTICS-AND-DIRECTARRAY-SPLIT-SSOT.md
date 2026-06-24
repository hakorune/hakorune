---
Status: Landed
Date: 2026-05-30
Scope: freeze the responsibility split between public ArrayBox semantics and DirectArrayI64 NativeDirect hot storage before legacy helper/cache retirement implementation.
Blocker: ARRAYBOX-PUBLIC-SEMANTICS-AND-DIRECTARRAY-SPLIT-SSOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-372-ARRAY-SLOT-NATIVEDIRECT-LEGACY-HELPER-CACHE-RETIREMENT-SELECTION.md
  - crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs
  - crates/nyash_kernel/src/plugin/array_slot_backend.rs
  - tools/checks/k2_wide_phase296x_arraybox_public_semantics_and_directarray_split_ssot_guard.sh
---

# 296x-373 ArrayBox Public Semantics And DirectArray Split SSOT

## Purpose

Freeze the responsibility split before retiring legacy Array helper/cache code.

ArrayBox is not removed. It retreats from the NativeDirect performance path and
remains the public materialized array object. DirectArrayI64BufferV0 owns exact
i64 hot storage for proven NativeDirect regions.

## Contract

```text
output_contract=arraybox-public-semantics-and-directarray-split-ssot-v0
input_contract=array-slot-nativedirect-legacy-helper-cache-retirement-selection-v0
public_arraybox_owner=plugin_runtime_public_semantics
direct_array_owner=native_direct_i64_hot_storage_substrate
nyash_array_birth_h_public=1
nyash_array_direct_i64_birth_h_separate=1
public_arraybox_handle_reinterpret_as_direct=0
public_arraybox_roles=public_object|dynamic_mixed_storage|debug_observer|materialization|fallback|generic_api
direct_array_roles=exact_i64_hot_storage|native_direct_lowering_receiver|c_like_array_access
arraybox_performance_substrate_role=0
plugin_internal_cache_as_llvm_abi=0
direct_array_materialization_route_required=1
silent_fallback_allowed=0
legacy_retirement_scope=single_thread_exact_array_helper_backend
handle_entry_cache_retirement_deferred=1
public_helper_fast_lane_retirement_deferred=1
selected_boundary=directarray_family_storage_substrate_roadmap
next_diagnostic=directarray_family_storage_substrate_roadmap
selected_next=directarray_family_storage_substrate_roadmap
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Responsibility Split

ArrayBox remains the owner for:

- public Array object semantics
- dynamic or mixed storage
- string / boxed / bool / f64 / generic values
- plugin API compatibility
- debug, observer, materialized view, and fallback routes
- non-hot or unproven array access

DirectArrayI64BufferV0 owns:

- exact i64 hot storage
- DirectArray handle identity
- compiler-consumable stable layout
- NativeDirect lowering receiver representation
- C-like load/store path for proven regions

## Forbidden

- Do not change `nyash.array.birth_h` to return DirectArray.
- Do not reinterpret a public ArrayBox host handle as a DirectArray pointer.
- Do not expose ArrayBox internal `RwLock`, `Vec`, or diagnostic cache layout as
  LLVM ABI.
- Do not silently fall back from a selected DirectArray NativeDirect plan.
- Do not delete public ArrayBox semantics during legacy helper/cache retirement.

## Decision

The next row fixes the long-term direction: DirectArray family becomes the
array storage substrate, and ArrayBox becomes the public facade/materialized
view. Implementation remains closed until that task order is fixed.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_arraybox_public_semantics_and_directarray_split_ssot_guard.sh
```
