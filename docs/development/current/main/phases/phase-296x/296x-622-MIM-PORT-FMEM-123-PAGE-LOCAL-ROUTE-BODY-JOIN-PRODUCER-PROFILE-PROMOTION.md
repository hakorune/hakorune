---
Status: Done
Date: 2026-06-08
Scope: MIM-PORT-FMEM-123.
Related:
  - docs/development/current/main/phases/phase-296x/296x-621-MIM-PORT-FMEM-122-PAGE-LOCAL-ROUTE-BODY-JOIN-PREFLIGHT-PROMOTION.md
  - docs/development/current/main/phases/phase-296x/296x-579-MIM-PORT-FMEM-080-PAGE-LOCAL-ALLOC-FREE-ROUTE-BODY-JOIN-PRODUCER-PILOT.md
  - lang/src/hako_alloc/memory/page_meta_page_local_alloc_route_cfg_preflight_box.hako
  - tools/hako_check/manifests/fastmem_source_syntax_smoke.toml
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-622 MIM-PORT-FMEM-123 Page-Local Route Body Join Producer Profile Promotion

## Purpose

Promote the page-local route body join producer profile for
`page_meta_page_local_alloc_route_cfg_preflight_box.hako` into the manifest
runner.

This row reuses the same source fixture and MIR from the preflight row, but it
opens the producer evidence that keeps the join open while downstream terminal
ladder rows take over.

## Implementation

```text
fixture:
  PAGE_LOCAL_ROUTE_BODY_JOIN_PRODUCER

source:
  lang/src/hako_alloc/memory/page_meta_page_local_alloc_route_cfg_preflight_box.hako

producer profile:
  page-local-route-body-join

expected evidence:
  replacement_front_selected_route=page_local_route_body_join_producer_pilot
  replacement_front_selected_memop_family=page_local_route_body_join
  replacement_front_selected_memop_kinds=PageLocalRouteBodyJoinProducer
  replacement_front_next_producer_slice=terminal_ladder_refresh_preflight
  page_local_route_body_join_selected=1
  page_local_route_body_join_open=1
  page_local_alloc_route_cfg_lowering_enabled=1
  page_local_free_route_cfg_lowering_enabled=1
  Type ABI / Provider ABI hot paths closed
  product/hook/global/winner claims closed
```

The old bespoke shell assertions for this join producer were removed from
`tools/hako_check/fastmem_source_syntax_smoke.sh`. The shell keeps only the
shared AST/MIR generation for later terminal ladder checks.

## Closed

```text
TLS transfer
product activation / hooks / global allocator / winner
```

## Verification

```bash
python3 tools/hako_check/fastmem_source_manifest_runner.py \
  --manifest tools/hako_check/manifests/fastmem_source_syntax_smoke.toml \
  --only PAGE_LOCAL_ROUTE_BODY_JOIN_PRODUCER
bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed

```text
page_local_route_body_join producer evidence is now manifest-backed while the
shell keeps only the shared AST/MIR generation for later terminal ladder
checks.
```

## Closeout

```text
next: 296x-623 terminal ladder shared-input split
```
