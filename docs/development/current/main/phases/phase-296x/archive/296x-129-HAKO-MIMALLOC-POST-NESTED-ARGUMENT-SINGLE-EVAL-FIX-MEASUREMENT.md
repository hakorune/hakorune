---
Status: Landed
Date: 2026-05-28
Scope: measure the nested-argument single-eval fix before continuing to the nested-field owner fix.
Blocker: HAKO-MIMALLOC-POST-NESTED-ARGUMENT-SINGLE-EVAL-FIX-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-128-MIR-BUILDER-NESTED-ARGUMENT-SINGLE-EVAL-OWNER-FIX.md
---

# 296x-129 Hako Mimalloc Post Nested Argument Single Eval Fix Measurement

## Purpose

Measure the nested-argument single-eval fix before moving to the nested-field
owner fix.

## Required Output

```text
output_contract=hako-mimalloc-post-nested-argument-single-eval-fix-measurement-v0
input_contract=mir-builder-nested-argument-single-eval-owner-fix-v0
measurement_scope=object_lifecycle_facade_exact_exe_after_nested_argument_single_eval_fix
summary=ok
```

## Landed Evidence

```text
output_contract=hako-mimalloc-post-single-eval-fixes-measurement-v0
input_contract=mir-builder-env-method-single-eval-owner-fix-v0
measurement_profile=object_lifecycle_facade_exact_exe
measurement_scope=object_lifecycle_facade_exact_exe_after_single_eval_fixes
workload_id=representative-object-lifecycle-small-block-v0
operation_repeat=8192
sample_count=1
select_page_single_fast_path_count=524288
release_known_page_fast_path_count=524288
sample_0_hako_external_elapsed_ms=370
elapsed_median_ms=370
elapsed_min_ms=370
elapsed_max_ms=370
external_rss_median_bytes=3604480
previous_checkpoint_hako_elapsed_median_ms=610
previous_checkpoint_source=296x-124-post-hako-reason-bind-measurement
single_eval_fix_effect=accepted
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Stop Line

Do not add another MIR-builder change in this row.

