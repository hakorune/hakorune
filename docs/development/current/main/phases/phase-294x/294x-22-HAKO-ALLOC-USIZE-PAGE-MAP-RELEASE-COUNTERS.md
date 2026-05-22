---
Status: Complete
Date: 2026-05-22
Scope: migrate one page-map release-seam event/reject counter field group from
  `i64` to exact `usize`.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-21-HAKO-ALLOC-USIZE-PAGE-MAP-COUNTERS.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/page_map_release_box.hako
  - tools/checks/k2_wide_mimalloc_page_map_release_guard.sh
---

# 294x-22 Hako Alloc Usize Page-Map Release Counters

## Decision

Migrate only the `HakoAllocPageMapReleaseSeam` event/reject counter group:

- `page_register_count`
- `release_count`
- `unregister_count`
- `lookup_miss_count`
- `stale_page_count`
- `page_release_reject_count`
- `reject_count`

These fields are non-negative counters owned by the release seam and are
downstream of the already migrated `HakoAllocPageMap` counters.

## Stop Line

`page_count` remains `i64` in this row. It is compared with signed `page_id`
values in the release seam, so migrating it requires a separate field-group row
with an explicit page-id / page-count comparison contract.

No page-map entry pointer/id fields, page model fields, release observer
fields, realloc paths, provider activation, host allocator replacement, hooks,
or `#[global_allocator]` are opened by this row.

## Acceptance

```bash
bash tools/checks/k2_wide_mimalloc_page_map_release_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
