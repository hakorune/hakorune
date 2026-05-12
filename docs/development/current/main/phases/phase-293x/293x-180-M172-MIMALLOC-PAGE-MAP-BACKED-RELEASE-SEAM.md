---
Status: Complete
Date: 2026-05-12
Scope: M172 `.hako` mimalloc page-map-backed release seam.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - lang/src/hako_alloc/memory/page_map_box.hako
  - lang/src/hako_alloc/memory/page_box.hako
---

# 293x-180 M172 Mimalloc Page-Map-Backed Release Seam

## Goal

Compose the M171 page-map ownership model with page-local release.

M172 owns only this sequence:

```text
HakoAllocPageMap.lookup(ptr)
  -> resolve page_id / block_id
  -> HakoAllocPageModel.releaseLocal(block_id)
  -> HakoAllocPageMap.unregister(ptr)
```

The implementation should live in a small orchestration box rather than growing
`page_map_box.hako`, `page_box.hako`, or `page_heap_box.hako`.

## Stop Line

M172 does not implement realloc, aligned allocation, huge allocation,
secure-list hardening, remote-free atomics, byte copy, OSVM unreserve/release,
provider activation, hook install, process allocator replacement, `.inc` name
matching, or production `usize` field migration.

Unknown pointers, stale page ids, double release, and page-local release reject
must return a structured scalar reject result and keep the failure observable.

## Proof

```bash
bash tools/checks/k2_wide_mimalloc_page_map_release_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The M172 guard is intentionally VM execution plus MIR route contract evidence.
The route contract confirms `HakoAllocPageMap.lookup(...)`,
`HakoAllocPageModel.releaseLocal(...)`, and `HakoAllocPageMap.unregister(...)`
are all same-module lowering-plan calls. Full pure-first EXE parity is a later
object-return/lowering row and must not be treated as part of M172.
