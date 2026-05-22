---
Status: Landed
Date: 2026-05-23
Scope: OSVM-backed backing-array length exact `usize` id/index seam.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/osvm_backed_fast_path_heap_box.hako
  - tools/checks/k2_wide_mimalloc_osvm_page_source_composition_guard.sh
  - tools/checks/k2_wide_hako_alloc_mimalloc_comparison_huge_osvm_slice_guard.sh
---

# 294x-63 Hako Alloc Usize OSVM Backing Count Id Seam

## Decision

Migrate only `HakoAllocOsVmBackedFastPathHeap.backing_count` to exact `usize`
storage.

`backing_count` is the backing-array length. It starts at `0`, increments once
after a backing entry is pushed, bounds `backingFor(page_id)`, and drives
`decommitAll()`. The comparison seam remains explicit: `backingFor(page_id)`
first rejects negative signed `page_id` values, then compares the non-negative
signed `page_id` against exact `usize` `backing_count`.

This row is the narrow id/index seam needed before broader OSVM-backed backing
metadata can become exact.

## Stop Line

This row does not migrate:

- `HakoAllocOsVmBackedFastPathHeap.bin` or `next_page_id`;
- `HakoAllocOsVmBackedHandle.page_id` or `block_id`;
- `HakoAllocOsVmPageBacking.page_id` or `base`;
- OSVM native pointer representation, provider activation, host allocator
  replacement, hooks, TLS, atomics, or `#[global_allocator]`.

Page/block identity values remain signed until their own id/index contract rows.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_osvm_page_source_composition_guard.sh
bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_huge_osvm_slice_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
