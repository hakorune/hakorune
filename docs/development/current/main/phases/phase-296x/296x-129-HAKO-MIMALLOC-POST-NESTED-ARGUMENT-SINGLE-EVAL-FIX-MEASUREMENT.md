---
Status: Current
Date: 2026-05-28
Scope: measure object-lifecycle facade exact-EXE after the MIR builder single-eval correctness fix.
Blocker: HAKO-MIMALLOC-POST-NESTED-ARGUMENT-SINGLE-EVAL-FIX-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/design/nested-argument-single-evaluation-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-128-MIR-BUILDER-NESTED-ARGUMENT-SINGLE-EVAL-OWNER-FIX.md
---

# 296x-129 Hako Mimalloc Post Nested Argument Single Eval Fix Measurement

## Purpose

Row128 fixes a compiler correctness bug that also removes duplicate facade
reason getter calls. Measure the object-lifecycle facade exact-EXE again before
opening the static-scalar method fact lane.

## Required Output

```text
output_contract=hako-mimalloc-post-nested-argument-single-eval-fix-measurement-v0
input_contract=mir-builder-nested-argument-single-eval-owner-fix-v0
measurement_profile=object_lifecycle_facade_exact_exe
sample_count
elapsed_median_ms
facade_reason_duplicate_failure_count=0
winner_claim=0
replacement_active=0
summary=ok
```

## Stop Line

Do not add static scalar method fact inference or lowering in this row.
