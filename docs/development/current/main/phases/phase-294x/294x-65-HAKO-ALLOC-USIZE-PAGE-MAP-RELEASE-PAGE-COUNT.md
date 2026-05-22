---
Status: Landed
Date: 2026-05-23
Scope: page-map release seam page-array length exact `usize` migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-64-HAKO-ALLOC-USIZE-PAGE-MAP-RELEASE-PAGE-COUNT-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/page_map_release_box.hako
  - tools/checks/k2_wide_mimalloc_page_map_release_guard.sh
---

# 294x-65 Hako Alloc Usize Page Map Release Page Count

## Decision

Migrate only `HakoAllocPageMapReleaseSeam.page_count` to exact `usize`
storage.

`page_count` is the owner-local page-array length. `addPage(page)` accepts a
page only when `page.page_id == page_count`, pushes it into `pages`, then
increments `page_count`. `releasePtr(ptr)` keeps the signed page id seam
explicit: it rejects `page_id < 0` before comparing the non-negative signed
`page_id` against exact `usize` `page_count`.

## Stop Line

This row does not migrate:

- `HakoAllocPageMapEntry.ptr`, `page_id`, `block_id`, or `live`;
- `HakoAllocPageModel.page_id` or block ids;
- release seam event/reject counters already exact via
  `HAKO-ALLOC-USIZE-FIELD-GROUP-052`;
- provider activation, host allocator replacement, hooks, TLS, atomics, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_page_map_release_guard.sh
bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_small_path_slice_guard.sh
bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_realloc_aligned_slice_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
