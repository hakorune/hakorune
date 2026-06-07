---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-108.
Related:
  - docs/development/current/main/phases/phase-296x/296x-606-MIM-PORT-FMEM-107-SAME-REMOTE-FREE-PUBLISH-BODY-PREFLIGHT.md
  - lang/src/hako_alloc/memory/page_map_release_box.hako
  - lang/src/hako_alloc/memory/page_map_box.hako
  - lang/src/hako_alloc/memory/page_meta_same_remote_free_publish_body_box.hako
---

# 296x-607 MIM-PORT-FMEM-108 PageMapRelease Pointer Lookup Preflight Selection

## Purpose

Select the next narrow source slice for connecting the landed page-meta
same/remote free publish body to a caller pointer lookup surface.

This row is selection-only. It decides the smallest preflight before wiring
`PageMapReleaseSeam.releasePtr` to the publish body.

## Chosen Mode

```text
BoxShape
```

## Candidate Surface

```text
PageMapReleaseSeam.releasePtr(ptr, block)
PageMapBox.lookup(ptr)
future source-level PageMapBridge / pointer-derived PageKey surface
existing page-meta same/remote free publish body
```

## Required Boundary

```text
do not add code before selecting one lookup preflight shape
do not open raw pointer arithmetic in general .hako
do not add source-level RawPtr<T>
do not open product activation, hooks, global allocator claim, or winner behavior
do not claim full hako mimalloc algorithm completion
```

## Selection Questions

```text
Can the next row stay page-map-local, using an explicit PageKey/PageMeta handle
input, or must it introduce the first pointer-derived lookup surface?

If pointer-derived lookup is selected, which narrow source truth should own it:
  PageMapReleaseSeam.releasePtr
  PageMapBox.lookup
  a new PageMapBridge preflight box

What evidence should prove the lookup without becoming product activation:
  AddressToken/PageKey-like source model
  TableIndex/FieldLoad evidence
  no Type ABI hot lookup
  no Provider ABI hot dispatch
```

## Acceptance Sketch

```text
one next lookup preflight shape is selected
required existing MemOps/proofs are named
missing substrate is explicitly called out before implementation
CURRENT_STATE points at the selected implementation row
```

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
```
