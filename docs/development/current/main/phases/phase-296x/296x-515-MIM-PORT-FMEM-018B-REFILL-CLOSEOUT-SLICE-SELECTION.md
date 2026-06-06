---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-018B.
Related:
  - docs/development/current/main/phases/phase-296x/296x-514-MIM-PORT-FMEM-018-LOCAL-FREE-TO-FREE-REFILL-BODY-PILOT.md
  - lang/src/hako_alloc/memory/page_meta_local_free_to_free_refill_body_box.hako
---

# 296x-515 MIM-PORT-FMEM-018B Refill Closeout Slice Selection

## Decision

Select `MIM-PORT-FMEM-019` as the next implementation row:

```text
refill counter field-group pilot
```

The next body should extend the landed single-block refill transfer:

```text
LocalFreePop(page)
FreeHeadPush(page, block)
page.local_free_collect_count = page.local_free_collect_count + 1
page.local_free_collected_blocks = page.local_free_collected_blocks + 1
```

These two counters are the source-level refill accounting fields used by
`HakoAllocPage.acquire` when it moves one local-free block back into the
ordinary free list.

## Why This Slice

This keeps the row on the `.hako hako_alloc -> MIR MemOp -> LLVM/object` body
migration path while closing the accounting surface for the landed single-block
refill body.

Rejected for this row:

```text
multi-block refill:
  too broad before the single-block refill body has its source accounting
  fields represented in PageMetaLayoutV0.

branching allocation route:
  fastmem builder currently records `if` bodies without real control-flow
  routing; branch semantics need a separate row.

refill-then-alloc:
  useful later, but it needs either a derived free_head non-empty proof from
  FreeHeadPush or an extra source assumption. That proof transfer should not be
  bundled with the counter field-group row.
```

## Still Closed

```text
multi-block refill transfer policy
refill-then-alloc body
derived free_head non-empty proof from FreeHeadPush
fastmem control-flow branch routing
ordinary free_head FieldLoad / FieldStore mutation
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

## Acceptance For MIM-PORT-FMEM-019

```text
PageMetaLayoutV0 exposes local_free_collect_count and
local_free_collected_blocks as mutable plain-scalar fields.
The landed single-block refill body increments both counters by 1.
The new .hako body lowers through MIR-to-LLVM producer evidence.
No Type ABI / Provider ABI hot lookup appears.
No product activation / hook / global allocator / winner claim appears.
```
