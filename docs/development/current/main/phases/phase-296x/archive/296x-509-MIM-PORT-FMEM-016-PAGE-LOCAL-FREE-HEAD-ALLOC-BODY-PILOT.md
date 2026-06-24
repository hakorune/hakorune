---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-016.
Related:
  - docs/development/current/main/phases/phase-296x/296x-508-MIM-PORT-FMEM-015D-FREE-HEAD-POP-ROUTE-CLOSEOUT-SELECTION.md
  - lang/src/hako_alloc/memory/page_meta_free_head_alloc_body_box.hako
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-509 MIM-PORT-FMEM-016 Page-Local FreeHead Alloc Body Pilot

## Decision

Open a narrow `.hako hako_alloc` page-local allocation body pilot that composes
the verified `FreeHeadPop` route with a `PageMeta.used` update.

This row proves the ordinary page free-list route can now be consumed as a
small allocation-body shape:

```text
same-owner proof
free_head non-empty proof
FreeHeadPop
PageMeta.used load/increment/store
ordinary scalar return/use evidence
```

The new source pilot is:

```text
lang/src/hako_alloc/memory/page_meta_free_head_alloc_body_box.hako
```

## Source Shape

```text
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
  local next_used = used + 1
  page.used = next_used
  return popped + next_used
}
```

This is intentionally parallel to the earlier `LocalFreePop` allocation body
pilot, but consumes the ordinary page free-list head.

## Evidence

The fastmem source syntax smoke now checks the new pilot through AST, MIR
metadata, MIR-to-LLVM producer report, and `fastmem-check`.

Key expected evidence:

```text
fastmem_memop_free_head_pop_count=1
fastmem_verified_mem_access_plan_count=5
fastmem_verified_field_access_count=3
fastmem_verified_table_access_count=1
fastmem_free_head_pop_plan_count=1
fastmem_free_head_pop_lowerable_count=1
fastmem_free_head_access_plan_incomplete_count=0

replacement_front_producer=mir_to_llvm_lowering
replacement_front_selected_memop_family=local_free
replacement_front_selected_memop_kinds=FreeHeadPop
memop_table_index_lowered_count=1
memop_field_load_lowered_count=2
memop_field_store_lowered_count=1
memop_free_head_pop_lowered_count=1
memop_free_head_pop_layout_ref_consumed_count=1
fastmem_free_head_plain_store_lowered_count=0
fastmem_free_head_pop_lowering_uses_verified_plan=1
fastmem_free_head_pop_lowering_enabled=1
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
global_allocator_claim=0
winner_claim=0
summary=ok
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

## Acceptance

```bash
cargo build --release --bin hakorune
bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIM-PORT-FMEM-016B:
  close out the page-local free_head allocation body route and choose the next
  slice. Main candidates:
    - local_free -> free refill collection route
    - PageMeta counter field-group migration
    - narrow allocation-body route packaging evidence
```
