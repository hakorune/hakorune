---
Status: Landed
Date: 2026-05-12
Scope: hako_alloc direct-page stored sentinel split before usize migration.
Related:
  - lang/src/hako_alloc/memory/page_queue_box.hako
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - apps/mimalloc-page-queue-proof/
  - tools/checks/k2_wide_mimalloc_page_queue_guard.sh
---

# 294x-17 Direct Page Sentinel Split

## Decision

`HakoAllocPageQueue.direct_page_index` no longer uses `-1` as stored sentinel
state. The queue now stores:

```hako
has_direct_page: i64 = 0
direct_page_index: i64 = 0
```

`has_direct_page` owns the presence bit. `direct_page_index` is kept
non-negative so it can later migrate as an index candidate.

## Scope

Changed:

- split direct-page cache presence from the stored index;
- removed local `found_index = -1` from the refresh path;
- updated the page-queue proof and guard to check `direct=1,2,12`;
- updated the numeric field inventory from 36 to 37 stored numeric fields.

Deferred:

- return-value sentinel cleanup for `directPageId()` / `addPage(...)` /
  `HakoAllocPageModel.acquire(...)`;
- migration of `direct_page_index` or `has_direct_page` to `usize` / `bool`;
- page-map, OSVM, TLS, atomics, remote-free, and allocator fast-path behavior.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_page_queue_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
