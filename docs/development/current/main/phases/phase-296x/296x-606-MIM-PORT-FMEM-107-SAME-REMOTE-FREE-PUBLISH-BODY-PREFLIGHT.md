---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-107.
Related:
  - docs/development/current/main/phases/phase-296x/296x-605-MIM-PORT-FMEM-106-HAKO-ALLOC-NEXT-BODY-SLICE-SELECTION.md
  - lang/src/hako_alloc/memory/page_meta_same_owner_free_body_box.hako
  - lang/src/hako_alloc/memory/page_meta_atomic_remote_head_push_vocabulary_box.hako
---

# 296x-606 MIM-PORT-FMEM-107 Same/Remote Free Publish Body Preflight

## Purpose

Add the narrow source preflight for a `.hako fastmem` free/release body that
routes a block by owner:

```text
same owner:
  LocalFreePush + page.used decrement

remote owner:
  AtomicRemoteHeadPush
```

This is still page-meta-local. It does not wire `PageMapReleaseSeam.releasePtr`
or caller pointer lookup.

## Chosen Mode

```text
BoxCount
```

## Required Boundary

```text
one new source fixture/body only
use existing MemOps only
do not add source-level PageMapBridge or pointer-derived lookup
do not open TLS transfer, product activation, hooks, global allocator claim, or winner behavior
do not claim full hako mimalloc algorithm completion
```

## Acceptance Sketch

```text
new source body parses and lowers through existing fastmem branch CFG support
same branch selects LocalFreePush evidence
remote branch selects AtomicRemoteHeadPush evidence
same_remote_free_body report/check family remains green
fastmem_source_syntax_smoke stays green
```

## Verification

```bash
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
```
