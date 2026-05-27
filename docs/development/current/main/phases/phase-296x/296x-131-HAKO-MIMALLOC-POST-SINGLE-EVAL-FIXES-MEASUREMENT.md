---
Status: Planned
Date: 2026-05-28
Scope: measure object-lifecycle facade exact-EXE after nested argument and field single-eval fixes.
Blocker: HAKO-MIMALLOC-POST-SINGLE-EVAL-FIXES-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-128-MIR-BUILDER-NESTED-ARGUMENT-SINGLE-EVAL-OWNER-FIX.md
  - docs/development/current/main/phases/phase-296x/296x-130-MIR-BUILDER-NESTED-FIELD-SINGLE-EVAL-OWNER-FIX.md
---

# 296x-131 Hako Mimalloc Post Single Eval Fixes Measurement

## Purpose

Measure the object-lifecycle facade exact-EXE after the compiler
single-evaluation correctness fixes are closed.

## Required Output

```text
output_contract=hako-mimalloc-post-single-eval-fixes-measurement-v0
input_contract=mir-builder-nested-field-single-eval-owner-fix-v0
measurement_profile=object_lifecycle_facade_exact_exe
sample_count
elapsed_median_ms
winner_claim=0
replacement_active=0
summary=ok
```

## Stop Line

Do not add static scalar method fact inference or lowering in this row.
