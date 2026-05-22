---
Status: Complete
Date: 2026-05-22
Scope: migrate one page-map realloc alloc-copy-release fallback counter field
  group from `i64` to exact `usize`.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-23-HAKO-ALLOC-USIZE-PAGE-MAP-REALLOC-SAME-CLASS-COUNTERS.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/page_map_realloc_alloc_copy_release_box.hako
  - tools/checks/k2_wide_mimalloc_realloc_alloc_copy_release_guard.sh
---

# 294x-24 Hako Alloc Usize Page-Map Realloc Alloc-Copy-Release Counters

## Decision

Migrate only the `HakoAllocPageMapReallocAllocCopyReleasePath` event/reject
counter group:

- `success_count`
- `copy_count`
- `same_class_reject_count`
- `alloc_fail_count`
- `lookup_miss_count`
- `stale_page_count`
- `released_block_count`
- `reject_count`

These fields are non-negative counters owned by the alloc-copy-release fallback
path. They sit downstream of the page-map, release-seam, and same-class/no-move
counter migrations and do not carry pointer, id, size, or signed-sentinel
meaning.

## Stop Line

`next_ptr`, `last_result_ptr`, `last_alloc_page_id`, and `last_alloc_block_id`
remain `i64` in this row. `next_ptr` / `last_result_ptr` are pointer-shaped
state, and the `last_alloc_*` fields use `-1` sentinels.

No page-map entry pointer/id fields, page/release observer fields, realloc
failure-contract fields, provider activation, host allocator replacement,
hooks, or `#[global_allocator]` are opened by this row.

## Acceptance

```bash
bash tools/checks/k2_wide_mimalloc_realloc_alloc_copy_release_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
