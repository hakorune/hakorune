# 293x-259 M212 Bounded Purge Decommit Scheduler Small Path

Status: Complete

## Purpose

M212 adds a bounded purge/decommit scheduler small path.

The row scans at most a caller-provided number of pages, consumes M207
lifecycle reports through the M211 candidate inventory, and delegates the
first eligible page to the M199 state-aware decommit guard.

## Decision

Decision: accepted.

Add:

```text
lang/src/hako_alloc/memory/purge_bounded_scheduler_box.hako
```

The owner exposes:

```text
HakoAllocBoundedPurgeDecommitScheduler.run(heap, guard, max_scan_pages)
```

M212 may call:

```text
HakoAllocPageLifecycleInvariantObserver.observeHeapPage(...)
HakoAllocPurgeCandidatePolicyInventory.classifyLifecycleReport(...)
HakoAllocPurgeStateAwareDecommitGuard.attemptHeapPage(...)
```

M212 must not call M197, M195, M196, or page-source APIs directly.
M199 remains the duplicate-guarded execution seam.

## Stop Lines

- Do not scan beyond `max_scan_pages`.
- Do not attempt more than one candidate per invocation.
- Do not call M197 directly.
- Do not call M195 directly.
- Do not call page-source APIs directly.
- Do not call `heap.decommitPage(...)`.
- Do not mutate heap/page/backing state in the scheduler owner.
- Do not recommit.
- Do not unreserve or release OSVM pages.
- Do not source, reserve, or commit fresh pages.
- Do not add provider activation, hooks, env toggles, or allocator replacement.
- Do not add backend `.inc` app/name matchers.
- Do not change allocation, release, realloc, or reuse priority behavior.

## Acceptance

- `max_scan_pages = 0` scans zero pages and attempts nothing.
- Active pages are observed and rejected without an M199 attempt.
- A retired eligible page within the bound is attempted once through M199.
- An eligible page beyond the bound is not reached.
- Two eligible pages within the bound still attempt only the first page.
- A duplicate run with the same guard does not execute source a second time.
- A page over the bounded decommit byte limit reports attempted-but-blocked.
- Pure-first EXE proof output matches the decision matrix.
- The guard confirms no direct page-source/M197/M195/M196, provider, hook,
  backend matcher, unreserve, OS release, or gate-growth leak.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_bounded_purge_decommit_scheduler_guard.sh
```
