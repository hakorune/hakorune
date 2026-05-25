---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap the M215 thread heap owner-token inventory guard root into an impl-backed wrapper.
Related:
  - tools/checks/k2_wide_hako_alloc_thread_heap_owner_inventory_guard.sh
---

# 295x-167 M215 Thread Heap Owner-Inventory Thin-Wrap Batch

## Blocker

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
```

## Decision

Thin-wrap the M215 thread heap owner-token inventory guard root. The validation
semantics stay the same while the real shell body moves into
`tools/checks/impl/`.

Selected root:

- `k2_wide_hako_alloc_thread_heap_owner_inventory_guard.sh`

## Cleanup

- Keep the root script as a thin wrapper that execs its impl body.
- Keep the thread heap owner-token inventory owner note in the memory README.
- Leave the current mimalloc comparison blocker unchanged.

## Result

The M215 inventory guard is easier to scan at the root level.

## Stop Line

This batch does not open real thread scheduling, atomic claim, remote-free
drain, owner mutation, reclaim execution, page-source, unreserve, OS release,
provider activation, or host allocator swap work.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_thread_heap_owner_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
