---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-110.
Related:
  - docs/development/current/main/phases/phase-296x/296x-607-MIM-PORT-FMEM-108-PAGEMAPRELEASE-POINTER-LOOKUP-PREFLIGHT-SELECTION.md
  - lang/src/hako_alloc/memory/page_map_release_box.hako
  - lang/src/hako_alloc/memory/page_map_bridge_box.hako
  - lang/src/hako_alloc/memory/page_map_box.hako
  - tools/checks/impl/k2_wide_mimalloc_page_map_release_guard.sh
---

# 296x-609 MIM-PORT-FMEM-110 PageMapRelease Pointer Lookup Bridge

## Purpose

Lift the release-side pointer ownership lookup into an explicit source bridge
so `HakoAllocPageMapReleaseSeam` composes a named PageMap bridge rather than
embedding the `HakoAllocPageMap.lookup(...)` / `unregister(...)` calls
directly.

This stays page-map-local. It does not open raw pointer arithmetic, PageKey /
AddressToken derivation, product allocator activation, hooks, or global
allocator claims.

## Chosen Mode

```text
BoxShape
```

## Required Boundary

```text
do not add a new MemOp family
do not add general unsafe pointer arithmetic
do not open PageKey / AddressToken derivation yet
do not open product activation, hooks, global allocator claim, or winner behavior
do not widen the lookup surface beyond the explicit page-map bridge route
```

## Implementation Shape

```text
Add a thin HakoAllocPageMapBridge wrapper:
  lookup(ptr) -> HakoAllocPageMap.lookup(ptr)
  unregister(ptr) -> HakoAllocPageMap.unregister(ptr)
  liveCount() -> page_map.live_count

Update HakoAllocPageMapReleaseSeam:
  own a HakoAllocPageMapBridge
  resolve release ownership through bridge.lookup(ptr)
  unregister through bridge.unregister(ptr)
  keep page-local release delegated to HakoAllocPageModel.releaseLocal(...)

Keep HakoAllocPageMap itself as the storage owner.
Keep observer/realloc rows page-map-local.
```

## Acceptance Sketch

```text
release seam route resolves through HakoAllocPageMapBridge.lookup/unregister
page-local release remains delegated to HakoAllocPageModel.releaseLocal(...)
page_map_release smoke and route guards remain green
no Type ABI hot lookup or Provider ABI hot dispatch is introduced
CURRENT_STATE points at the next implementation topic after closeout
```

## Non-Goals

```text
PageKey / AddressToken derivation
general fastpath rename
product activation / hook / global allocator / winner claims
changing page-map ownership registration rules
changing page-local release semantics
```

## Verification

```bash
bash tools/checks/impl/k2_wide_mimalloc_page_map_release_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
