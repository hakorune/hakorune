---
Status: Current
Date: 2026-05-30
Scope: define DirectArray family as the long-term array storage substrate and ArrayBox as the public facade/materialized view before continuing legacy helper/cache retirement.
Blocker: DIRECTARRAY-FAMILY-STORAGE-SUBSTRATE-ROADMAP-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-373-ARRAYBOX-PUBLIC-SEMANTICS-AND-DIRECTARRAY-SPLIT-SSOT.md
  - crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs
  - tools/checks/k2_wide_phase296x_directarray_family_storage_substrate_roadmap_guard.sh
---

# 296x-374 DirectArray Family Storage Substrate Roadmap

## Purpose

Fix the long-term array direction before retiring more Array helper/cache code.

The short-term path keeps `ArrayBox` and `DirectArrayI64BufferV0` separate.
The long-term target is not two permanent primary array worlds. The target is:

```text
DirectArray family:
  long-term array storage substrate

ArrayBox:
  public facade / materialized view / fallback owner
```

`DirectArrayI64BufferV0` is the first exact-storage member of that family and
the allocator hot-path pilot.

## Contract

```text
output_contract=directarray-family-storage-substrate-roadmap-v0
input_contract=arraybox-public-semantics-and-directarray-split-ssot-v0
short_term_split_required=1
long_term_primary_storage=directarray_family
first_directarray_member=DirectArrayI64BufferV0
arraybox_long_term_role=public_facade|materialized_view|dynamic_fallback|generic_api
arraybox_long_term_performance_substrate=0
directarray_family_members_planned=i64|bool|f64|handle|boxed_optional_later
array_repr_layer_planned=1
array_repr_variants=DirectI64|DirectBool|DirectF64|DirectHandle|PublicArrayBoxFallback
stage0_array_seed=rust_keep
stage0_rust_array_seed_is_semantics_owner=0
array_semantics_owner=hako_ring1_array_core
array_storage_substrate=directarray_family
public_materialized_view=arraybox
rust_private_layout_as_semantic_truth=0
rust_private_layout_as_llvm_abi=0
nyash_array_birth_h_public_until_array_repr_promotion=1
nyash_array_direct_i64_birth_h_first_pilot=1
public_handle_reinterpret_as_direct=0
materialization_route_required=1
silent_fallback_allowed=0
phase_1=separate_direct_array_i64_birth_and_selected_native_direct_lowering
phase_2=directarray_i64_hot_path_primary_for_allocator_arrays
phase_3=arraybox_i64_exact_storage_can_be_directarray_backed
phase_4=arraybox_facade_over_directarray_family_for_exact_storage
phase_5=legacy_helper_routes_become_fallback_debug_generic_only
next_implementation_scope=single_thread_exact_array_helper_backend_retirement_only
selected_boundary=array_slot_nativedirect_legacy_helper_cache_retirement_implementation
next_diagnostic=array_slot_nativedirect_legacy_helper_cache_retirement_implementation
selected_next=array_slot_nativedirect_legacy_helper_cache_retirement_implementation
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Task Order

1. Keep `nyash.array.birth_h` as public ArrayBox birth.
2. Keep `nyash.array.direct_i64.birth_h` as the first DirectArray exact-storage
   birth.
3. Continue the current hot-path lane by retiring only the
   `single_thread_exact_array_helper_backend` surface that DirectArrayI64 has
   replaced.
4. Later, introduce an `ArrayRepr` layer instead of making ArrayBox internals a
   compiler ABI.
5. Move exact i64 ArrayBox storage to DirectArray-backed storage only after the
   public facade/materialization contract is proven.
6. Extend the DirectArray family only with explicit storage members and no
   silent fallback.
7. Keep stage0 Rust ArraySeed as a bootstrap/recovery keep until a separate
   cutover proves it can be retired.
8. Move visible collection semantics toward `.hako` ring1 ArrayCore, not into
   DirectArray or Rust private substrate.

## Non-Goals

- Do not replace all public ArrayBox behavior with DirectArrayI64.
- Do not make `nyash.array.birth_h` return a DirectArray pointer now.
- Do not expose ArrayBox plugin internals as LLVM ABI.
- Do not make both ArrayBox helper storage and DirectArray storage primary.
- Do not retire handle-entry cache or public helper fast lanes in this row.
- Do not read stage0 Rust ArraySeed as the final collection owner.
- Do not expose Rust `Vec`, `RefCell`, ArrayBox storage enum, or diagnostic
  cache layout as semantic truth or LLVM ABI.

## Decision

The user-facing model remains ArrayBox. The compiler performance model moves
toward DirectArray family. `DirectArrayI64BufferV0` is the first member and the
current allocator hot-path target.

## Stage / Owner Split

Stage0 keeps a Rust ArraySeed for bootstrap, buildability, and recovery. This
does not make Rust ArrayBox the final collection owner.

The long-term owner split is:

```text
stage0_array_seed=rust_keep
array_semantics_owner=hako_ring1_array_core
array_storage_substrate=directarray_family
public_materialized_view=arraybox
```

Meaning:

- Stage0 Rust ArraySeed may stay as a bootstrap/recovery keep.
- `.hako` ring1 ArrayCore owns user-visible collection semantics.
- DirectArray family owns raw storage / NativeDirect performance substrate.
- ArrayBox remains the public materialized facade and compatibility view.

Rust reliance at stage0 is not a failure. It is a bootstrap/substrate keep. The
forbidden shape is leaking Rust private collection layout as the semantic or
LLVM ABI truth.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_directarray_family_storage_substrate_roadmap_guard.sh
```
