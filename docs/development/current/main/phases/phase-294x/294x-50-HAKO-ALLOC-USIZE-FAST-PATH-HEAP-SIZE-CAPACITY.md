---
Status: Landed
Date: 2026-05-23
Scope: production fast-path heap size/capacity exact `usize` migration.
Related:
  - lang/src/hako_alloc/memory/alloc_fast_path_heap_box.hako
  - lang/src/hako_alloc/memory/page_box.hako
  - tools/checks/k2_wide_mimalloc_alloc_fast_path_guard.sh
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
---

# 294x-50 Hako Alloc Usize Fast Path Heap Size Capacity

## Decision

Migrate only `HakoAllocFastPathHeap.block_size` and
`HakoAllocFastPathHeap.page_capacity` to exact `usize` storage.

`HakoAllocPageModel` already accepts exact page block-size and capacity fields.
Keeping the fast-path heap producer signed would preserve an unnecessary
integer seam at page creation time.

## Stop Line

This row does not migrate `HakoAllocFastPathHeap.bin` or `next_page_id`.
It also does not migrate `HakoAllocFastPathHandle.page_id`, `block_id`, or
`requested_size`.

It does not open OSVM/page-source ownership, remote-free ownership, provider
activation, host allocator replacement, hooks, or global allocator integration.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_alloc_fast_path_guard.sh
bash tools/checks/k2_wide_mimalloc_osvm_page_source_composition_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
