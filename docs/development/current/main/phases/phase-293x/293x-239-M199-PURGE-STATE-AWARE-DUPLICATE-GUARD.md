# 293x-239 M199 Purge State-Aware Duplicate Guard

Status: Complete

## Purpose

M199 consumes the M198 decommit state marker before executing the M197 heap
decommit integration. Pages already marked as decommitted are blocked before
the page-source adapter can run.

This keeps duplicate-decommit prevention outside the page-source adapter and
outside heap/page state mutation.

## Decision

Decision: accepted.

Add:

```text
lang/src/hako_alloc/memory/purge_state_aware_decommit_box.hako
```

`HakoAllocPurgeStateAwareDecommitGuard.attemptHeapPage(heap, page_id)`:

1. checks `HakoAllocPurgeDecommitStateMarker.isMarked(page_id)`
2. blocks already-marked pages before calling M197 integration
3. delegates unmarked pages to `HakoAllocPurgeHeapDecommitIntegration`
4. marks successful decommit reports through the M198 marker
5. returns a structured report with source-execution and marker state facts

## Stop Lines

- Do not call page-source APIs directly from the state-aware guard owner.
- Do not call `heap.decommitPage(...)`.
- Do not mutate heap/page/backing state from the guard owner.
- Do not unreserve pages.
- Do not release OSVM pages.
- Do not change allocation, release, realloc, aligned, huge, or purge policy
  behavior.
- Do not add provider activation, hooks, or process allocator replacement.

## Acceptance

- A live/ineligible page reaches M197 and remains source-execution inactive.
- The first eligible empty/retired page decommit executes once and marks state.
- A repeated attempt for the same page is blocked before M197/M196 source
  execution.
- Adapter call count remains `1` after live, first, and duplicate attempts.
- Existing heap `decommit_count` remains `0`; this row does not call the
  heap's direct decommit helper.
- Pure-first EXE proof output matches the duplicate-guard matrix.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_purge_state_aware_duplicate_guard.sh
```
