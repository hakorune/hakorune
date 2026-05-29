---
Status: Landed
Date: 2026-05-29
Scope: select the DirectArrayI64BufferV0 stable layout before any ArraySlot NativeDirect implementation.
Blocker: DIRECT-ARRAY-I64-BUFFER-V0-LAYOUT-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-348-ARRAY-SLOT-NATIVEDIRECT-GUARD-SURFACE.md
  - crates/nyash_kernel/src/plugin/array_slot_backend.rs
---

# 296x-349 Direct Array I64 Buffer V0 Layout Selection

## Purpose

Select the stable ArraySlot NativeDirect storage layout.

Row348 closed repeated Array runtime-helper thinning and selected a separate
NativeDirect representation for hot exact i64 Array slots. This row chooses the
ABI shape of that representation. It does not implement storage, connect a
backend, or open LLVM lowering.

## Contract

```text
output_contract=direct-array-i64-buffer-v0-layout-selection-v0
input_contract=array-slot-nativedirect-guard-surface-v0
selected_layout=DirectArrayI64BufferV0
selected_owner=array_slot_nativedirect_storage_layout
layout_repr=repr_c
header_kind=u32
header_flags=u32
header_generation=u32
header_element_tag=u32
header_len=u64
header_capacity=u64
header_size_bytes=32
header_alignment_bytes=8
element_layout=trailing_i64_slice
element_size_bytes=8
element_alignment_bytes=8
data0_offset_bytes=32
element_tag=i64
mixed_storage_supported=0
boxed_storage_supported=0
string_storage_supported=0
bool_f64_storage_supported=0
per_element_tag_supported=0
direct_slot_cell_reuse=0
public_arraybox_semantics_unchanged=1
default_safe_rwlock_path_unchanged=1
arraybox_items_rwlock_exposure=0
array_slot_cache_vec_exposure=0
plugin_runtime_helper_boundary_owner=fallback_materialization_debug
storage_pilot_open_next=1
materialization_policy=deferred_required_before_lowering
fallback_sync_policy=deferred_required_before_lowering
implementation_open=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
provider_activation=0
host_replacement=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
selected_next=direct_array_i64_buffer_v0_storage_pilot
summary=ok
```

## Layout

`DirectArrayI64BufferV0` is a stable, compiler-consumable storage substrate for
exact i64 Array slots.

```text
DirectArrayI64BufferV0:
  kind: u32
  flags: u32
  generation: u32
  element_tag: u32
  len: u64
  capacity: u64
  data: trailing [i64; capacity]
```

The selected layout uses one array-level element tag instead of per-element
`DirectSlotCellV0` tags:

```text
element_tag=i64
per_element_tag_supported=0
direct_slot_cell_reuse=0
```

This keeps the Array NativeDirect path close to a C-style contiguous `i64`
buffer. `DirectSlotCellV0` remains the typed-object field storage cell, not the
Array element representation.

## Ownership Boundary

`ArrayBox` remains the public runtime/plugin semantic owner. The current
`RwLock<ArrayStorage>` public path and the diagnostic `single_thread_exact`
helper path stay fallback/materialization/debug surfaces.

The NativeDirect buffer must not expose plugin internals:

```text
arraybox_items_rwlock_exposure=0
array_slot_cache_vec_exposure=0
```

## Closed Work

This row selects only the layout. It does not:

```text
implement storage allocation
connect the ArrayBox backend
route helper fallback
emit LLVM direct load/store
change .hako source
change public ArrayBox semantics
```

## Next Row

The next row may implement a storage-only pilot:

```text
selected_next=direct_array_i64_buffer_v0_storage_pilot
```

That pilot must keep LLVM lowering closed and prove header/data offsets with a
small Rust smoke.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_array_i64_buffer_v0_layout_selection_guard.sh
```
