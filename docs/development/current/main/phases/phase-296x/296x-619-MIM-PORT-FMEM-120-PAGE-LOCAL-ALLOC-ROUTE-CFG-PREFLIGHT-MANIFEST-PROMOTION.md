---
Status: Done
Date: 2026-06-08
Scope: MIM-PORT-FMEM-120.
Related:
  - docs/development/current/main/phases/phase-296x/296x-618-MIM-PORT-FMEM-119-REMAINING-SOURCE-SYNTAX-SMOKE-RETIREMENT-TASK-ORDER.md
  - docs/development/current/main/phases/phase-296x/296x-574-MIM-PORT-FMEM-076-PAGE-LOCAL-ALLOC-ROUTE-CFG-PREFLIGHT.md
  - lang/src/hako_alloc/memory/page_meta_page_local_alloc_route_cfg_preflight_box.hako
  - tools/hako_check/manifests/fastmem_source_syntax_smoke.toml
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-619 MIM-PORT-FMEM-120 Page-Local Alloc Route CFG Preflight Manifest Promotion

## Purpose

Promote `page_meta_page_local_alloc_route_cfg_preflight_box.hako` from the
legacy source-syntax smoke block into the manifest runner.

The shell smoke still emits this file's AST/MIR once because later route-body
join and terminal ladder checks consume the MIR as shared input. The
preflight report/check assertions themselves are now manifest-owned.

## Implementation

```text
fixture:
  PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT

source:
  lang/src/hako_alloc/memory/page_meta_page_local_alloc_route_cfg_preflight_box.hako

producer profile:
  page-local-alloc-route-cfg-preflight

expected evidence:
  replacement_front_selected_route=page_local_alloc_route_cfg_preflight
  replacement_front_selected_memop_family=page_local_alloc_route_cfg
  replacement_front_next_producer_slice=page_local_alloc_route_cfg_producer_pilot
  page_local_alloc_route_cfg_selected=1
  page_local_alloc_route_cfg_lowering_enabled=0
  page_local_alloc_route_branch_claim=0
  Type ABI / Provider ABI hot paths closed
  product/global/winner claims closed
```

The old bespoke shell assertions for this body were removed from
`tools/hako_check/fastmem_source_syntax_smoke.sh`. Only the AST/MIR generation
needed by later route-body join and terminal ladder checks remains.

## Closed

```text
page-local route branch execution
route-body join producer
terminal ladder refresh
multi-block refill
TLS transfer
product activation / hooks / global allocator / winner
```

## Verification

```bash
python3 tools/hako_check/fastmem_source_manifest_runner.py \
  --manifest tools/hako_check/manifests/fastmem_source_syntax_smoke.toml \
  --only PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT
bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed

```text
page_local_alloc_route_cfg_preflight is now manifest-backed with AST, MIR,
preflight report, and fastmem-check expected KV fixtures. The shell keeps only
shared AST/MIR generation for later route-body join and terminal ladder checks.
```

## Closeout

```text
next: 296x-620 page-local alloc route CFG producer profile promotion
```
