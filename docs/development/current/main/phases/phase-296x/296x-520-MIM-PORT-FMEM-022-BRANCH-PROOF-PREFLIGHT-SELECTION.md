---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-022.
Related:
  - docs/development/current/main/phases/phase-296x/296x-519-MIM-PORT-FMEM-021-PAGE-LOCAL-ALLOC-ROUTE-REPORT-SURFACE.md
  - src/mir/builder/fastmem.rs
---

# 296x-520 MIM-PORT-FMEM-022 Branch Proof Preflight Selection

## Purpose

Select the next proof surface needed before fastmem can lower real allocator
branch routing.

MIM-021 proves route candidates from verified straight-line bodies:

```text
local_free_alloc
free_head_alloc
refill_then_free_head_alloc
```

The next missing shape is not another free-list primitive. It is the branch
proof envelope that prevents facts from a non-taken branch from becoming
available to later MemOps.

## Problem

Current fastmem `if` lowering is observation-only:

```text
condition
then body
else body
```

It does not create mutually-exclusive CFG structure. Therefore order-sensitive
facts such as `DerivedFromFreeHeadPush` must not be treated as path-sensitive
until a dedicated proof envelope exists.

## Candidate Next Slices

```text
A. Branch proof vocabulary only:
   introduce report/check names for route-exclusive proof envelopes, no source
   lowering behavior change.

B. Fastmem branch rejection gate:
   make branch-shaped fastmem bodies fail-fast for allocation-route profiles
   until CFG proof support exists.

C. Minimal CFG branch pilot:
   teach fastmem lowering one branch shape and add path-sensitive proof
   dominance.
```

## Selection Guard

Prefer A or B unless a concrete branch fixture can prove:

```text
facts from an untaken branch are not visible to later MemOps
LayoutRef lifetime does not cross branch joins unsafely
route candidate evidence remains producer-neutral
no backend redecision or ABI lookup appears
```

## Still Closed

```text
generic fastmem CFG semantics
multi-block refill transfer
remote owner routing
AtomicRemoteHead
TLS backing transfer
provider activation
process allocator replacement
hook installation
global allocator claim
winner claim
```
