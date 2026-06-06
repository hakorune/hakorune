---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-025.
Related:
  - docs/development/current/main/phases/phase-296x/296x-522-MIM-PORT-FMEM-024-PAGE-LOCAL-SAME-OWNER-FREE-BODY.md
  - lang/src/hako_alloc/memory/page_meta_same_owner_free_body_box.hako
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
---

# 296x-523 MIM-PORT-FMEM-025 Page-Local Free Route Report Surface

## Purpose

Add a producer-neutral report surface for verified straight-line free route
candidates.

MIM-021 added allocation route candidate reporting:

```text
local_free_alloc
free_head_alloc
refill_then_free_head_alloc
```

MIM-024 added the first straight-line same-owner free body. That body should
not be classified through the allocation route field. It needs a separate free
route report surface.

## Report Fields

```text
page_local_free_route_report_v0=1
page_local_free_route_candidate=<none|same_owner_local_free|mixed>
page_local_free_route_candidate_count
page_local_free_route_branch_claim=0
page_local_free_route_cfg_lowering_enabled=0
page_local_free_route_verified_plan_source=fastmem_access_plans
```

## Classification Rules

```text
same_owner_local_free:
  LocalFreePush verified/lowerable
  LocalFreePop absent
  FreeHeadPush absent
  FreeHeadPop absent

mixed:
  verified free-list plans exist but do not match the same-owner local-free
  candidate.
```

The classifier must consume verified MemOp plans only. It must not inspect
source file names or Box names.

## Still Closed

```text
fastmem branch CFG lowering
remote owner routing
AtomicRemoteHead
TLS backing transfer
provider activation
process allocator replacement
hook installation
global allocator claim
winner claim
```
