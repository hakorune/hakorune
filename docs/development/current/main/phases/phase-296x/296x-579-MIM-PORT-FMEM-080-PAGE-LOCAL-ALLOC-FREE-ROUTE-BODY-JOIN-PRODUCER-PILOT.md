---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-080.
Related:
  - docs/development/current/main/phases/phase-296x/296x-578-MIM-PORT-FMEM-079-PAGE-LOCAL-ALLOC-FREE-ROUTE-BODY-JOIN-PREFLIGHT.md
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_check.py
---

# 296x-579 MIM-PORT-FMEM-080 Page-Local Alloc/Free Route Body Join Producer Pilot

## Purpose

Open the producer pilot for the page-local alloc/free route body join selected
by MIM-PORT-FMEM-079.

This row should move the join boundary from preflight to producer evidence,
without changing product activation or allocator installation claims.

## Required Boundaries

```text
producer-evidence row
no new MemOp kind
no new FastMemory source syntax
no product activation / hook / global allocator / winner claim change
no TLS backing transfer / owner slot reuse / abandoned reclaim change
no diagnostic Python-template C bridge retirement
```

## Expected Report Shape

```text
replacement_front_selected_route=page_local_route_body_join_producer_pilot
replacement_front_selected_memop_family=page_local_route_body_join
replacement_front_selected_memop_kinds=PageLocalRouteBodyJoinProducer
replacement_front_next_producer_slice=tls_backing_transfer_preflight

page_local_route_body_join_selected=1
page_local_route_body_join_open=1
page_local_alloc_route_cfg_lowering_enabled=1
page_local_free_route_cfg_lowering_enabled=1

product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
```

## Acceptance Sketch

```text
producer profile exists for page_local_route_body_join_producer_pilot
preflight profile remains green
join producer opens only page_local_route_body_join_open
closed activation / hook / allocator / winner claims remain 0
FastMemory check smoke remains green
FastMemory source syntax smoke remains green
current state pointer guard passes
git diff --check passes
```

## Non-goals

```text
opening product allocator replacement
changing TLS/owner lifecycle rows
retiring the diagnostic Python-template C bridge
```
