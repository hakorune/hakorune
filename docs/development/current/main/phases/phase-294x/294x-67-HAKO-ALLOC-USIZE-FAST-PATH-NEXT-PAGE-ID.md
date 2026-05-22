---
Status: Landed
Date: 2026-05-23
Scope: fast-path heap next-page id exact `usize` migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-66-HAKO-ALLOC-USIZE-FAST-PATH-NEXT-PAGE-ID-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/alloc_fast_path_heap_box.hako
  - tools/checks/k2_wide_mimalloc_alloc_fast_path_guard.sh
---

# 294x-67 Hako Alloc Usize Fast Path Next Page Id

## Decision

Migrate only `HakoAllocFastPathHeap.next_page_id` to exact `usize` storage.

`next_page_id` is the non-OSVM fast-path heap's owner-local page-array length
and next-id source. `addPage()` copies the current value into a new
`HakoAllocPageModel`, pushes that page, then increments `next_page_id`.
`release(handle)` preserves the signed identity seam by rejecting
`handle.page_id < 0` before comparing the non-negative signed handle page id
against exact `usize` `next_page_id`.

## Stop Line

This row does not migrate:

- `HakoAllocFastPathHeap.bin`;
- `HakoAllocFastPathHandle.page_id` or `block_id`;
- `HakoAllocPageModel.page_id`;
- OSVM-backed `next_page_id` or backing payloads;
- provider activation, host allocator replacement, hooks, TLS, atomics, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_alloc_fast_path_guard.sh
bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_small_path_slice_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
