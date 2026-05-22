---
Status: Landed
Date: 2026-05-23
Scope: production page queue direct-page index exact `usize` migration.
Related:
  - lang/src/hako_alloc/memory/page_queue_box.hako
  - tools/checks/k2_wide_mimalloc_page_queue_guard.sh
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - docs/development/current/main/phases/phase-294x/294x-17-DIRECT-PAGE-SENTINEL-SPLIT.md
---

# 294x-49 Hako Alloc Usize Page Queue Direct Index

## Decision

Migrate only `HakoAllocPageQueue.direct_page_index` to exact `usize` storage.

The old `-1` sentinel was removed in `294x-17`; `has_direct_page` is the
presence flag. That makes `direct_page_index` a non-negative cache index, while
the presence flag itself remains signed until a bool/flag row.

## Stop Line

This row does not migrate `HakoAllocPageQueue.bin` or `has_direct_page`.

It does not migrate release-seam page ids/counts, provider activation, host
allocator replacement, hooks, or global allocator integration.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_page_queue_guard.sh
bash tools/checks/k2_wide_mimalloc_alloc_fast_path_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
