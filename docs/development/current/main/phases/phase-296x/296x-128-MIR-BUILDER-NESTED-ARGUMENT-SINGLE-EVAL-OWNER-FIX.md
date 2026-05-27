---
Status: Current
Date: 2026-05-28
Scope: fix MIR builder nested argument single-evaluation correctness.
Blocker: MIR-BUILDER-NESTED-ARGUMENT-SINGLE-EVAL-OWNER-FIX-296X-001
Related:
  - docs/development/current/main/design/nested-argument-single-evaluation-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-127-GENERIC-NESTED-ARGUMENT-SINGLE-EVAL-FIXTURE.md
---

# 296x-128 MIR Builder Nested Argument Single Eval Owner Fix

## Purpose

Row127 proved a generic duplicate-evaluation bug:

```text
fixture=generic_nested_argument_single_eval
nested_call_symbol=NestedArgumentSide.tick/0
expected_nested_call_count=1
actual_nested_call_count=2
```

Fix MIR builder lowering so nested argument expressions are evaluated exactly
once before the outer call.

## Required Output

```text
output_contract=mir-builder-nested-argument-single-eval-owner-fix-v0
input_contract=generic-nested-argument-single-eval-fixture-v0
fixture=generic_nested_argument_single_eval
actual_nested_call_count=1
facade_reason_duplicate_failure_count
semantic_summary=ok
summary=ok
```

## Stop Line

Do not add generic CSE or static scalar lowering in this row.
