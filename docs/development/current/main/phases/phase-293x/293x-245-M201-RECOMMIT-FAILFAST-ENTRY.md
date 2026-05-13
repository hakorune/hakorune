# 293x-245 M201 Recommit Fail-Fast Entry

Status: Complete

## Purpose

M201 adds the first explicit recommit attempt entry after M200 classified
decommitted pages as unavailable.

This is still blocked/report-only. It creates a structured place for future
recommit behavior, but does not recommit pages, call page-source APIs, unreserve
pages, release OSVM pages, or mutate heap/page state.

## Decision

Decision: accepted.

Add:

```text
lang/src/hako_alloc/memory/purge_recommit_failfast_box.hako
```

`HakoAllocRecommitFailFastEntry.attemptHeapPage(heap, page_id, marker)`:

1. delegates to `HakoAllocDecommittedPageReusePrecondition`
2. returns no-op reusable for committed/unmarked pages
3. returns blocked/recommit-required for decommitted pages
4. returns missing-page facts for unknown page ids
5. keeps `recommit_executed=0` and `source_executed=0`

## Stop Lines

- Do not call page-source APIs from the recommit fail-fast owner.
- Do not call commit/decommit/reserve helpers.
- Do not call heap direct decommit helpers.
- Do not mutate heap/page/backing state.
- Do not remove or clear the M198 decommit marker.
- Do not unreserve pages.
- Do not release OSVM pages.
- Do not change allocation, release, realloc, aligned, huge, or purge execution
  behavior.
- Do not add provider activation, hooks, or process allocator replacement.

## Acceptance

- An unmarked heap page returns no-op reusable with no execution.
- A page marked decommitted by M199 returns blocked with
  `requires_recommit=1`.
- Unknown page ids return missing-page facts.
- Recommit/source execution counters remain zero.
- Existing page-source decommit adapter call count remains `1`; this row only
  observes the already-marked state.
- Existing heap `decommit_count` remains `0`; this row does not call the
  heap's direct decommit helper.
- Pure-first EXE proof output matches the recommit fail-fast matrix.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_recommit_failfast_guard.sh
```
