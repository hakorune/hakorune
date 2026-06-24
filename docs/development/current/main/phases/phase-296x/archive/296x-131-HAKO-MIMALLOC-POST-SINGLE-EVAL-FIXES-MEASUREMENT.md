---
Status: Landed
Date: 2026-05-28
Scope: measure the object-lifecycle facade exact-EXE after the nested-field owner fix.
Blocker: HAKO-MIMALLOC-POST-SINGLE-EVAL-FIXES-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-130-MIR-BUILDER-NESTED-FIELD-SINGLE-EVAL-OWNER-FIX.md
---

# 296x-131 Hako Mimalloc Post Single Eval Fixes Measurement

## Purpose

Measure the object-lifecycle facade exact-EXE after the nested-field owner fix
and keep the single-eval fix evidence compact.

## Required Output

```text
output_contract=hako-mimalloc-post-single-eval-fixes-measurement-v0
input_contract=mir-builder-nested-field-single-eval-owner-fix-v0
measurement_profile=object_lifecycle_facade_exact_exe
summary=ok
```

## Evidence

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
sample_0_hako_external_elapsed_ms=360
elapsed_median_ms=360
elapsed_min_ms=360
elapsed_max_ms=360
external_rss_median_bytes=3575808
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

Do not add another MIR-builder fix in this row.
