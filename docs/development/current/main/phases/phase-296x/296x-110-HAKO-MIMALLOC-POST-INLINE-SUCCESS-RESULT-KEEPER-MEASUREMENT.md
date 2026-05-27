---
Status: Current
Date: 2026-05-27
Scope: measure the object-lifecycle facade exact-EXE after the inline success result keeper.
Blocker: HAKO-MIMALLOC-POST-INLINE-SUCCESS-RESULT-KEEPER-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-109-HAKO-MIMALLOC-SMALL-ALLOC-INLINE-SUCCESS-RESULT-FAST-PATH-KEEPER.md
---

# 296x-110 Hako Mimalloc Post Inline Success Result Keeper Measurement

## Purpose

Rerun the repeated exact-EXE measurement after row109 so the inline success
result keeper is judged by measured behavior.

## Required Output

```text
output_contract=hako-mimalloc-post-inline-success-result-keeper-measurement-v0
input_contract=hako-mimalloc-small-alloc-inline-success-result-fast-path-keeper-v0
measurement_scope=object_lifecycle_facade_exact_exe_after_inline_success_result_keeper
keeper=small_alloc_inline_success_result_fast_path
sample_count
after_hako_elapsed_median_ms
select_page_single_fast_path_count
select_page_single_fallback_count=0
release_known_page_fast_path_count
winner_claim=0
replacement_active=0
summary=ok
```

## Stop Line

Do not implement another keeper, open provider activation, replacement, hooks,
globals, or winner claims in this row.
