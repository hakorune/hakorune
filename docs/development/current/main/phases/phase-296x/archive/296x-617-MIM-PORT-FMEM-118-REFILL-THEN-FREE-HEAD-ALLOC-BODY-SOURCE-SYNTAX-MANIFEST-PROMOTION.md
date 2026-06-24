---
Status: Done
Date: 2026-06-08
Scope: MIM-PORT-FMEM-118.
Related:
  - docs/development/current/main/phases/phase-296x/296x-614-MIM-PORT-FMEM-115-REMAINING-FASTMEM-MIGRATION-TARGET-INVENTORY.md
  - lang/src/hako_alloc/memory/page_meta_refill_then_free_head_alloc_body_box.hako
  - tools/hako_check/manifests/fastmem_source_syntax_smoke.toml
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-617 MIM-PORT-FMEM-118 Refill-Then-Free-Head Alloc Body Source-Syntax Manifest Promotion

## Purpose

Promote `page_meta_refill_then_free_head_alloc_body_box.hako` from its legacy
source-syntax smoke assertions into the manifest runner.

The shell smoke still emits this file's AST/MIR once because later
winner/product preflight checks consume the MIR as shared input. The source body
assertions themselves are now manifest-owned.

## Implementation

```text
fixture:
  REFILL_THEN_FREE_HEAD_ALLOC_BODY

source:
  lang/src/hako_alloc/memory/page_meta_refill_then_free_head_alloc_body_box.hako

producer profile:
  local-free

expected evidence:
  LocalFreePop(page)
  FreeHeadPush(page, block)
  FreeHeadPop(page)
  derived free-head non-empty evidence
  page_local_alloc_route_candidate=refill_then_free_head_alloc
  Type ABI / Provider ABI hot paths closed
  product/global/winner claims closed
```

The old bespoke shell assertions for this body were removed from
`tools/hako_check/fastmem_source_syntax_smoke.sh`. Only the AST/MIR generation
needed by later terminal ladder checks remains.

## Closed

```text
page-local route branch execution
multi-block refill
TLS transfer
product activation / hooks / global allocator / winner
```

## Verification

```bash
python3 tools/hako_check/fastmem_source_manifest_runner.py \
  --manifest tools/hako_check/manifests/fastmem_source_syntax_smoke.toml \
  --only REFILL_THEN_FREE_HEAD_ALLOC_BODY
bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed

```text
REFILL_THEN_FREE_HEAD_ALLOC_BODY is now manifest-backed with AST, MIR,
local-free report, and fastmem-check expected KV fixtures. Later terminal
ladder checks still reuse its MIR from the shell smoke without owning its
source-body assertions.
```

## Closeout

```text
next: 296x-618 page-local alloc route CFG preflight source-syntax manifest
promotion
```
