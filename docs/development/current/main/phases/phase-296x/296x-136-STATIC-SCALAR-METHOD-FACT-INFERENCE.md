---
Status: Current
Date: 2026-05-28
Scope: infer static scalar method facts for the selected reason getter family without lowering calls.
Blocker: STATIC-SCALAR-METHOD-FACT-INFERENCE-296X-001
Related:
  - docs/development/current/main/design/nested-argument-single-evaluation-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-135-STATIC-SCALAR-METHOD-FACT-SELECTION.md
---

# 296x-136 Static Scalar Method Fact Inference

## Purpose

Add the first compiler-side fact surface for selected static scalar reason
getters. This row should record verified facts only; unsupported shapes must
keep the ordinary call path.

## Required Output

```text
output_contract=static-scalar-method-fact-inference-v0
input_contract=static-scalar-method-fact-selection-v0
fact_family
candidate_count
verified_fact_count
unverified_count
selected_next
summary=ok
```

## Stop Line

Do not lower calls to constants in this row.
