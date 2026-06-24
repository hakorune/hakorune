---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-014.
Related:
  - docs/development/current/main/phases/phase-296x/296x-502-MIM-PORT-FMEM-013B-LOCAL-FREE-POP-LLVM-PRODUCER.md
  - lang/src/hako_alloc/memory/page_meta_local_free_alloc_body_box.hako
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-503 MIM-PORT-FMEM-014 Page-Local Alloc Pop Body Pilot

## Decision

Open the first `.hako hako_alloc` page-local allocation body pilot that composes:

```text
same-owner proof
local_free non-empty proof
LocalFreePop
PageMeta.used load/increment/store
```

This is the first row where `LocalFreePop` is no longer only a free-list
primitive pilot. It is consumed by a narrow allocation-body shape:

```text
block = mem.localFreePop(page)
used = page.used
page.used = used + 1
return block + next_used
```

`alloc_count` and other hako_alloc counters stay closed in this row because
`PageMetaLayoutV0` does not yet contain those fields. Adding fields and opening
an allocation-body route are separate rows.

## Still Closed

```text
alloc_count / peak_used / lifecycle counter fields
free_head refill path
local_free -> free migration path
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

## Evidence

The new source pilot is:

```text
lang/src/hako_alloc/memory/page_meta_local_free_alloc_body_box.hako
```

Expected report/check shape:

```text
fastmem_memop_local_free_pop_count=1
fastmem_local_free_pop_lowerable_count=1
fastmem_verified_mem_access_plan_count=5
fastmem_verified_field_access_count=3
fastmem_verified_table_access_count=1
fastmem_field_load_plan_count=2
fastmem_field_store_plan_count=1
memop_local_free_pop_lowered_count=1
memop_field_load_lowered_count=2
memop_field_store_lowered_count=1
fastmem_local_free_head_plain_store_lowered_count=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
global_allocator_claim=0
winner_claim=0
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
MIM-PORT-FMEM-014B:
  choose the next page-local route closeout slice. Candidates are either
  free_head refill observation or an explicit PageMeta field-group row for
  additional allocation counters. Remote-free, AtomicRemoteHead, TLS transfer,
  and product activation remain separate rows.
```
