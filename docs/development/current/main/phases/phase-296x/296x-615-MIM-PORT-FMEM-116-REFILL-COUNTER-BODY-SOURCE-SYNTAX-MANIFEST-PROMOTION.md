---
Status: Done
Date: 2026-06-08
Scope: MIM-PORT-FMEM-116.
Related:
  - docs/development/current/main/phases/phase-296x/296x-614-MIM-PORT-FMEM-115-REMAINING-FASTMEM-MIGRATION-TARGET-INVENTORY.md
  - lang/src/hako_alloc/memory/page_meta_local_free_to_free_refill_counter_body_box.hako
  - tools/hako_check/manifests/fastmem_source_syntax_smoke.toml
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-615 MIM-PORT-FMEM-116 Refill Counter Body Source-Syntax Manifest Promotion

## Purpose

Promote `page_meta_local_free_to_free_refill_counter_body_box.hako` from the
legacy source-syntax smoke block into the manifest runner.

This is a smoke ownership move only. It does not change `.hako` allocator
behavior, widen FastMemory MemOps, or open route execution.

## Implementation

```text
fixture:
  LOCAL_FREE_TO_FREE_REFILL_COUNTER_BODY

source:
  lang/src/hako_alloc/memory/page_meta_local_free_to_free_refill_counter_body_box.hako

producer profile:
  local-free

expected evidence:
  LocalFreePop(page)
  FreeHeadPush(page, block)
  refill counter field load/store evidence
  verified access plans complete
  Type ABI / Provider ABI hot paths closed
  product/global/winner claims closed
```

The old bespoke shell assertions for this body were removed from
`tools/hako_check/fastmem_source_syntax_smoke.sh`.

## Closed

```text
multi-block refill
refill-then-alloc
route CFG execution
PageMapRelease/realloc mutation
TLS transfer
product activation / hooks / global allocator / winner
```

## Verification

```bash
python3 tools/hako_check/fastmem_source_manifest_runner.py \
  --manifest tools/hako_check/manifests/fastmem_source_syntax_smoke.toml \
  --only LOCAL_FREE_TO_FREE_REFILL_COUNTER_BODY
bash tools/hako_check/fastmem_source_syntax_smoke.sh
```

## Landed

```text
LOCAL_FREE_TO_FREE_REFILL_COUNTER_BODY is now manifest-backed with AST, MIR,
local-free report, and fastmem-check expected KV fixtures.
```

## Closeout

```text
next: 296x-616 free-head allocation body source-syntax manifest promotion
```
