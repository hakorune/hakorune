---
Status: Landed
Date: 2026-05-23
Scope: page-queue bin exact `usize` migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-70-HAKO-ALLOC-USIZE-PAGE-QUEUE-BIN-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/page_queue_box.hako
  - tools/checks/k2_wide_mimalloc_page_queue_guard.sh
---

# 294x-71 Hako Alloc Usize Page Queue Bin

## Decision

Migrate only `HakoAllocPageQueue.bin` to exact `usize` storage.

`bin` is the queue-local size-class index. It does not carry a negative
sentinel, pointer-like payload, or occupancy count. This row also tightens
`HakoAllocPageQueue.birth` to accept `bin: usize`, while keeping heap-level bin
mirrors and size-class policy return shapes outside this row.

## Stop Line

This row does not migrate:

- `HakoAllocFastPathHeap.bin`;
- `HakoAllocOsVmBackedFastPathHeap.bin`;
- `SizeClassBox.size_to_bin(...)` or `size_to_bin_usize(...)` return shape;
- `HakoAllocPageQueue.has_direct_page`;
- page identity, direct-page index, page/block identity payloads, pointer-like
  fields, or sentinel-returning methods;
- provider activation, host allocator replacement, hooks, TLS, atomics, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_page_queue_guard.sh
bash tools/checks/k2_wide_mimalloc_alloc_fast_path_guard.sh
bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_small_path_slice_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
