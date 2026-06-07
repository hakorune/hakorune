---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-078.
Related:
  - docs/development/current/main/phases/phase-296x/296x-575-MIR-BUILDER-FASTMEM-BRANCH-RETURN-SCOPE-FIX.md
  - docs/development/current/main/phases/phase-296x/296x-576-MIM-PORT-FMEM-077-PAGE-LOCAL-ALLOC-ROUTE-CFG-PRODUCER-PILOT.md
  - lang/src/hako_alloc/memory/page_meta_page_local_alloc_route_cfg_preflight_box.hako
---

# 296x-577 MIM-PORT-FMEM-078 hako_alloc Route Body Reentry Selection

## Purpose

Select the next concrete `hako_alloc` body migration slice after the page-local
allocation route CFG producer pilot and the branch-local return scope fix.

This is the reentry point back into implementation rows. It should choose one
small body slice and keep the route/claim ladder explicit.

## Current Inputs

```text
page-local allocation route CFG producer evidence is open
branch-local returns inside FastMemory regions are accepted and pinned by smoke
page-local free route CFG evidence exists from prior rows
activation / hook / global allocator / winner claims remain closed
```

## Candidate Next Slices

```text
1. page-local alloc/free route body join preflight
2. branch-local return route body producer pilot
3. route CFG closeout evidence over both alloc and free paths
4. next missing hako_alloc body identified by fastmem inventory
```

## Required Boundaries

```text
selection row only unless a single next slice is explicitly chosen
no new MemOp kind
no LayoutRef phi/join rule
no TLS backing transfer change
no owner slot reuse / abandoned reclaim change
no product activation / hook / global allocator claim / winner claim change
```

## Acceptance Sketch

```text
next hako_alloc body slice is named
source fixture and report/check owner are identified
closed activation / hook / allocator / winner claims remain closed
current state pointer guard passes
git diff --check passes
```

## Non-goals

```text
opening product allocator execution
retiring diagnostic Python-template C bridge
changing FastMemory producer-neutral report schema
```
