---
Status: Current
Date: 2026-05-27
Scope: measure the object-lifecycle facade exact-EXE after rolling back the inline success result keeper.
Blocker: HAKO-MIMALLOC-POST-ROLLBACK-INLINE-SUCCESS-RESULT-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-111-HAKO-MIMALLOC-ROLLBACK-INLINE-SUCCESS-RESULT-KEEPER.md
---

# 296x-112 Hako Mimalloc Post Rollback Inline Success Result Measurement

## Purpose

Rerun the repeated exact-EXE measurement after row111 to confirm the source has
returned to the accepted direct select keeper baseline before selecting another
optimization.

## Required Output

```text
output_contract=hako-mimalloc-post-rollback-inline-success-result-measurement-v0
input_contract=hako-mimalloc-rollback-inline-success-result-keeper-v0
measurement_scope=object_lifecycle_facade_exact_exe_after_inline_success_result_rollback
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
