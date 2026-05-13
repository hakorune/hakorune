# 293x-268 M215 Thread Heap Owner Token Inventory

Status: Complete
Decision: accepted
Date: 2026-05-14

## Purpose

M215 adds a read-only `.hako` owner for thread heap owner-token inventory.

The row names owner-token facts needed by future abandoned/reclaim work without
scheduling threads, using atomics, draining remote frees, mutating owner state,
calling page-source APIs, unreserving, releasing OSVM pages, activating
providers, installing hooks, or replacing the process allocator.

## Landed files

| Path | Role |
| --- | --- |
| `docs/development/current/main/design/hako-alloc-thread-heap-owner-inventory-ssot.md` | M215 owner-token inventory SSOT. |
| `lang/src/hako_alloc/memory/thread_heap_owner_inventory_box.hako` | Read-only owner-token inventory owner. |
| `apps/hako-alloc-thread-heap-owner-inventory-proof/` | VM / pure-first EXE proof app for M215. |
| `tools/checks/k2_wide_hako_alloc_thread_heap_owner_inventory_guard.sh` | Local-run guard for the M215 proof and stop lines. |

## Fixed decisions

M215 records only read-only facts:

```text
unknown owner token
same-thread owner token
active foreign owner token
remote-free pending reject
decommitted reject
abandoned inactive owner-token candidate
```

All execution booleans stay false:

```text
would_schedule_thread = 0
would_atomic_claim = 0
would_drain_remote_free = 0
would_change_page_owner = 0
would_execute_reclaim = 0
would_call_page_source = 0
would_unreserve = 0
would_release_osvm = 0
```

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_thread_heap_owner_inventory_guard.sh
```

## Next blocker

```text
D209 mimalloc post-M215 closeout check
```
