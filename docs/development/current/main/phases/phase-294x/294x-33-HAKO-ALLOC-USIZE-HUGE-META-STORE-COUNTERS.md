---
Status: Complete
Date: 2026-05-23
Scope: migrate huge-page metadata store counters from `i64` to exact `usize`.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/huge_page_meta_store_box.hako
  - tools/checks/k2_wide_huge_page_metadata_record_store_guard.sh
  - tools/checks/k2_wide_mimalloc_huge_page_model_guard.sh
---

# 294x-33 Hako Alloc Usize Huge Meta Store Counters

## Decision

Migrate the C205d huge-page metadata store counters:

- `HakoAllocHugePageMetaStore.count`
- `HakoAllocHugePageMetaStore.live_count`

Both fields are owner-local non-negative counters. `count` tracks appended
metadata rows. `live_count` tracks live huge metadata rows and follows the same
guarded decrement shape already used by exact `usize` page-map live counters.

## Stop Line

Huge-page pointer, id, requested-size, committed-size, live-flag payload
columns, and indexed observer returns remain `i64`. This row does not migrate
`HakoAllocHugePageModel` page ids, pointers, sizes, status fields, or
model-level counters, and it does not open packed-record storage, OSVM, provider
activation, host allocator replacement, hooks, or `#[global_allocator]`.

## Acceptance

```bash
bash tools/checks/k2_wide_huge_page_metadata_record_store_guard.sh
bash tools/checks/k2_wide_mimalloc_huge_page_model_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
