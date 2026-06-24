---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-015C.
Related:
  - docs/development/current/main/phases/phase-296x/296x-506-MIM-PORT-FMEM-015B-FREE-HEAD-POP-VERIFIER-PRECONDITIONS.md
  - lang/src/hako_alloc/memory/page_meta_free_head_pop_precondition_box.hako
  - src/llvm_py/instructions/memop.py
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-507 MIM-PORT-FMEM-015C FreeHeadPop LLVM Producer Pilot

## Decision

Lower verified `FreeHeadPop` plans through the MIR-to-LLVM/object producer.

`FreeHeadPop` consumes the verifier-owned `FreeHead` plan added in 015B. The
lowerer reads only the plan payload:

```text
free_head byte offset / size / type / alignment
FreeBlockNodeLayoutV0.next byte offset / size / type / alignment
same-owner proof
free_head non-empty proof
remote-owner rejected proof
```

It does not infer `free_head` layout from strings in the producer, and it does
not route through ordinary `FieldLoad` / `FieldStore` mutation.

## Producer Shape

The lowering mirrors the existing local-free pop route:

```text
page LayoutRef
  -> load page.free_head
  -> treat old head as FreeBlockNodeLayoutV0
  -> load old_head.next
  -> store next back to page.free_head
  -> return old head as ordinary i64/pointer-sized value
```

The ordinary value path remains separate from backend-private LayoutRefs.

## Evidence

The source syntax smoke now checks:

```text
replacement_front_selected_memop_kinds=FreeHeadPop
fastmem_free_head_pop_plan_count=1
memop_free_head_pop_lowered_count=1
memop_free_head_pop_layout_ref_consumed_count=1
fastmem_free_head_access_plan_incomplete_count=0
fastmem_free_head_plain_store_lowered_count=0
fastmem_free_head_pop_lowering_uses_verified_plan=1
fastmem_free_head_pop_lowering_enabled=1
```

The vocabulary-only pilot still fails closed, but now with a stronger reason:

```text
[llvm/fastmem:missing-verified-free-head-pop-plan]
```

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
MIM-PORT-FMEM-015D:
  close out the FreeHeadPop route and choose the next page allocation slice.

Candidate next slices:
  - compose FreeHeadPop into a narrow page-local alloc body pilot
  - open local_free -> free refill collection route
  - open allocation counter field-group migration

Keep product activation and remote routing closed unless a later row explicitly
opens them.
```
