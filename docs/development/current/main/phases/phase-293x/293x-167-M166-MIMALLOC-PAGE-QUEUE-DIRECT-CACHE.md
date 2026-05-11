---
Status: Complete
Date: 2026-05-11
Scope: M166 `.hako` mimalloc page queue/direct-page cache.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - lang/src/hako_alloc/memory/page_box.hako
  - lang/src/hako_alloc/memory/page_queue_box.hako
  - apps/mimalloc-page-queue-proof/
---

# 293x-167 M166 Mimalloc Page Queue Direct Cache

## Goal

Add a page-selection owner on top of the M165 page model without adding the
allocator fast path.

`HakoAllocPageQueue` owns per-bin page ordering and the direct-page cache. It
selects a page by observing `freeCount()` and never pops a block from the page.
Block allocation remains the M167 owner.

## Changes

- Added `lang/src/hako_alloc/memory/page_queue_box.hako` with
  `HakoAllocPageQueue`.
- Exported `memory.page_queue_box` from `hako_module.toml`.
- Added `apps/mimalloc-page-queue-proof/` to prove:
  - pages can be added to a bin queue;
  - direct-page cache hits avoid refresh;
  - full cached pages refresh to the next page with free capacity;
  - empty queues fail explicitly;
  - newly added pages restore the direct-page cache.
- Added a focused M166 guard that rejects `page_queue_box.hako` calling
  `.acquire(...)`.

## Stop Line

M166 does not add block allocation fast-path ownership, generic fallback,
OSVM page sourcing, local-free collection/retire, remote-free integration,
page-map lookup, provider activation, hook install, process allocator
replacement, or `.inc` name matching.

## Proof

```bash
bash tools/checks/k2_wide_mimalloc_page_queue_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

M167 may wire page selection to block allocation. It should use the queue owner
for page selection and the page owner for the block pop, while still keeping
OSVM fresh-page sourcing out of scope.
