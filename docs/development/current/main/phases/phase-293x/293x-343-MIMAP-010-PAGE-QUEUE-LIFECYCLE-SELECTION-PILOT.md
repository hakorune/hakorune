# 293x-343 MIMAP-010 Page Queue Lifecycle Selection Pilot

Status: landed.
Decision: accepted.

## Goal

Add a lifecycle-aware page selection owner that can skip decommitted pages, reuse
eligible retired pages, and then select ordinary active pages.

## Landing shape

New owner:

```text
lang/src/hako_alloc/memory/page_queue_lifecycle_box.hako
```

The owner is intentionally separate from the older page queue so the mimalloc
lifecycle policy has a clear boundary.

## Guard

```bash
bash tools/checks/k2_wide_mimalloc_page_queue_lifecycle_selection_guard.sh
```

## Explicit non-goals

- no OSVM
- no segment source
- no provider activation
- no allocator hooks
- no host allocator replacement

Next selected row: `MIMAP-011 allocator facade lifecycle route pilot`.
