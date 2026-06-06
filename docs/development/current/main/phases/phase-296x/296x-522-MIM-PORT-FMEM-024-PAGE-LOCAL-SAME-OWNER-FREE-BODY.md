---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-024.
Related:
  - docs/development/current/main/phases/phase-296x/296x-521-MIM-PORT-FMEM-023-FASTMEM-BRANCH-REJECTION-GATE.md
  - lang/src/hako_alloc/memory/page_meta_local_free_push_precondition_box.hako
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-522 MIM-PORT-FMEM-024 Page-Local Same-Owner Free Body

## Purpose

Add the first straight-line `.hako hako_alloc` free body pilot after branch CFG
was explicitly closed.

The active allocation body pilots cover:

```text
local_free_alloc
free_head_alloc
refill_then_free_head_alloc
```

The matching same-owner free route should remain straight-line for now:

```text
page = page_table[page_index]
same_owner = mem.ownerEq(page.owner_worker_id, mem.currentAllocOwnerId())
mem.assumeSameOwner(page, same_owner)
mem.assumeLocalFreeBlockNext(block)
mem.localFreePush(page, block)
page.used = page.used - 1
```

## Acceptance

```text
new hako_alloc body source exists under lang/src/hako_alloc/memory/
MIR inventory verifies TableIndex, FieldLoad, FieldStore, OwnerEq,
CurrentAllocOwnerId, and LocalFreePush plans
MIR-to-LLVM producer report lowers LocalFreePush through verified plan
used decrement is expressed with existing MemOp Sub plus FieldStore
branch claim remains 0
CFG lowering remains 0
remote-owner route remains closed
AtomicRemoteHead remains closed
product activation / hook / global allocator / winner claims remain 0
```

## Landed Evidence

```text
Source:
  lang/src/hako_alloc/memory/page_meta_same_owner_free_body_box.hako

Body:
  TableIndex(page_table, page_index)
  FieldLoad(owner_worker_id)
  CurrentAllocOwnerId
  OwnerEq
  assumeSameOwner
  assumeLocalFreeBlockNext
  LocalFreePush(page, block)
  FieldLoad(used)
  Sub(used, 1)
  FieldStore(used)

Smoke:
  bash tools/hako_check/fastmem_source_syntax_smoke.sh
```

## Still Closed

```text
fastmem branch CFG lowering
remote owner routing
AtomicRemoteHead
TLS backing transfer
process allocator replacement
hook installation
global allocator claim
winner claim
```

## Next Row

```text
MIM-PORT-FMEM-025:
  Page-local free route report surface

Goal:
  classify verified straight-line free route candidates separately from the
  existing page-local allocation route report.

Candidate:
  same_owner_local_free
```
