---
Status: Complete
Date: 2026-05-22
Scope: migrate one page queue counter field group from `i64` to exact `usize`.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-17-DIRECT-PAGE-SENTINEL-SPLIT.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/page_queue_box.hako
  - tools/checks/k2_wide_mimalloc_page_queue_guard.sh
---

# 294x-28 Hako Alloc Usize Page Queue Counters

## Decision

Migrate only the `HakoAllocPageQueue` stats counter group:

- `add_count`
- `select_count`
- `direct_hit_count`
- `refresh_count`
- `reject_count`

These fields are non-negative queue-local counters. They do not carry bin,
queue length, presence, or index meaning.

## Stop Line

`bin`, `page_count`, `has_direct_page`, and `direct_page_index` remain `i64` in
this row. `direct_page_index` no longer stores `-1`, but it is still an index
contract and must migrate separately from stats counters. `page_count` remains
signed while it drives the current loop/index comparisons.

No page model counters, page-map entry pointer/id fields, queue lifecycle
selectors, provider activation, host allocator replacement, hooks, or
`#[global_allocator]` are opened by this row.

## Acceptance

```bash
bash tools/checks/k2_wide_mimalloc_page_queue_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
