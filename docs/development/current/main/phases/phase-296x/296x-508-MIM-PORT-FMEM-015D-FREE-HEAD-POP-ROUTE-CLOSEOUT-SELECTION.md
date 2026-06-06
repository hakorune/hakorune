---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-015D.
Related:
  - docs/development/current/main/phases/phase-296x/296x-507-MIM-PORT-FMEM-015C-FREE-HEAD-POP-LLVM-PRODUCER-PILOT.md
  - docs/development/current/main/phases/phase-296x/296x-503-MIM-PORT-FMEM-014-PAGE-LOCAL-ALLOC-POP-BODY-PILOT.md
  - lang/src/hako_alloc/memory/page_meta_free_head_pop_precondition_box.hako
  - lang/src/hako_alloc/memory/page_meta_local_free_alloc_body_box.hako
---

# 296x-508 MIM-PORT-FMEM-015D FreeHeadPop Route Closeout Selection

## Decision

Close the dedicated `FreeHeadPop` route and select a narrow page-local
allocation body pilot as the next row.

The `free_head` sequence now has the same substrate shape as the earlier
`local_free_head` route:

```text
source vocabulary
verifier-owned same-owner / non-empty proof
verifier-owned list-head and block-next access material
MIR-to-LLVM/object producer lowering
producer-neutral report/check evidence
```

The next useful slice is not another free-list primitive. It should consume the
verified `FreeHeadPop` route inside a small allocation-body composition:

```text
same-owner proof
free_head non-empty proof
FreeHeadPop
PageMeta.used load/increment/store
ordinary scalar return/use evidence
```

This mirrors the `MIM-PORT-FMEM-014` local-free body pilot, but uses the
ordinary page free list. That route is closer to the page allocation hot path
described by `page_box.hako` than a standalone primitive pilot.

## Selected Next Row

```text
MIM-PORT-FMEM-016:
  Page-local free_head alloc body pilot.
```

Expected source shape:

```text
fastmem PageMapV0 {
  mem.assumeTableLength(page_table, 64)
  mem.assumeIndexInRange(page_index, 64)
  local page = page_table[page_index]
  local current = mem.currentAllocOwnerId()
  local same = mem.ownerEq(page.owner_worker_id, current)
  mem.assumeSameOwner(page, same)
  mem.assumeFreeHeadNonEmpty(page)
  local block = mem.freeHeadPop(page)
  local used = page.used
  local next_used = used + 1
  page.used = next_used
  return block + next_used
}
```

## Why Not Refill First

`local_free -> free` refill collection is the next major route, but it is wider
than a body-composition pilot. Refill needs at least:

```text
local_free empty/non-empty transition evidence
free_head publication target
multiple block movement or collection accounting
empty-list branch shape
same-owner boundary
```

Opening refill before the ordinary `free_head` body is composed would mix route
selection with multi-block transfer semantics. Keep refill for a later row
after the single-block `free_head` allocation path is proven.

## Why Not Counter Fields First

`alloc_count`, `peak_used`, `requested_bytes`, and related page counters are
still not part of `PageMetaLayoutV0`. They should land as an explicit field
group after the allocation body shape is stable.

Counter fields are important for parity and diagnostics, but they do not decide
the free-list allocation route. Opening them now would grow the layout contract
without proving a new mimalloc-shaped execution path.

## Still Closed

```text
ordinary free_head FieldLoad / FieldStore mutation
local_free -> free refill collection route
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
MIM-PORT-FMEM-016:
  add a .hako hako_alloc source pilot that composes FreeHeadPop with
  PageMeta.used load/increment/store, then require MIR-to-LLVM producer evidence
  for the combined body while keeping refill, counters, remote routing,
  AtomicRemoteHead, TLS transfer, and product activation closed.
```
