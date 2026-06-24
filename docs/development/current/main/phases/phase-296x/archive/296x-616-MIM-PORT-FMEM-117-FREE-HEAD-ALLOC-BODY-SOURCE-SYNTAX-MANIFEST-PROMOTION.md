---
Status: Done
Date: 2026-06-08
Scope: MIM-PORT-FMEM-117.
Related:
  - docs/development/current/main/phases/phase-296x/296x-614-MIM-PORT-FMEM-115-REMAINING-FASTMEM-MIGRATION-TARGET-INVENTORY.md
  - lang/src/hako_alloc/memory/page_meta_free_head_alloc_body_box.hako
  - tools/hako_check/manifests/fastmem_source_syntax_smoke.toml
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-616 MIM-PORT-FMEM-117 Free-Head Alloc Body Source-Syntax Manifest Promotion

## Purpose

Promote `page_meta_free_head_alloc_body_box.hako` from the legacy
source-syntax smoke block into the manifest runner.

This keeps the allocation body observable through expected KV fixtures without
changing the `.hako` body or opening a broader allocation route.

## Implementation

```text
fixture:
  FREE_HEAD_ALLOC_BODY

source:
  lang/src/hako_alloc/memory/page_meta_free_head_alloc_body_box.hako

producer profile:
  local-free

expected evidence:
  FreeHeadPop(page)
  page_local_alloc_route_candidate=free_head_alloc
  same-owner and free-head non-empty facts present
  used + 1 field store evidence
  Type ABI / Provider ABI hot paths closed
  product/global/winner claims closed
```

The old bespoke shell assertions for this body were removed from
`tools/hako_check/fastmem_source_syntax_smoke.sh`.

## Closed

```text
refill composition
derived non-empty proof from FreeHeadPush
branch route execution
remote routing
activation/product claims
```

## Verification

```bash
python3 tools/hako_check/fastmem_source_manifest_runner.py \
  --manifest tools/hako_check/manifests/fastmem_source_syntax_smoke.toml \
  --only FREE_HEAD_ALLOC_BODY
bash tools/hako_check/fastmem_source_syntax_smoke.sh
```

## Landed

```text
FREE_HEAD_ALLOC_BODY is now manifest-backed with AST, MIR, local-free report,
and fastmem-check expected KV fixtures.
```

## Closeout

```text
next: 296x-617 refill-then-free-head allocation body source-syntax manifest
promotion
```
