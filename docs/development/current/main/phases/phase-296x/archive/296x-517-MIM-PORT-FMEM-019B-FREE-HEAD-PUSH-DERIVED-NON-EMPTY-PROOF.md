---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-019B.
Related:
  - docs/development/current/main/phases/phase-296x/296x-516-MIM-PORT-FMEM-019-REFILL-COUNTER-FIELD-GROUP-PILOT.md
  - lang/src/hako_alloc/memory/page_meta_local_free_to_free_refill_counter_body_box.hako
  - src/mir/fastmem_access_plan.rs
---

# 296x-517 MIM-PORT-FMEM-019B FreeHeadPush-Derived Non-Empty Proof

## Decision

Let a verified `FreeHeadPush(page, block)` derive a same-region
`free_head non-empty` verifier fact for the same `page`.

The derived proof is order-sensitive:

```text
FreeHeadPush(page, block)
  -> derived free_head non-empty proof for page
  -> later FreeHeadPop(page) may consume it
```

This is verifier-owned proof transfer, not a new source assumption. The source
body must not need to call `mem.assumeFreeHeadNonEmpty(page)` immediately after
it has just pushed a block to `free_head`.

## Implementation Boundary

The proof transfer belongs to `refresh_function_fastmem_access_plans` because
that pass already walks function MemOps in execution order and owns verified
free-list plans.

```text
MIRBuilder:
  still only emits MemOps and source assumptions

FastMem access planner:
  derives the fact after a verified FreeHeadPush

LLVM producer:
  consumes the verified plans only
```

## Body Pilot

Add a `.hako hako_alloc` body that composes:

```text
LocalFreePop(page)
FreeHeadPush(page, block)
FreeHeadPop(page)
page.used = page.used + 1
```

without `mem.assumeFreeHeadNonEmpty(page)`.

## Still Closed

```text
multi-block refill transfer policy
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

```text
FreeHeadPush plans remain verified only with same-owner and block-next proof.
FreeHeadPop plans can consume the derived non-empty proof in the same region.
The refill-then-alloc body lowers through MIR-to-LLVM evidence.
No source-level assumeFreeHeadNonEmpty is required in that body.
No Type ABI / Provider ABI hot lookup appears.
No product activation / hook / global allocator / winner claim appears.
```
