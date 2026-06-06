---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-016B.
Related:
  - docs/development/current/main/phases/phase-296x/296x-509-MIM-PORT-FMEM-016-PAGE-LOCAL-FREE-HEAD-ALLOC-BODY-PILOT.md
  - docs/development/current/main/phases/phase-296x/296x-504-MIM-PORT-FMEM-014B-PAGE-LOCAL-ROUTE-CLOSEOUT-SELECTION.md
  - lang/src/hako_alloc/memory/page_box.hako
---

# 296x-510 MIM-PORT-FMEM-016B Next Refill-Prerequisite Slice Selection

## Decision

Select `FreeHeadPush` vocabulary/source preflight as the next row.

The page-local allocation body now has both single-block pop routes:

```text
LocalFreePop + used update
FreeHeadPop + used update
```

The next mimalloc-shaped route is `local_free -> free` refill, but refill should
not open as one large row. It needs two primitive directions:

```text
local_free_head pop:
  already covered by LocalFreePop

free_head push:
  still missing
```

Therefore the next durable slice is a dedicated `FreeHeadPush` MemOp family.
This mirrors the existing `LocalFreePush` route, but writes to the ordinary page
free-list head. A later row can compose:

```text
block = mem.localFreePop(page)
mem.freeHeadPush(page, block)
```

as the first narrow refill body.

## Selected Next Row

```text
MIM-PORT-FMEM-017A:
  FreeHeadPush vocabulary/source preflight.
```

Expected initial shape:

```text
mem.assumeSameOwner(page, same)
mem.assumeFreeHeadBlockNext(block)
mem.freeHeadPush(page, block)
```

The 017A row should be vocabulary/source/MIR observation only. Verifier-owned
preconditions and LLVM lowering should stay for follow-up rows, mirroring the
`LocalFreePush` and `FreeHeadPop` ladder.

## Why Not Refill Directly

Opening refill directly would mix multiple new responsibilities:

```text
local_free pop
free_head push
empty/non-empty branch shape
single-block vs multi-block transfer policy
collection counters
```

`LocalFreePop` is already proven, but `free_head` publication still has no
dedicated push route. Adding that primitive first keeps the next row to one
acceptance shape.

## Why Not Counter Fields

Counter fields such as `alloc_count`, `peak_used`, `requested_bytes`,
`local_free_collect_count`, and `local_free_collected_blocks` remain important,
but they are not the next route seam.

Counters should land as explicit field-group rows after the free-list mutation
surface is complete enough to describe refill. Otherwise layout growth and
route semantics would be mixed.

## Still Closed

```text
FreeHeadPush verifier preconditions
FreeHeadPush LLVM lowering
local_free -> free refill body
ordinary free_head FieldLoad / FieldStore mutation
alloc_count / peak_used / requested_bytes field-group migration
remote owner routing
AtomicRemoteHead
TLS backing transfer
owner slot reuse
abandoned reclaim behavior
provider activation
process allocator replacement
hook installation
global allocator claim
winner claim
```

## Next

```text
MIM-PORT-FMEM-017A:
  add FreeHeadPush MemOp vocabulary/source/MIR observation and fail-closed
  LLVM producer evidence. Verifier-owned FreeHeadPush plans stay closed for the
  following precondition row.
```
