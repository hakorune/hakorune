# 293x-253 M208 Heap Reuse Priority Policy

Status: Complete

## Purpose

M208 adds a read-only policy owner that ranks existing heap pages before fresh
page creation using the frozen M207 lifecycle vocabulary. The row makes the
reuse ordering explicit without changing allocator behavior.

## Decision

Decision: accepted.

Add:

```text
lang/src/hako_alloc/memory/heap_reuse_priority_box.hako
```

The owner returns a read-only `HakoAllocHeapReusePriorityDecision` from
`HakoAllocHeapReusePriorityPolicy.selectHeapPage(...)`.

## Row Contract

M208 orders reuse buckets as:

```text
active page with acquire_allowed == 1
recommitted-active page with acquire_allowed == 1
retired page with reusable local/free blocks (reactivate route only)
fresh page fallback
```

Decommitted pages stay ineligible until recommit:

```text
state == 3 -> counted as blocked, never selected for direct reuse
```

## Stop Lines

- Do not mutate heap/page state.
- Do not call acquire, release, reactivate, decommit, recommit, or page-source APIs.
- Do not add scheduler, unreserve, or OS release behavior.
- Do not change provider activation, hooks, process allocator replacement, or
  packed-record/backend/materialization stop lines.

## Acceptance

- `HakoAllocHeapReusePriorityPolicy.selectHeapPage(...)` returns stable route
  decisions for:
  - active over recommitted-active
  - recommitted-active over retired
  - retired over fresh
  - fresh when only decommitted pages remain
- Pure-first EXE proof output matches the priority matrix.
- The guard confirms the policy box stays read-only and `.inc` matcher-free.
- M208 guard stays local-run / index-listed and is not added to quick/dev gates.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_heap_reuse_priority_policy_guard.sh
```
