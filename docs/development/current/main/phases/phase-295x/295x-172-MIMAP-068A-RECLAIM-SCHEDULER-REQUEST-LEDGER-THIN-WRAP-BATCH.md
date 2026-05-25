---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap the MIMAP-068A reclaim scheduler request ledger guard root into an impl-backed wrapper and keep the memory README owner note in sync.
Related:
  - tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_guard.sh
---

# 295x-172 MIMAP-068A Reclaim Scheduler Request Ledger Thin-Wrap Batch

## Blocker

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
```

## Decision

Thin-wrap the MIMAP-068A reclaim scheduler request ledger guard root. The
validation semantics stay the same while the real shell body moves into
`tools/checks/impl/`.

Selected root:

- `k2_wide_hako_alloc_reclaim_scheduler_request_ledger_guard.sh`

## Cleanup

- Keep the root script as a thin wrapper that execs its impl body.
- Keep the reclaim scheduler request ledger owner note in the memory README.
- Leave the current mimalloc comparison blocker unchanged.

## Result

The MIMAP-068A request ledger guard is easier to scan at the root level.

## Stop Line

This batch does not open real scheduling, worker spawning, source concurrency,
atomics, page-source, unreserve, OS release, provider activation, host
allocator swap, or allocator replacement work.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
