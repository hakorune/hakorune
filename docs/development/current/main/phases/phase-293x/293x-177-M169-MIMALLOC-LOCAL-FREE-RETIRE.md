---
Status: Complete
Date: 2026-05-12
Scope: M169 `.hako` mimalloc local-free collection and empty-page retire state.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - lang/src/hako_alloc/memory/page_box.hako
  - apps/mimalloc-local-free-retire-proof/
---

# 293x-177 M169 Mimalloc Local-Free Retire

## Goal

Add the page-local part of mimalloc-style local-free collection.

`HakoAllocPageModel` now owns:

- bounded one-block `local_free` collection in `acquire(...)` when the normal
  free stack is empty;
- empty-page retire state recorded by the final `releaseLocal(...)`;
- counters for local-free collection, collected blocks, and retire events.

## Stop Line

M169 stays page-local. It does not add remote-free atomics, abandoned-page
reclaim, page-map lookup, arbitrary pointer free, OSVM unreserve/release,
provider activation, hook install, process allocator replacement, `.inc` name
matching, or production `usize` field migration.

Heap/queue orchestration remains unchanged in this row. `M170` owns remote-free
integration and any later queue/heap policy that consumes the new page-local
retire state. Full drain/abandoned-page collection remains future work; M169
only proves the first same-thread local-free reuse step.

## Proof

```bash
bash tools/checks/k2_wide_mimalloc_local_free_retire_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
