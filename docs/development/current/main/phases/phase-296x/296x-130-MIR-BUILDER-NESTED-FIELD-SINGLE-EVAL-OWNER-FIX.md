---
Status: Current
Date: 2026-05-28
Scope: fix MIR builder nested field access single-evaluation correctness.
Blocker: MIR-BUILDER-NESTED-FIELD-SINGLE-EVAL-OWNER-FIX-296X-001
Related:
  - docs/development/current/main/design/nested-argument-single-evaluation-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-129-MIR-BUILDER-NESTED-FIELD-SINGLE-EVAL-FIXTURE.md
---

# 296x-130 MIR Builder Nested Field Single Eval Owner Fix

## Purpose

Fix `object.field` lowering so field-origin / weak-field inference never
re-lowers already evaluated nested object expressions.

## Required Output

```text
output_contract=mir-builder-nested-field-single-eval-owner-fix-v0
input_contract=mir-builder-nested-field-single-eval-fixture-v0
fixture=nested_field_single_eval
actual_nested_call_count=1
owner_fix
semantic_summary=ok
summary=ok
```

## Stop Line

Do not add generic CSE, static scalar lowering, or broad field-shape rewrites in
this row.
