---
Status: Current
Date: 2026-05-28
Scope: select the first static scalar method fact boundary after single-eval correctness fixes.
Blocker: STATIC-SCALAR-METHOD-FACT-SELECTION-296X-001
Related:
  - docs/development/current/main/design/nested-argument-single-evaluation-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-134-MIR-BUILDER-SINGLE-EVAL-SURFACE-SWEEP.md
---

# 296x-135 Static Scalar Method Fact Selection

## Purpose

Select the first narrow static-scalar method fact boundary. This is not generic
CSE. The candidate must be verified by method body shape, such as same-module
zero-arg static method returning a literal scalar with no calls, fields,
allocation, branches, or safepoints.

## Required Output

```text
output_contract=static-scalar-method-fact-selection-v0
input_contract=mir-builder-single-eval-surface-sweep-v0
candidate_family
selection
selected_next
summary=ok
```

## Stop Line

Do not lower calls to constants in this row.
