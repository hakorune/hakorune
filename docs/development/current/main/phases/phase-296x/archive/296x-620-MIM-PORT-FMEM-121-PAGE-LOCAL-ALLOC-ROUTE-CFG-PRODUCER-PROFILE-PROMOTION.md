---
Status: Done
Date: 2026-06-08
Scope: MIM-PORT-FMEM-121.
Related:
  - docs/development/current/main/phases/phase-296x/296x-619-MIM-PORT-FMEM-120-PAGE-LOCAL-ALLOC-ROUTE-CFG-PREFLIGHT-MANIFEST-PROMOTION.md
  - docs/development/current/main/phases/phase-296x/296x-576-MIM-PORT-FMEM-077-PAGE-LOCAL-ALLOC-ROUTE-CFG-PRODUCER-PILOT.md
  - lang/src/hako_alloc/memory/page_meta_page_local_alloc_route_cfg_preflight_box.hako
  - tools/hako_check/manifests/fastmem_source_syntax_smoke.toml
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-620 MIM-PORT-FMEM-121 Page-Local Alloc Route CFG Producer Profile Promotion

## Purpose

Promote the `page-local-alloc-route-cfg` producer profile for
`page_meta_page_local_alloc_route_cfg_preflight_box.hako` into the manifest
runner.

This row reuses the same source fixture and MIR from MIM-PORT-FMEM-120, but it
opens the producer profile evidence that later route-body join and terminal
ladder rows depend on.

## Implementation

```text
fixture:
  PAGE_LOCAL_ALLOC_ROUTE_CFG_PRODUCER

source:
  lang/src/hako_alloc/memory/page_meta_page_local_alloc_route_cfg_preflight_box.hako

producer profile:
  page-local-alloc-route-cfg

expected evidence:
  fastmem_page_local_alloc_route_cfg_producer_pilot=1
  replacement_front_selected_route=page_local_alloc_route_cfg_producer_pilot
  replacement_front_selected_memop_family=page_local_alloc_route_cfg
  replacement_front_selected_memop_kinds=PageLocalAllocRouteCfgProducer
  replacement_front_next_producer_slice=page_local_free_route_cfg_preflight
  page_local_alloc_route_cfg_selected=1
  page_local_alloc_route_cfg_lowering_enabled=1
  page_local_alloc_route_branch_claim=0
  fastmem_branch_cfg_open=1
  fastmem_branch_cfg_lowered_count=1
  Type ABI / Provider ABI hot paths closed
  product/global/winner claims closed
```

The old bespoke shell assertions for this producer profile were removed from
`tools/hako_check/fastmem_source_syntax_smoke.sh`. The shell keeps only the
shared AST/MIR generation and the later route-body join / terminal ladder rows.

## Closed

```text
page-local alloc branch claim
page-local free route claim
terminal ladder refresh
TLS transfer
product activation / hooks / global allocator / winner
```

## Verification

```bash
python3 tools/hako_check/fastmem_source_manifest_runner.py \
  --manifest tools/hako_check/manifests/fastmem_source_syntax_smoke.toml \
  --only PAGE_LOCAL_ALLOC_ROUTE_CFG_PRODUCER
bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed

```text
page_local_alloc_route_cfg producer evidence is now manifest-backed while the
shell keeps only the shared AST/MIR generation for downstream route-body join
and terminal ladder checks.
```

## Closeout

```text
next: 296x-621 page-local route body join preflight promotion
```
