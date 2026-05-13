# 293x-249 M205 Recommit Heap Integration

Status: Complete

## Purpose

M205 composes the recommit ladder into the heap/page model:

```text
M200 precondition
  -> M202 bounded recommit policy
  -> M203 page-source recommit adapter
  -> M204 marker transition
  -> HakoAllocPageModel.reactivate()
```

This is the first row that mutates page-local state after recommit. It does not
add page sourcing, unreserve, OS release, provider activation, hooks, or process
allocator replacement.

## Decision

Decision: accepted.

Add:

```text
lang/src/hako_alloc/memory/purge_recommit_heap_integration_box.hako
```

Extend:

```text
lang/src/hako_alloc/memory/page_box.hako
```

`HakoAllocPageModel.reactivate()` moves local-free blocks back to the free-list
and clears `retired` only when the page is empty and has available blocks.

The integration report status contract is code-local in
`purge_recommit_heap_integration_box.hako`. Nonzero status is always a blocked
final reuse attempt; status `3` is the defensive partial-success case where
recommit source execution succeeded but marker transition failed, so
`success_count` is intentionally not incremented.

## Stop Lines

- Do not allocate or source new pages in the recommit integration owner.
- Do not call reserve, decommit, unreserve, or OS release APIs.
- Do not mutate heap/backing arrays.
- Do not change allocation, release, realloc, aligned, or huge semantics beyond
  page-local reactivation after successful recommit.
- Do not add provider activation, hooks, or process allocator replacement.

## Acceptance

- A retired page rejects `acquire(...)` before recommit integration.
- Successful recommit transitions marker state and reactivates page-local state.
- Queue selection can see the reactivated page.
- A block can be acquired from the recommitted page.
- The owner documents its status code table and blocked-count semantics.
- Heap direct `decommit_count` remains `0`; setup decommit still flows through
  M199/M197.
- Pure-first EXE proof output matches the heap integration matrix.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_recommit_heap_integration_guard.sh
```
