# 293x-240 M200 Decommitted Page Reuse Precondition

Status: Complete

## Purpose

M200 freezes the reuse rule for pages recorded by the M198 decommit state
marker: a page marked as decommitted is not reusable until a later row provides
an explicit recommit/reopen path.

This is a state/contract row only. It does not recommit pages, unreserve pages,
release OSVM pages, or mutate heap/page state.

## Decision

Decision: accepted.

Add:

```text
lang/src/hako_alloc/memory/purge_decommitted_page_reuse_precondition_box.hako
```

`HakoAllocDecommittedPageReusePrecondition.classifyHeapPage(heap, page_id, marker)`:

1. reads existing heap page/backing observers
2. reads the M198 marker state
3. returns reusable for committed/unmarked pages
4. returns blocked/requires-recommit for decommitted pages
5. returns missing-page facts for unknown page ids

## Stop Lines

- Do not call page-source APIs from the precondition owner.
- Do not call `commitPage(...)`, `decommitPage(...)`, or `heap.decommitPage(...)`.
- Do not mutate heap/page/backing state from the precondition owner.
- Do not recommit pages.
- Do not unreserve pages.
- Do not release OSVM pages.
- Do not change allocation, release, realloc, aligned, huge, or purge execution
  behavior.
- Do not add provider activation, hooks, or process allocator replacement.

## Acceptance

- An unmarked heap page with backing is classified reusable.
- A page marked decommitted by M199 is classified blocked with
  `requires_recommit=1`.
- Unknown page ids return missing-page facts.
- Adapter call count remains `1`; this row only observes state after M199.
- Existing heap `decommit_count` remains `0`; this row does not call the
  heap's direct decommit helper.
- Pure-first EXE proof output matches the reuse precondition matrix.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_decommitted_page_reuse_precondition_guard.sh
```
