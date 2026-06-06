---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-019.
Related:
  - docs/development/current/main/phases/phase-296x/296x-515-MIM-PORT-FMEM-018B-REFILL-CLOSEOUT-SLICE-SELECTION.md
  - lang/src/hako_alloc/memory/page_meta_local_free_to_free_refill_counter_body_box.hako
  - src/mir/fastmem_layout_contract.rs
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-516 MIM-PORT-FMEM-019 Refill Counter Field-Group Pilot

## Decision

Open only the refill counter field group needed by the landed single-block
`local_free_head -> free_head` refill body.

The two newly visible `PageMetaLayoutV0` fields are:

```text
local_free_collect_count
local_free_collected_blocks
```

Both are mutable plain-scalar fields. They are intentionally ordinary verified
`FieldLoad` / `Add` / `FieldStore` accesses, not free-list mutation MemOps.

## Evidence

The new source pilot is:

```text
lang/src/hako_alloc/memory/page_meta_local_free_to_free_refill_counter_body_box.hako
```

It composes:

```text
LocalFreePop(page)
FreeHeadPush(page, block)
local_free_collect_count += 1
local_free_collected_blocks += 1
```

The fastmem source syntax smoke now checks:

```text
fastmem_memop_field_load_count=3
fastmem_memop_field_store_count=2
fastmem_verified_mem_access_plan_count=8
fastmem_verified_field_access_count=5
memop_field_load_lowered_count=3
memop_field_store_lowered_count=2
memop_local_free_pop_lowered_count=1
memop_free_head_push_lowered_count=1
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
summary=ok
```

## Still Closed

```text
multi-block refill transfer policy
refill-then-alloc body
derived free_head non-empty proof from FreeHeadPush
fastmem control-flow branch routing
ordinary free_head / local_free_head FieldLoad or FieldStore mutation
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
cargo test -q --lib fastmem
cargo build --release --bin hakorune
bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIM-PORT-FMEM-019B:
  select the next post-counter refill slice.

  Candidate direction:
    - FreeHeadPush-derived free_head non-empty proof
    - refill-then-alloc body pilot

  Keep multi-block refill and fastmem control-flow branch routing closed until
  their own cards.
```
