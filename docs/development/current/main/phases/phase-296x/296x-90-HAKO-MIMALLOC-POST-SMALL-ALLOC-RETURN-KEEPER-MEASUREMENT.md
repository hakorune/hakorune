---
Status: Current
Date: 2026-05-27
Scope: measure the object-lifecycle facade exact-EXE after the small-alloc selected-page return keeper.
Blocker: HAKO-MIMALLOC-POST-SMALL-ALLOC-RETURN-KEEPER-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-89-HAKO-MIMALLOC-SMALL-ALLOC-SELECTED-PAGE-RETURN-KEEPER.md
---

# 296x-90 Hako Mimalloc Post Small-Alloc Return Keeper Measurement

## Purpose

Rerun the object-lifecycle facade exact-EXE measurement after row89. Preserve
semantic counters and keep winner/replacement claims closed.

## Required Output

```text
output_contract=hako-mimalloc-post-small-alloc-return-keeper-measurement-v0
input_contract=hako-mimalloc-small-alloc-selected-page-return-keeper-v0
measurement_scope=object_lifecycle_facade_exact_exe_after_small_alloc_return_keeper
workload_id=representative-object-lifecycle-small-block-v0
operation_repeat=8192
sample_count
after_hako_elapsed_median_ms
select_page_single_fast_path_count=524288
release_known_page_fast_path_count=524288
winner_claim=0
replacement_active=0
summary=ok
```

## Stop Line

Do not implement another keeper in this measurement row.
