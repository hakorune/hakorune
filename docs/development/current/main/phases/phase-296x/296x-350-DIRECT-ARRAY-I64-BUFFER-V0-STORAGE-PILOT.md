---
Status: Landed
Date: 2026-05-29
Scope: implement a storage-only DirectArrayI64BufferV0 pilot while keeping ArrayBox and lowering unchanged.
Blocker: DIRECT-ARRAY-I64-BUFFER-V0-STORAGE-PILOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-349-DIRECT-ARRAY-I64-BUFFER-V0-LAYOUT-SELECTION.md
  - crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs
---

# 296x-350 Direct Array I64 Buffer V0 Storage Pilot

## Purpose

Implement the storage-only `DirectArrayI64BufferV0` pilot.

This row proves the selected row349 layout with concrete allocation, header/data
offset checks, contiguous i64 store/load, append-at-end behavior, and OOB
preservation. It does not connect the buffer to public `ArrayBox`, does not add
helper ABI symbols, and does not open LLVM lowering.

## Evidence

```text
output_contract=direct-array-i64-buffer-v0-storage-pilot-v0
input_contract=direct-array-i64-buffer-v0-layout-selection-v0
implemented_owner=crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs
module_registered=crates/nyash_kernel/src/plugin/mod.rs
implemented_layout=DirectArrayI64BufferV0
layout_repr=repr_c
header_size_bytes=32
header_alignment_bytes=8
data0_offset_bytes=32
element_layout=trailing_i64_slice
element_size_bytes=8
element_alignment_bytes=8
element_tag=i64
allocation_stable=1
contiguous_i64_store_load_smoke=ok
append_at_end_smoke=ok
oob_preservation_smoke=ok
zero_generation_rejected=1
storage_only_dead_code_allowance=1
public_arraybox_semantics_unchanged=1
default_safe_rwlock_path_unchanged=1
existing_array_helper_abi_unchanged=1
backend_connection_open=0
materialization_policy=deferred_required_before_lowering
fallback_sync_policy=deferred_required_before_lowering
arraybox_items_rwlock_exposure=0
array_slot_cache_vec_exposure=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
provider_activation=0
host_replacement=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
selected_next=direct_array_i64_materialization_sync_ssot
summary=ok
```

## Decision

`DirectArrayI64BufferV0` is an ArraySlot NativeDirect storage substrate, not a
public `ArrayBox` replacement. It stores only exact i64 elements and has one
array-level element tag. Mixed or boxed storage remains a public ArrayBox/helper
fallback concern.

The storage pilot intentionally remains disconnected from the existing helper
path. The next row must define materialization/fallback sync before any backend
connection or LLVM direct load/store emission opens.

Because this is a storage-only substrate, the module may carry a local
`dead_code` allowance until a backend connection row consumes the API. The
allowance is not a general warning suppression policy and should be removed when
the buffer is wired into a selected backend.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_array_i64_buffer_v0_storage_pilot_guard.sh
```
