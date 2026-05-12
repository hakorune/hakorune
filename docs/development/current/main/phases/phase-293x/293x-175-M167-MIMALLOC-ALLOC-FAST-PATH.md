---
Status: Complete
Date: 2026-05-12
Scope: M167 `.hako` mimalloc alloc fast path plus deterministic fallback.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - lang/src/hako_alloc/memory/alloc_fast_path_heap_box.hako
  - lang/src/hako_alloc/memory/page_box.hako
  - lang/src/hako_alloc/memory/page_queue_box.hako
  - apps/mimalloc-alloc-fast-path-proof/
---

# 293x-175 M167 Mimalloc Alloc Fast Path

## Goal

Compose the M165 page-local free-list model with the M166 page queue/direct-page
cache owner.

`HakoAllocFastPathHeap` owns orchestration only:

- select a page through `HakoAllocPageQueue`;
- pop a block through `HakoAllocPageModel.acquire(...)`;
- create a deterministic modeled fallback page when the queue has no available
  page.

## Stop Line

M167 does not add OSVM page sourcing, local-free collection/retire,
remote-free atomics, page-map lookup, provider activation, hook install,
process allocator replacement, `.inc` name matching, or production `usize`
field migration.

## Proof

```bash
bash tools/checks/k2_wide_mimalloc_alloc_fast_path_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
