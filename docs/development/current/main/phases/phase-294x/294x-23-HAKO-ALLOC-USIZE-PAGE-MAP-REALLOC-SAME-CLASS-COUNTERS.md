---
Status: Complete
Date: 2026-05-22
Scope: migrate one page-map realloc same-class/no-move counter field group from
  `i64` to exact `usize`.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-22-HAKO-ALLOC-USIZE-PAGE-MAP-RELEASE-COUNTERS.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/page_map_realloc_same_class_box.hako
  - tools/checks/k2_wide_mimalloc_realloc_same_class_guard.sh
---

# 294x-23 Hako Alloc Usize Page-Map Realloc Same-Class Counters

## Decision

Migrate only the `HakoAllocPageMapReallocSameClassPath` event/reject counter
group:

- `same_class_count`
- `grow_reject_count`
- `lookup_miss_count`
- `stale_page_count`
- `released_block_count`
- `reject_count`

These fields are non-negative counters owned by the same-class/no-move realloc
path. They sit downstream of the page-map and release-seam counter migrations
and do not carry pointer, id, index, size, or signed-sentinel meaning.

## Stop Line

`last_result_ptr` remains `i64` in this row. It records the pointer-shaped
result observer and must not be grouped with event counters.

No page-map entry pointer/id fields, page/release observer fields, realloc
alloc-copy-release fallback fields, realloc failure-contract fields, provider
activation, host allocator replacement, hooks, or `#[global_allocator]` are
opened by this row.

## Acceptance

```bash
bash tools/checks/k2_wide_mimalloc_realloc_same_class_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
