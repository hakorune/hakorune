---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-076.
Related:
  - docs/development/current/main/phases/phase-296x/296x-573-MIM-PORT-FMEM-075-HAKO-ALLOC-BODY-MIGRATION-REENTRY-SELECTION.md
  - docs/development/current/main/phases/phase-296x/296x-519-MIM-PORT-FMEM-021-PAGE-LOCAL-ALLOC-ROUTE-REPORT-SURFACE.md
  - docs/development/current/main/phases/phase-296x/296x-555-MIM-PORT-FMEM-057-PAGE-LOCAL-FREE-ROUTE-CFG-PRODUCER-PILOT.md
  - lang/src/hako_alloc/memory/page_meta_local_free_alloc_body_box.hako
  - lang/src/hako_alloc/memory/page_meta_free_head_alloc_body_box.hako
  - lang/src/hako_alloc/memory/page_meta_refill_then_free_head_alloc_body_box.hako
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_check.py
---

# 296x-574 MIM-PORT-FMEM-076 Page-Local Alloc Route CFG Preflight

## Purpose

Define the report/check boundary for a source-truth page-local allocation route
CFG before opening allocation branch execution.

The landed straight-line allocation bodies are:

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

This row should select the branch route envelope that will later choose between
those bodies. It must not lower the allocation route branch yet.

## Required Boundaries

```text
preflight/report-only row
no new MemOp kind
no new lowering
no path-sensitive allocation branch execution
no LayoutRef join / phi rule
no multi-block refill
no remote-owner free expansion
no TLS backing transfer change
no owner slot reuse / abandoned reclaim change
no product activation / hook / global allocator claim / winner claim change
```

## Expected Report Shape

```text
replacement_front_selected_route=page_local_alloc_route_cfg_preflight
replacement_front_selected_memop_family=page_local_alloc_route_cfg
replacement_front_next_producer_slice=page_local_alloc_route_cfg_producer_pilot

page_local_alloc_route_cfg_selected=1
page_local_alloc_route_cfg_lowering_enabled=0
page_local_alloc_route_branch_claim=0

page_local_alloc_route_report_v0=1
page_local_alloc_route_verified_plan_source=fastmem_access_plans

type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=<unchanged from current profile>
hook_install=<unchanged from current profile>
global_allocator_claim=<unchanged from current profile>
winner_claim=<unchanged from current profile>
```

## Source Fixture

Add one narrow fixture:

```text
lang/src/hako_alloc/memory/page_meta_page_local_alloc_route_cfg_preflight_box.hako
```

The fixture should express the route shape as source truth, but the preflight
row must keep CFG lowering disabled. If the current builder rejects source
branching for this shape, the report/check surface should still be able to
select the row without claiming execution.

### Fixture Shape

Use the smallest branch shape that the current MIRBuilder can represent:

```hako
if same_owner {
    page.used = next_local_used
} else {
    page.used = next_free_used
}
return page.used
```

Do not use branch-local returns in this row:

```hako
if same_owner {
    return local_result
} else {
    return free_result
}
```

That shape currently trips the MIRBuilder lexical-scope guard:

```text
[freeze:contract][lexical_scope/unbalanced_pop]
```

MIM-PORT-FMEM-076 should not fix that. It is a separate builder acceptance
shape, tracked by:

```text
docs/development/current/main/phases/phase-296x/296x-575-MIR-BUILDER-FASTMEM-BRANCH-RETURN-SCOPE-FIX.md
```

## Acceptance Sketch

```text
new preflight route is selected by report/check
next producer slice is page_local_alloc_route_cfg_producer_pilot
page_local_alloc_route_cfg_lowering_enabled remains 0
existing straight-line allocation body smokes still pass
FastMemory check smoke remains green
FastMemory source syntax smoke remains green
current state pointer guard passes
git diff --check passes
```

## Non-goals

```text
implementing page-local alloc route CFG producer lowering
opening source allocation branch execution
accepting fastmem branch-local return shapes
retiring diagnostic Python-template C bridge
changing product activation or allocator claim behavior
```
