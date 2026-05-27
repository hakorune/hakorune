---
Status: Current
Date: 2026-05-27
Scope: implement one selectPage single-page fast path keeper.
Blocker: HAKO-MIMALLOC-PERF-SELECT-PAGE-SINGLE-PAGE-FAST-PATH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-80-HAKO-MIMALLOC-PERF-NEXT-KEEPER-SELECTION.md
  - lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
  - lang/src/hako_alloc/memory/object_lifecycle_page_queue_box.hako
---

# 296x-81 Hako Mimalloc Perf SelectPage Single-Page Fast Path

## Purpose

Implement exactly one next keeper selected by row 80: avoid the full
`selectPage` scan on the object-lifecycle small-alloc path when the queue has a
single known usable page.

## Required Output

```text
output_contract=hako-mimalloc-perf-select-page-single-page-fast-path-v0
input_contract=hako-mimalloc-perf-next-keeper-selection-v0
keeper=select_page_single_page_fast_path
target_method=objectLifecycleSmallAlloc
winner_claim=0
replacement_active=0
summary=ok
```

## Stop Line

Do not combine this with result-capsule reduction or observer getter reduction.
