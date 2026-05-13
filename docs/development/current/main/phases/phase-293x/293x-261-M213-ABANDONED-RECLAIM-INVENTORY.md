# 293x-261 M213 Abandoned Reclaim Inventory

Status: Complete

## Purpose

M213 adds abandoned/reclaim vocabulary as a read-only inventory row.

The row names abandoned ownership and reclaim candidates without adding thread
scheduling, atomics expansion, reclaim execution, page-source calls, unreserve,
or OS release behavior.

## Decision

Decision: accepted.

Add:

```text
lang/src/hako_alloc/memory/abandoned_reclaim_inventory_box.hako
```

The owner exposes:

```text
HakoAllocAbandonedReclaimInventory.classifyPage(...)
```

M213 classifies:

```text
missing backing -> rejected
active or same owner -> rejected
remote free pending -> rejected
decommitted -> rejected
abandoned live page -> reclaim candidate
abandoned empty retired page -> reclaim candidate and future purge-forward candidate
```

All execution booleans stay false:

```text
would_schedule_reclaim = 0
would_reclaim = 0
would_atomic_claim = 0
would_decommit = 0
would_unreserve = 0
would_release_osvm = 0
```

## Stop Lines

- Do not schedule threads.
- Do not add atomics.
- Do not execute reclaim.
- Do not call page-source APIs.
- Do not decommit or recommit.
- Do not unreserve or release OSVM pages.
- Do not mutate heap/page/marker/page-source state.
- Do not add provider activation, hooks, env toggles, or allocator replacement.
- Do not add backend `.inc` app/name matchers.
- Do not alter allocation, release, realloc, purge scheduling, or reuse
  priority behavior.

## Acceptance

- `HakoAllocAbandonedReclaimInventory.classifyPage(...)` returns stable
  structured decisions for missing backing, active owner, same owner, remote
  pending, decommitted, abandoned live, and abandoned empty-retired inputs.
- VM and pure-first EXE proof output match the decision matrix.
- The guard confirms no thread scheduling, atomics, page-source, execution,
  provider, hook, backend matcher, or gate-growth leak.
- M213 guard stays local-run / index-listed and is not added to quick/dev gates.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_abandoned_reclaim_inventory_guard.sh
```

