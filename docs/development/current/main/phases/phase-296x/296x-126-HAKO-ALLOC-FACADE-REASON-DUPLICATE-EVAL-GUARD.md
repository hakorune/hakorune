---
Status: Current
Date: 2026-05-28
Scope: add a narrow guard for duplicate facade reason-call evaluation before MIR builder changes.
Blocker: HAKO-ALLOC-FACADE-REASON-DUPLICATE-EVAL-GUARD-296X-001
Related:
  - docs/development/current/main/design/nested-argument-single-evaluation-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-125-HAKO-MIMALLOC-POST-HAKO-REASON-BIND-SOURCE-MIR-REFRESH.md
---

# 296x-126 Hako Alloc Facade Reason Duplicate Eval Guard

## Purpose

Row125 found seven remaining duplicate reason-call evaluation candidates in the
object-lifecycle facade:

```text
failing_method_count=7
total_unused_duplicate_reason_call_count=20
```

Add a narrow guard around this proven bad shape before changing MIR builder
lowering.

## Required Output

```text
output_contract=hako-alloc-facade-reason-duplicate-eval-guard-v0
input_contract=hako-alloc-facade-reason-duplicate-inventory-v0
guard_scope=hako_alloc_object_lifecycle_facade_reason_calls
known_current_failure_count=7
selected_next=generic_nested_argument_single_eval_fixture
summary=ok
```

## Stop Line

Do not change MIR builder lowering in this row.
