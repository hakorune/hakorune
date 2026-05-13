# 293x-237 M197 Purge Decommit Heap Integration

Status: Complete

## Purpose

M197 composes the purge/decommit ladder with an existing OSVM-backed heap page:
dry-run observation, bounded decommit policy, and the page-source decommit
adapter now work together for empty retired page/backing state.

The integration does not mutate heap/page state itself. Unreserve and OS
release remain closed.

## Decision

Decision: accepted.

Add:

```text
lang/src/hako_alloc/memory/purge_heap_decommit_box.hako
```

`HakoAllocPurgeHeapDecommitIntegration.attemptHeapPage(heap, page_id)`:

1. observes page/backing state through `HakoAllocPurgeDryRunObserver`
2. reads `heap.pageBase(page_id)` and `heap.pageBackingBytes(page_id)` only
   when the decision is eligible
3. calls `HakoAllocBoundedDecommitPolicy.attemptDecommit(...)`
4. executes decommit through `HakoAllocPageSourceDecommitAdapter`

## Stop Lines

- Do not call page-source APIs directly from the integration owner.
- Do not call `heap.decommitPage(...)`.
- Do not mutate heap/page state from the integration owner.
- Do not unreserve pages.
- Do not release OSVM pages.
- Do not change allocation, release, realloc, aligned, or huge behavior.
- Do not add provider activation, hooks, or process allocator replacement.

## Acceptance

- Live page observation rejects before source decommit.
- Empty retired page observation reaches bounded decommit.
- Page-source decommit adapter is called exactly once for the eligible page.
- `unreserve_executed` and `os_release_executed` remain `0`.
- Existing heap `decommit_count` remains `0`; this row does not call the heap's
  direct decommit helper.
- Pure-first EXE proof output matches the integration matrix.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_purge_heap_decommit_guard.sh
```
