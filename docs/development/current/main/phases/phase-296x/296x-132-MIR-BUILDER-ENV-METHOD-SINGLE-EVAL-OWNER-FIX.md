---
Status: Current
Date: 2026-05-28
Scope: fix env method fallback single-evaluation correctness.
Blocker: MIR-BUILDER-ENV-METHOD-SINGLE-EVAL-OWNER-FIX-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-131-MIR-BUILDER-ENV-METHOD-SINGLE-EVAL-FIXTURE.md
---

# 296x-132 MIR Builder Env Method Single Eval Owner Fix

## Purpose

Fix `try_handle_env_method()` so it selects a supported env method spec before
lowering arguments.

## Required Output

```text
output_contract=mir-builder-env-method-single-eval-owner-fix-v0
input_contract=mir-builder-env-method-single-eval-fixture-v0
fixture=env_method_single_eval
actual_nested_call_count=1
owner_fix=env_method_spec_checked_before_argument_lowering
semantic_summary=ok
summary=ok
```

## Stop Line

Do not add generic CSE, static scalar lowering, or broad env route rewrites in
this row.
