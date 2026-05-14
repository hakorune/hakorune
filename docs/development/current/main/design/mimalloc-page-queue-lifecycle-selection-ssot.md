# MIMAP-010 page queue lifecycle selection SSOT

Decision: accepted.

`MIMAP-010` adds a separate lifecycle-aware page selection owner instead of
forcing the older page queue to carry mimalloc lifecycle policy.

## Owner

```text
lang/src/hako_alloc/memory/page_queue_lifecycle_box.hako
```

The owner observes scalar page lifecycle state and selects pages in this order:

1. skip decommitted pages
2. select retired pages only when the caller reports reusable state
3. select active pages with available blocks
4. report a miss if no page can satisfy the request

## Non-goals

- no OSVM
- no segment ownership
- no remote-free/TLS policy
- no provider activation
- no host allocator replacement

## Proof and guard

```text
apps/mimalloc-page-queue-lifecycle-selection-proof/main.hako
tools/checks/k2_wide_mimalloc_page_queue_lifecycle_selection_guard.sh
```

Next row: `MIMAP-011 allocator facade lifecycle route pilot`.
