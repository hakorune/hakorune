---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-014B.
Related:
  - docs/development/current/main/phases/phase-296x/296x-503-MIM-PORT-FMEM-014-PAGE-LOCAL-ALLOC-POP-BODY-PILOT.md
  - docs/development/current/main/phases/phase-296x/296x-496-MIM-PORT-FMEM-009-FREE-LIST-MUTATION-SUBSTRATE-SELECTION.md
  - lang/src/hako_alloc/memory/page_box.hako
  - src/mir/fastmem_layout_contract.rs
---

# 296x-504 MIM-PORT-FMEM-014B Page-Local Route Closeout Selection

## Decision

Select a dedicated `free_head` free-list MemOp family as the next page-local
allocation route slice.

The current `LocalFreePop` pilot proves that `.hako hako_alloc` can compose a
same-owner / non-empty free-list primitive with a narrow `used` update. That is
useful as a body-composition pilot, but it is not the full page allocation hot
path in `page_box.hako`.

`HakoAllocPageModel.acquire()` / `acquire_usize()` allocate from the ordinary
free list first:

```text
free_top = free_top - 1
block_id = free.get(free_top)
block_used.set(block_id, 1)
used = used + 1
alloc_count = alloc_count + 1
free_top = free_top
```

Only when the ordinary free list is empty does the model collect from
`local_free` into `free`. Therefore the next route should open the ordinary
`free_head` pop path before adding extra allocation counters or remote/free
publication behavior.

## Selected Next Row

```text
MIM-PORT-FMEM-015:
  FreeHeadPop vocabulary/source preflight.
```

Initial expected shape:

```text
mem.assumeFreeHeadNonEmpty(page)
local block = mem.freeHeadPop(page)
local used = page.used
page.used = used + 1
```

The row should mirror the LocalFree sequence:

```text
015A:
  FreeHeadPop source vocabulary and non-lowerable plan rows.

015B:
  FreeHeadPop preconditions:
    same-owner proof
    free_head non-empty proof
    FreeBlockNodeLayoutV0.next access material

015C:
  FreeHeadPop LLVM producer pilot.
```

## Why Not Counter Fields First

`PageMetaLayoutV0` currently contains:

```text
owner_worker_id
block_size
free_head
local_free_head
remote_head
capacity
used
```

`alloc_count`, `peak_used`, `requested_bytes`, `reject_count`, and lifecycle
counters are present in the `.hako` page model but not in the fastmem PageMeta
layout contract yet. Adding those fields should be an explicit field-group row
after the main page-local pop route is selected.

Counter fields are useful for parity and diagnostics, but they do not decide
the allocation route. Opening them before `free_head` pop would make the layout
larger without moving the allocator hot-path shape closer to mimalloc.

## Why Not Ordinary free_head FieldStore

`free_head` may be observed as a pointer-like field, but mutation should not be
opened as ordinary `FieldLoad` / `FieldStore`. Free-list mutation needs
block-next provenance, non-empty/empty proof, and same-owner proof. Those facts
belong in a dedicated free-list plan, not in generic field lowering.

`local_free_head` already follows this rule through `LocalFreePush` /
`LocalFreePop`; `free_head` should follow the same pattern.

## Still Closed

```text
ordinary free_head FieldStore lowering
alloc_count / peak_used / requested_bytes field-group migration
local_free -> free refill collection route
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
MIM-PORT-FMEM-015A:
  add FreeHeadPop MemOp vocabulary/source observation and fail-closed
  verifier plan rows, with LLVM lowering still closed.
```
