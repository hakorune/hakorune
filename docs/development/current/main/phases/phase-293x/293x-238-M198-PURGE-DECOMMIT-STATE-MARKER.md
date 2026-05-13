# 293x-238 M198 Purge Decommit State Marker

Status: Complete

## Purpose

M198 adds a metadata/state marker for successful purge decommit reports. The
marker records which heap page ids have already reached the decommitted state
without mutating heap/page state, unreserving pages, releasing OSVM pages, or
replacing allocators.

This keeps M197 as the decommit integration owner and gives later rows a
separate state owner for duplicate-decommit prevention.

## Decision

Decision: accepted.

Add:

```text
lang/src/hako_alloc/memory/purge_decommit_state_marker_box.hako
```

`HakoAllocPurgeDecommitStateMarker.markIfDecommitted(page_id, report)`:

1. rejects invalid page ids
2. rejects missing or non-decommitted reports
3. rejects reports that claim unreserve or OS release execution
4. records the page id exactly once when `report.decommit_executed != 0`
5. reports duplicates without executing any source operation

## Stop Lines

- Do not call page-source APIs from the marker owner.
- Do not call `heap.decommitPage(...)`.
- Do not mutate heap/page/backing state from the marker owner.
- Do not unreserve pages.
- Do not release OSVM pages.
- Do not change allocation, release, realloc, aligned, huge, or purge execution
  behavior.
- Do not add provider activation, hooks, or process allocator replacement.

## Acceptance

- A live/ineligible decommit report is not marked.
- A successful decommit report marks the page once.
- Re-marking the same page reports an already-marked state.
- Marker counters expose attempts, successful marks, rejects, and duplicates.
- Existing heap `decommit_count` remains `0`; this row does not call the
  heap's direct decommit helper.
- Pure-first EXE proof output matches the marker matrix.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_purge_decommit_state_marker_guard.sh
```
