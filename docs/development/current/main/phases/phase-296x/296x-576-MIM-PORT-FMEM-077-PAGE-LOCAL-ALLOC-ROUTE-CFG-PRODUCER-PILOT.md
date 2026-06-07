---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-077.
Related:
  - docs/development/current/main/phases/phase-296x/296x-574-MIM-PORT-FMEM-076-PAGE-LOCAL-ALLOC-ROUTE-CFG-PREFLIGHT.md
  - lang/src/hako_alloc/memory/page_meta_page_local_alloc_route_cfg_preflight_box.hako
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_check.py
---

# 296x-576 MIM-PORT-FMEM-077 Page-Local Alloc Route CFG Producer Pilot

## Purpose

Open producer evidence for the page-local allocation route CFG selected by
MIM-PORT-FMEM-076.

The row should prove the route envelope can move from preflight to producer
pilot, while still keeping the actual allocation route execution conservative.

## Required Boundaries

```text
producer-evidence row
no new MemOp kind
no branch-local return acceptance
no LayoutRef phi/join rule
no multi-block refill
no remote-owner free expansion
no TLS backing transfer change
no owner slot reuse / abandoned reclaim change
no product activation / hook / global allocator claim / winner claim change
```

## Expected Report Shape

```text
replacement_front_selected_route=page_local_alloc_route_cfg_producer_pilot
replacement_front_selected_memop_family=page_local_alloc_route_cfg
replacement_front_selected_memop_kinds=PageLocalAllocRouteCfgProducer
replacement_front_next_producer_slice=page_local_free_route_cfg_preflight

page_local_alloc_route_cfg_selected=1
page_local_alloc_route_cfg_lowering_enabled=1
page_local_alloc_route_branch_claim=0

page_local_alloc_route_report_v0=1
page_local_alloc_route_verified_plan_source=fastmem_access_plans

type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
```

## Source Fixture

Reuse:

```text
lang/src/hako_alloc/memory/page_meta_page_local_alloc_route_cfg_preflight_box.hako
```

The fixture remains the same minimal branch shape used by MIM-PORT-FMEM-076.
Do not introduce branch-local returns in this row; that remains tracked by
`296x-575`.

## Acceptance Sketch

```text
producer pilot route is selected by report/check
next producer slice returns to page_local_free_route_cfg_preflight
page_local_alloc_route_cfg_lowering_enabled becomes 1
page_local_alloc_route_branch_claim remains 0
closed activation / hook / allocator / winner claims remain 0
FastMemory check smoke remains green
FastMemory source syntax smoke remains green
current state pointer guard passes
git diff --check passes
```

## Verification

```bash
python3 -m py_compile tools/hako_check/fastmem_route_profiles.py tools/hako_check/fastmem_check_route_rules.py tools/hako_check/fastmem_mir_to_llvm_producer_report_rows.py tools/hako_check/fastmem_mir_to_llvm_producer_report_body.py tools/hako_check/fastmem_mir_to_llvm_producer_report_route_rows.py tools/hako_check/fastmem_mir_to_llvm_producer_report_tail_rows.py
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Non-goals

```text
opening product allocation route execution
accepting fastmem branch-local return shapes
retiring diagnostic Python-template C bridge
changing product activation or allocator claim behavior
```
