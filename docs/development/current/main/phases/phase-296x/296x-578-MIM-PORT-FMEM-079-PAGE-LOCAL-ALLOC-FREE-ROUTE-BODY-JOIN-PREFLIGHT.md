---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-079.
Related:
  - docs/development/current/main/phases/phase-296x/296x-555-MIM-PORT-FMEM-057-PAGE-LOCAL-FREE-ROUTE-CFG-PRODUCER-PILOT.md
  - docs/development/current/main/phases/phase-296x/296x-576-MIM-PORT-FMEM-077-PAGE-LOCAL-ALLOC-ROUTE-CFG-PRODUCER-PILOT.md
  - docs/development/current/main/phases/phase-296x/296x-577-MIM-PORT-FMEM-078-HAKO-ALLOC-ROUTE-BODY-REENTRY-SELECTION.md
---

# 296x-578 MIM-PORT-FMEM-079 Page-Local Alloc/Free Route Body Join Preflight

## Purpose

Add a preflight row that proves the page-local allocation route CFG and the
page-local free route CFG are both visible as route-body evidence before the
lane returns to later terminal/product claims.

This row is a report/check join boundary. It does not open a new allocator
execution claim.

## Required Boundaries

```text
join-preflight row only
no new MemOp kind
no new FastMemory source syntax
no product activation / hook / global allocator / winner claim change
no TLS backing transfer / owner slot reuse / abandoned reclaim change
no diagnostic Python-template C bridge retirement
```

## Expected Report Shape

```text
replacement_front_selected_route=page_local_route_body_join_preflight
replacement_front_selected_memop_family=page_local_route_body_join
replacement_front_selected_memop_kinds=PageLocalRouteBodyJoin
replacement_front_next_producer_slice=page_local_route_body_join_producer_pilot

page_local_alloc_route_cfg_selected=1
page_local_alloc_route_cfg_lowering_enabled=1
page_local_free_route_cfg_selected=1
page_local_free_route_cfg_lowering_enabled=1
page_local_route_body_join_selected=1
page_local_route_body_join_open=0

type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
```

## Acceptance Sketch

```text
report/check profile exists for page_local_route_body_join_preflight
positive evidence requires both alloc and free route CFG evidence
join row remains preflight: page_local_route_body_join_open=0
later activation / hook / allocator / winner claims remain closed
FastMemory check smoke remains green
FastMemory source syntax smoke remains green
current state pointer guard passes
git diff --check passes
```

## Non-goals

```text
opening page-local route execution as a product allocator
adding branch/path-sensitive allocation semantics
changing existing page-local alloc/free route fixtures
```
