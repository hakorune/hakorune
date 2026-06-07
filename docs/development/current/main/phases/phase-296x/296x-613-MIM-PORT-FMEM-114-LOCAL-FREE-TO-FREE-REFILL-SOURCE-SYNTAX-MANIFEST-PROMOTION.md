---
Status: Done
Date: 2026-06-08
Scope: MIM-PORT-FMEM-114.
Related:
  - docs/development/current/main/phases/phase-296x/296x-612-MIM-PORT-FMEM-113-HAKO-ALLOC-NEXT-BODY-SLICE-SELECTION.md
  - lang/src/hako_alloc/memory/page_meta_local_free_to_free_refill_body_box.hako
  - tools/hako_check/fastmem_source_syntax_smoke.sh
  - tools/hako_check/manifests/fastmem_source_syntax_smoke.toml
---

# 296x-613 MIM-PORT-FMEM-114 Local-Free To Free Refill Source-Syntax Manifest Promotion

## Purpose

Promote the selected single-block `local_free_head -> free_head` refill body
from the remaining legacy source-syntax smoke block into the compact manifest
runner introduced by 296x-608.

This is a smoke ownership move only. It does not change the `.hako` body, add
new MemOps, open multi-block refill, or change allocator semantics.

## Chosen Mode

```text
BoxShape
```

## Implementation

```text
Add manifest fixture:
  LOCAL_FREE_TO_FREE_REFILL_BODY

Source:
  lang/src/hako_alloc/memory/page_meta_local_free_to_free_refill_body_box.hako

Producer profile:
  local-free

Expected evidence:
  LocalFreePop(page)
  FreeHeadPush(page, block)
  verified access plans complete
  Type ABI / Provider ABI hot paths closed
  product/global/winner claims closed
```

The bespoke shell block for this body was removed from
`tools/hako_check/fastmem_source_syntax_smoke.sh`. The counter and
refill-then-alloc bodies remain in the legacy block for later rows.

## Required Boundary

```text
do not change page_meta_local_free_to_free_refill_body_box.hako
do not promote the counter body in this row
do not promote refill_then_free_head_alloc in this row
do not open multi-block refill
do not open branch route execution
do not open TLS transfer
do not open product activation, hook install, global allocator claim, or winner claim
```

## Verification

```bash
python3 tools/hako_check/fastmem_source_manifest_runner.py \
  --manifest tools/hako_check/manifests/fastmem_source_syntax_smoke.toml \
  --only LOCAL_FREE_TO_FREE_REFILL_BODY
bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed

```text
LOCAL_FREE_TO_FREE_REFILL_BODY is now manifest-backed. The selected refill body
has expected AST/MIR/report/check KV fixtures, and the old bespoke shell block
for that body is removed while later refill/counter candidates stay untouched.
```

## Closeout

```text
next: 296x-614 remaining fastmem migration target inventory
```
