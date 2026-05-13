# 293x-246 M202 Bounded Recommit Policy

Status: Complete

## Purpose

M202 opens the smallest execution-policy row after M201: a recommit attempt may
call a caller-provided `commitPage(base, bytes)` source only when M200 says the
page is decommitted and requires recommit.

This row does not wire the policy to the real page-source owner. It proves the
policy contract before a future adapter row can delegate to
`HakoAllocPageSourcePolicy.commitPage(...)`.

## Decision

Decision: accepted.

Add:

```text
lang/src/hako_alloc/memory/purge_bounded_recommit_box.hako
```

`HakoAllocBoundedRecommitPolicy.attemptRecommit(decision, source, base, bytes)`:

1. reads the M200 precondition report supplied by the caller
2. blocks missing pages and pages that do not require recommit
3. blocks zero base, non-positive bytes, and requests above the configured
   maximum byte count
4. calls only the caller-provided `source.commitPage(base, bytes)` executor
5. reports recommit success/failure without clearing markers or mutating heap
   page state

## Stop Lines

- Do not call `HakoAllocPageSourcePolicy` or `OsVmCoreBox` directly.
- Do not add a real page-source recommit adapter in this row.
- Do not clear or mutate the M198 decommit state marker.
- Do not mutate heap/page/backing state.
- Do not unreserve pages.
- Do not release OSVM pages.
- Do not change allocation, release, realloc, aligned, huge, or purge decommit
  behavior.
- Do not add provider activation, hooks, or process allocator replacement.

## Acceptance

- Committed/unmarked pages are blocked as "no recommit needed".
- M200 decommitted-page reports can execute exactly one caller-provided commit
  call when base/bytes are valid and within bounds.
- Oversized, missing-page, and source-reject cases return blocked reports.
- `marker_cleared`, `unreserve_executed`, and `os_release_executed` remain `0`.
- The proof app uses a fake caller-provided source; no page-source adapter is
  introduced.
- Pure-first EXE proof output matches the bounded recommit matrix.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_bounded_recommit_policy_guard.sh
```
