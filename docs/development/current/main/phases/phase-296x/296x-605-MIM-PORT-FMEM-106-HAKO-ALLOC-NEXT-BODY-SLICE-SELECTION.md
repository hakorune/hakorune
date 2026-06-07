---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-106.
Related:
  - docs/development/current/main/phases/phase-296x/296x-604-MIM-PORT-FMEM-105-IMPLEMENTATION-REENTRY-SELECTION.md
  - docs/development/current/main/workstreams/mimalloc-current.md
---

# 296x-605 MIM-PORT-FMEM-106 hako_alloc Next Body Slice Selection

## Purpose

Select the next `.hako` hako_alloc fastmem body slice to migrate now that the
post-refresh cleanup series has returned the lane to implementation work.

## Chosen Mode

```text
BoxShape
```

## Candidate Surface

```text
existing hako_alloc memory boxes under lang/src/hako_alloc/memory
existing source-syntax smoke fixtures
existing FastMemory MemOp vocabulary and verifier plans
```

## Required Boundary

```text
do not add code before selecting one body slice
do not add a new MemOp family in this selection card
do not open product activation, hooks, global allocator claim, or winner behavior
do not extend smoke scaffolding unless the selected body slice requires it
```

## Acceptance Sketch

```text
one next hako_alloc body slice is selected
required existing MemOps/proofs are named
missing substrate, if any, is explicitly called out before implementation
CURRENT_STATE points at the selected implementation row
```

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
```

## Selection

```text
selected next slice:
  page_meta same/remote free publish body

next row:
  296x-606 MIM-PORT-FMEM-107 same/remote free publish body preflight
```

## Why This Slice

```text
It is the smallest implementation reentry that uses existing FastMemory
substrate without reopening product behavior:

same owner:
  ownerEq -> assumeSameOwner -> LocalFreePush -> page.used decrement

remote owner:
  ownerEq false -> assumeRemoteOwner -> AtomicRemoteHeadPush

The existing branch-CFG, LocalFreePush, AtomicRemoteHeadPush, and page-local
body rows already prove the individual pieces. The missing piece is composing
the release/free publish body in one `.hako fastmem` source region.
```

## Rejected Candidates

```text
PageMapReleaseSeam.releasePtr:
  too broad for the first reentry row because source-level pointer-derived
  lookup / PageMapBridge must be selected before the page-local free body can
  be wired to caller ptr ownership.

continued source-syntax smoke split:
  useful later, but not a body migration row.

activation-readiness audit:
  behavior stays closed and the refreshed winner closeout already documents
  the terminal evidence.
```

## Required Existing Substrate

```text
TableIndex + FieldLoad/FieldStore
CurrentAllocOwnerId + OwnerEq
FastMemory branch CFG lowering
LocalFreePush verifier/lowering plan
AtomicRemoteHeadPush verifier/lowering plan
same/remote free body report/check family
```

## Still Deferred

```text
PageMapReleaseSeam ptr lookup
source-level PageMapBridge from caller ptr
TLS transfer
product activation
hook installation
global allocator product claim
winner behavior claim
```

## Closeout

```text
next: 296x-606 MIM-PORT-FMEM-107 same/remote free publish body preflight
```
