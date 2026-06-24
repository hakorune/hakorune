---
Status: Done
Date: 2026-06-08
Scope: MIM-PORT-FMEM-122.
Related:
  - docs/development/current/main/phases/phase-296x/296x-620-MIM-PORT-FMEM-121-PAGE-LOCAL-ALLOC-ROUTE-CFG-PRODUCER-PROFILE-PROMOTION.md
  - docs/development/current/main/phases/phase-296x/296x-578-MIM-PORT-FMEM-079-PAGE-LOCAL-ALLOC-FREE-ROUTE-BODY-JOIN-PREFLIGHT.md
  - lang/src/hako_alloc/memory/page_meta_page_local_alloc_route_cfg_preflight_box.hako
  - tools/hako_check/manifests/fastmem_source_syntax_smoke.toml
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-621 MIM-PORT-FMEM-122 Page-Local Route Body Join Preflight Promotion

## Purpose

Promote the page-local route body join preflight evidence for
`page_meta_page_local_alloc_route_cfg_preflight_box.hako` into the manifest
runner.

This row reuses the same source fixture and MIR from MIM-PORT-FMEM-120/121,
but it opens the join preflight boundary that proves both alloc and free route
CFG evidence are visible together.

## Implementation

```text
fixture:
  PAGE_LOCAL_ROUTE_BODY_JOIN_PREFLIGHT

source:
  lang/src/hako_alloc/memory/page_meta_page_local_alloc_route_cfg_preflight_box.hako

producer profile:
  page-local-route-body-join-preflight

expected evidence:
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
  Type ABI / Provider ABI hot paths closed
  product/hook/global/winner claims closed
```

The old bespoke shell assertions for this join preflight were removed from
`tools/hako_check/fastmem_source_syntax_smoke.sh`. The shell keeps only the
shared AST/MIR generation for later terminal ladder checks.

## Closed

```text
join producer
terminal ladder refresh
TLS transfer
product activation / hooks / global allocator / winner
```

## Verification

```bash
python3 tools/hako_check/fastmem_source_manifest_runner.py \
  --manifest tools/hako_check/manifests/fastmem_source_syntax_smoke.toml \
  --only PAGE_LOCAL_ROUTE_BODY_JOIN_PREFLIGHT
bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed

```text
page_local_route_body_join preflight evidence is now manifest-backed while the
shell keeps only the shared AST/MIR generation for later terminal ladder
checks.
```

## Closeout

```text
next: 296x-622 page-local route body join producer profile promotion
```
