---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: M215 read-only thread heap owner-token inventory surface.
Related:
  - docs/development/current/main/design/mimalloc-migration-closeout-check-ssot.md
  - docs/development/current/main/design/mimalloc-port-remaining-inventory-ssot.md
  - docs/development/current/main/design/purge-lifecycle-ladder-closeout-ssot.md
  - lang/src/hako_alloc/memory/thread_heap_owner_inventory_box.hako
  - apps/hako-alloc-thread-heap-owner-inventory-proof/
---

# Hako Alloc Thread Heap Owner Inventory SSOT

## Decision

M215 adds a read-only `.hako` thread heap owner-token inventory surface.

The owner classifies scalar page owner facts for future abandoned/reclaim work.
It does not schedule threads, claim ownership, drain remote frees, mutate page
owners, call page-source APIs, unreserve, release OS memory, activate providers,
install hooks, or replace the process allocator.

## Owner

```text
lang/src/hako_alloc/memory/thread_heap_owner_inventory_box.hako
```

Responsibilities:

```text
classify unknown owner token
classify same-thread owner token
classify active foreign owner token
classify remote-free pending owner token
classify decommitted owner token
classify abandoned inactive owner-token candidate
record read-only counts
```

Non-responsibilities:

```text
thread scheduling
atomic claim
remote-free drain
owner mutation
page-source calls
reclaim execution
unreserve / OS release
provider / hook / replacement
```

## Proof surface

```text
apps/hako-alloc-thread-heap-owner-inventory-proof/
tools/checks/k2_wide_hako_alloc_thread_heap_owner_inventory_guard.sh
```

Required inactive facts:

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
