---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-015B.
Related:
  - docs/development/current/main/phases/phase-296x/296x-505-MIM-PORT-FMEM-015A-FREE-HEAD-POP-VOCABULARY-SOURCE-PREFLIGHT.md
  - lang/src/hako_alloc/memory/page_meta_free_head_pop_precondition_box.hako
  - src/mir/fastmem_access_plan.rs
  - src/mir/function/types.rs
  - src/runner/mir_json_emit/metadata.rs
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-506 MIM-PORT-FMEM-015B FreeHeadPop Verifier Preconditions

## Decision

Add verifier-owned precondition evidence for `FreeHeadPop` without opening
LLVM lowering.

This row introduces:

```text
FastMemFreeHeadNonEmptyFact
mem.assumeFreeHeadNonEmpty(page)
FastMemAccessPlanKind::FreeHeadPop
FastMemAccessPlanPayload::FreeHead
```

The positive `.hako hako_alloc` pilot now proves:

```text
same-owner page access
free_head non-empty
free_head access material
FreeBlockNodeLayoutV0.next access material
```

## Source Shape

```hako
fastmem PageMapV0 {
    mem.assumeTableLength(page_table, 64)
    mem.assumeIndexInRange(page_index, 64)
    local page = page_table[page_index]
    local current = mem.currentAllocOwnerId()
    local same = mem.ownerEq(page.owner_worker_id, current)
    mem.assumeSameOwner(page, same)
    mem.assumeFreeHeadNonEmpty(page)
    local popped = mem.freeHeadPop(page)
    local used = page.used
    page.used = used + 1
}
```

`currentAllocOwnerId` / `ownerEq` are source proof producers here. The MIR
inventory is expected to carry the same-owner fact, not necessarily surviving
owner-runtime MemOps in the body report.

## Evidence

The source syntax smoke now fixes the positive precondition surface:

```text
fastmem_free_head_list_plan=1
fastmem_free_head_pop_plan_count=1
fastmem_free_head_nonlowerable_count=0
fastmem_free_head_pop_lowerable_count=1
fastmem_free_head_access_resolved_count=1
fastmem_free_head_block_next_access_resolved_count=1
fastmem_free_head_access_plan_incomplete_count=0
fastmem_free_head_non_empty_fact_count=1
fastmem_free_head_same_owner_required=1
fastmem_free_head_same_owner_missing_count=0
fastmem_free_head_non_empty_required=1
fastmem_free_head_non_empty_missing_count=0
fastmem_free_head_remote_owner_rejected_count=1
```

The MIR-to-LLVM producer still fails closed:

```text
[llvm/fastmem:unsupported-kind] free_head_pop
```

## Still Closed

```text
FreeHeadPop LLVM lowering
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
MIM-PORT-FMEM-015C:
  lower verified FreeHeadPop through MIR-to-LLVM/object using verifier-owned
  free_head and FreeBlockNodeLayoutV0.next access material.

  Keep refill, remote routing, AtomicRemoteHead, TLS transfer, and product
  activation closed.
```
