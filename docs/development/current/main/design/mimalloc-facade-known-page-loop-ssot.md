---
Status: SSOT
Decision: accepted
Date: 2026-05-16
Scope: MIMAP-038A object-lifecycle facade known-page lookup cleanup.
Related:
  - docs/development/current/main/phases/phase-293x/293x-469-MIMAP-038A-FACADE-KNOWN-PAGE-LOOP.md
  - lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
---

# Mimalloc Facade Known-Page Loop SSOT

## Decision

`HakoAllocObjectLifecycleFacade.objectLifecycleKnownPageIndexById(page_id)`
must scan the facade's known-page queue by its current length.

The old three-slot shape was a cleanup blocker:

```text
pages.get(0)
pages.get(1)
pages.get(2)
```

That shape is no longer allowed in the facade lookup owner.

## Contract

```text
input:
  page_id

source of truth:
  me.object_lifecycle_queue.pages

algorithm:
  count = pages.length()
  i = 0
  loop while i < count
    page = pages.get(i)
    if page != null and page.page_id == page_id
      return i
    i = i + 1
  return -1
```

## Scope

This row only changes the known-page lookup shape. It does not rewrite
`HakoAllocObjectLifecyclePageQueue.selectPage`, which still has separate
selection policy debt and should be handled by a future row if needed.

## Proof

```text
apps/mimalloc-facade-known-page-loop-proof/main.hako
```

The proof adds four pages, finds the fourth page by id, and releases a block
through the facade using that fourth-page lookup.

## Guard

```text
tools/checks/k2_wide_mimalloc_facade_known_page_loop_guard.sh
```

The guard rejects the old `page0/page1/page2` facade-local lookup shape, checks
the loop vocabulary, and runs the four-page proof app through the pure-first
EXE lane.
