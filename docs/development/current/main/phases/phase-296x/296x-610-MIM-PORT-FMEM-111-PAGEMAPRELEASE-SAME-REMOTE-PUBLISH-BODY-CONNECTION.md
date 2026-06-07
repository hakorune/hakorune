---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-111.
Related:
  - docs/development/current/main/phases/phase-296x/296x-609-MIM-PORT-FMEM-110-PAGEMAPRELEASE-POINTER-LOOKUP-BRIDGE.md
  - lang/src/hako_alloc/memory/page_map_release_box.hako
  - lang/src/hako_alloc/memory/page_meta_same_remote_free_publish_body_box.hako
  - lang/src/hako_alloc/memory/OWNER_CONTRACTS_ARENA_RECLAIM.md
  - tools/checks/impl/k2_wide_mimalloc_page_map_release_guard.sh
---

# 296x-610 MIM-PORT-FMEM-111 PageMapRelease Same/Remote Publish Body Connection

## Purpose

Connect the landed pointer-lookup bridge in `HakoAllocPageMapReleaseSeam` to
the page-meta-local same/remote free publish body, so the release seam now
routes through the shared owner-publish shape before unregistering the caller
pointer.

This stays page-map-local. It does not open raw pointer arithmetic,
PageKey/AddressToken derivation, product allocator activation, hooks, or global
allocator claims.

## Chosen Mode

```text
BoxShape
```

## Required Boundary

```text
keep the explicit page-map bridge lookup
compose the landed same/remote free publish body
keep pointer-derived lookup closed beyond the existing page-map bridge route
do not add RawPtr<T>, PageKey derivation, or general unsafe pointer arithmetic
do not open product activation, hooks, global allocator claim, or winner behavior
do not claim full hako mimalloc algorithm completion
```

## Implementation Shape

```text
Update HakoAllocPageMapReleaseSeam:
  keep HakoAllocPageMapBridge lookup/unregister
  reject dead blocks before publish
  call HakoAllocPageMetaSameRemoteFreePublishBody.sameRemoteFreePublishBodyProbe(...)
  keep page-local release observer and realloc callers on the same seam

Keep HakoAllocPageMap as the storage owner.
Keep the landed same/remote body as the shared page-meta-local publish route.
```

## Acceptance Sketch

```text
release seam composes lookup -> same/remote publish body -> unregister
release guard stays green with the published page-local route
page-local release observer remains stable
no Type ABI hot lookup or Provider ABI hot dispatch is introduced
CURRENT_STATE points at the next implementation topic after closeout
```

## Non-Goals

```text
PageKey / AddressToken derivation
general fastpath rename
product activation / hook / global allocator / winner claims
changing page-map ownership registration rules
changing PageMapBridge lookup ownership
changing the source-syntax manifest runner coverage set
```

## Verification

```bash
bash tools/checks/impl/k2_wide_mimalloc_page_map_release_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
