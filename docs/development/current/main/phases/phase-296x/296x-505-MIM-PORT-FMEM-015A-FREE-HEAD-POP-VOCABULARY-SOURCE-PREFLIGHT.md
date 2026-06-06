---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-015A.
Related:
  - docs/development/current/main/phases/phase-296x/296x-504-MIM-PORT-FMEM-014B-PAGE-LOCAL-ROUTE-CLOSEOUT-SELECTION.md
  - lang/src/hako_alloc/memory/page_meta_free_head_pop_vocabulary_box.hako
  - src/mir/instruction.rs
  - src/mir/contracts/fastmem_ops.rs
  - src/mir/builder/fastmem.rs
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-505 MIM-PORT-FMEM-015A FreeHeadPop Vocabulary Source Preflight

## Decision

Open `FreeHeadPop` as FastMemory vocabulary and source/MIR observation only.

This row lets `.hako hako_alloc` name the ordinary `free_head` pop route:

```hako
fastmem PageMapV0 {
    mem.assumeTableLength(page_table, 64)
    mem.assumeIndexInRange(page_index, 64)
    local page = page_table[page_index]
    local popped = mem.freeHeadPop(page)
    page.used = popped
}
```

The new MemOp is visible in AST inventory and MIR metadata as:

```text
fastmem_memop_free_head_pop_count=1
```

It is not yet a verified free-list access plan and it is not lowerable by the
MIR-to-LLVM producer.

## Why This Is Not A Free-List Implementation Yet

`free_head` mutation is a page-local free-list operation, not an ordinary
`FieldLoad` / `FieldStore` route. The lowering needs verifier-owned proof of:

```text
same-owner page access
free_head non-empty
FreeBlockNodeLayoutV0.next access material
free_head access material
```

Those proof rows are intentionally deferred to the next slice. This keeps 015A
as a vocabulary/source boundary and avoids hiding free-list semantics inside
generic layout lowering.

## Evidence

`tools/hako_check/fastmem_source_syntax_smoke.sh` now covers the vocabulary
pilot and checks:

```text
AST:
  fastmem_memop_table_index_count=1
  fastmem_memop_field_store_count=1
  fastmem_memop_free_head_pop_count=1

MIR:
  fastmem_memop_table_index_count=1
  fastmem_memop_field_store_count=1
  fastmem_memop_free_head_pop_count=1
  fastmem_verified_mem_access_plan_count=2
  fastmem_verified_field_access_count=1
  fastmem_verified_table_access_count=1

LLVM producer:
  [llvm/fastmem:unsupported-kind] free_head_pop
```

The two verified access plans are only the existing `TableIndex` and `used`
`FieldStore` rows. They are not `FreeHeadPop` plans.

## Still Closed

```text
FreeHeadPop verified access plan
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
MIM-PORT-FMEM-015B:
  add verifier-owned FreeHeadPop plan preconditions:
    same-owner proof
    free_head non-empty proof
    free_head access material
    FreeBlockNodeLayoutV0.next access material

  keep LLVM lowering closed until the plan is complete.
```
