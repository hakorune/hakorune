---
Status: Landed
Date: 2026-05-23
Scope: production page queue length exact `usize` migration.
Related:
  - lang/src/hako_alloc/memory/page_queue_box.hako
  - tools/checks/k2_wide_mimalloc_page_queue_guard.sh
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - docs/development/current/main/phases/phase-294x/294x-17-DIRECT-PAGE-SENTINEL-SPLIT.md
---

# 294x-48 Hako Alloc Usize Page Queue Page Count

## Decision

Migrate only the `HakoAllocPageQueue.page_count` owner-local length field to
exact `usize` storage.

The direct-page cache no longer stores `-1` in `direct_page_index`; presence is
represented by `has_direct_page`, so the queue length can migrate independently
from the remaining signed index/presence fields.

## Stop Line

This row does not migrate `HakoAllocPageQueue.bin`, `has_direct_page`, or
`direct_page_index`.

It does not migrate release-seam `page_count`, page ids, queue indexes,
provider activation, host allocator replacement, hooks, or global allocator
integration.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_page_queue_guard.sh
bash tools/checks/k2_wide_mimalloc_alloc_fast_path_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
