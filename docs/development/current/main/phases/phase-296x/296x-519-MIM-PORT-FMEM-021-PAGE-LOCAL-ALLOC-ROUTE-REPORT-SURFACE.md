---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-021.
Related:
  - docs/development/current/main/phases/phase-296x/296x-518-MIM-PORT-FMEM-020-REFILL-BRANCH-ROUTE-SELECTION.md
  - lang/src/hako_alloc/memory/page_meta_local_free_alloc_body_box.hako
  - lang/src/hako_alloc/memory/page_meta_free_head_alloc_body_box.hako
  - lang/src/hako_alloc/memory/page_meta_refill_then_free_head_alloc_body_box.hako
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
---

# 296x-519 MIM-PORT-FMEM-021 Page-Local Alloc Route Report Surface

## Purpose

Add a producer-neutral report surface that classifies verified page-local
allocation body candidates without opening fastmem branch execution.

The active `.hako hako_alloc` body pilots are straight-line candidates:

```text
local_free_alloc:
  LocalFreePop(page)
  page.used = page.used + 1

free_head_alloc:
  FreeHeadPop(page)
  page.used = page.used + 1

refill_then_free_head_alloc:
  LocalFreePop(page)
  FreeHeadPush(page, block)
  FreeHeadPop(page)
  page.used = page.used + 1
```

MIM-021 classifies these candidates from verified MemOp plans. It does not
choose a runtime branch.

## Report Fields

Add report/check evidence in the existing fastmem MIR-to-LLVM producer surface:

```text
page_local_alloc_route_report_v0=1
page_local_alloc_route_candidate=<none|local_free_alloc|free_head_alloc|refill_then_free_head_alloc|mixed>
page_local_alloc_route_candidate_count
page_local_alloc_route_branch_claim=0
page_local_alloc_route_cfg_lowering_enabled=0
page_local_alloc_route_verified_plan_source=fastmem_access_plans

fastmem_free_head_non_empty_source_assume_count
fastmem_free_head_non_empty_derived_from_free_head_push_count
```

The classifier must only consume verified/lowerable plans. It must not inspect
source file names or Box names.

## Classification Rules

```text
local_free_alloc:
  LocalFreePop verified/lowerable
  FreeHeadPop absent
  FreeHeadPush absent

free_head_alloc:
  FreeHeadPop verified/lowerable
  LocalFreePop absent
  FreeHeadPush absent

refill_then_free_head_alloc:
  LocalFreePop verified/lowerable
  FreeHeadPush verified/lowerable
  FreeHeadPop verified/lowerable

mixed:
  verified free-list plans exist but do not match one of the above single-route
  candidates.
```

Field loads/stores for `page.used` may be present, but route classification is
owned by the free-list MemOp plan set.

For refill routes, the report must also prove that the `FreeHeadPop` non-empty
proof came from the preceding verified `FreeHeadPush`, not from a source-level
`mem.assumeFreeHeadNonEmpty(page)`.

## Still Closed

```text
fastmem branch CFG lowering
route exclusivity / dominance proof
multi-block refill transfer
ordinary free_head / local_free_head FieldLoad or FieldStore mutation
remote owner routing
AtomicRemoteHead
TLS backing transfer
provider activation
process allocator replacement
hook installation
global allocator claim
winner claim
```

## Acceptance

```text
The three existing body pilots emit distinct route candidate names.
The refill-then-free_head alloc body reports derived non-empty proof evidence.
Route classification is based on verified MemOp plans only.
Branch claim and CFG lowering fields stay 0.
Existing fastmem source syntax smoke covers the new fields.
No Type ABI / Provider ABI hot lookup appears.
No product activation / hook / global allocator / winner claim appears.
```
