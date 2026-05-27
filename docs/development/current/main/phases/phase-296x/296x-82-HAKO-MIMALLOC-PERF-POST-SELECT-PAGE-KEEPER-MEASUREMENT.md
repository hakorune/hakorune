---
Status: Current
Date: 2026-05-27
Scope: measure object-lifecycle facade exact-EXE after the selectPage single-page fast path keeper.
Blocker: HAKO-MIMALLOC-PERF-POST-SELECT-PAGE-KEEPER-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-81-HAKO-MIMALLOC-PERF-SELECT-PAGE-SINGLE-PAGE-FAST-PATH.md
  - apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako
---

# 296x-82 Hako Mimalloc Perf Post SelectPage Keeper Measurement

## Purpose

Measure the object-lifecycle facade exact-EXE path after the
`select_page_single_page_fast_path` keeper.

## Required Output

```text
output_contract=hako-mimalloc-perf-post-select-page-keeper-measurement-v0
input_contract=hako-mimalloc-perf-select-page-single-page-fast-path-v0
operation_repeat=8192
sample_count=3
keeper=select_page_single_page_fast_path
select_page_single_fast_path_count=524288
select_page_single_fallback_count=0
winner_claim=0
replacement_active=0
summary=ok
```

## Stop Line

This row measures only. hako_check perf-surface v1 belongs to row 83.
