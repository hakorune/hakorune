---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-018.
Related:
  - docs/development/current/main/phases/phase-296x/296x-513-MIM-PORT-FMEM-017C-FREE-HEAD-PUSH-LLVM-PRODUCER-PILOT.md
  - lang/src/hako_alloc/memory/page_meta_local_free_to_free_refill_body_box.hako
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-514 MIM-PORT-FMEM-018 LocalFree To Free Refill Body Pilot

## Decision

Compose the first narrow `local_free_head -> free_head` refill body in `.hako`
`hako_alloc` using already verified free-list MemOps.

The accepted shape is a single-block transfer:

```text
mem.assumeSameOwner(page, same)
mem.assumeLocalFreeNonEmpty(page)
block = mem.localFreePop(page)
mem.assumeFreeHeadBlockNext(block)
mem.freeHeadPush(page, block)
```

This row does not open multi-block transfer policy, refill counters,
remote-owner routing, AtomicRemoteHead, TLS transfer, or product allocator
activation.

## Evidence

The new source pilot is:

```text
lang/src/hako_alloc/memory/page_meta_local_free_to_free_refill_body_box.hako
```

The fastmem source syntax smoke now checks:

```text
replacement_front_selected_memop_kinds=LocalFreePop,FreeHeadPush
memop_local_free_pop_lowered_count=1
memop_free_head_push_lowered_count=1
memop_local_free_pop_layout_ref_consumed_count=1
memop_free_head_push_layout_ref_consumed_count=1
fastmem_local_free_access_plan_incomplete_count=0
fastmem_free_head_access_plan_incomplete_count=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
summary=ok
```

## Still Closed

```text
multi-block refill transfer policy
refill / collection counters
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

## Acceptance

```bash
bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIM-PORT-FMEM-018B:
  choose the next refill closeout slice.

  Candidate directions:
    - add refill counters as an explicit field-group row
    - select the next allocation body branch shape
    - defer multi-block refill until counter/lifecycle evidence is ready
```
