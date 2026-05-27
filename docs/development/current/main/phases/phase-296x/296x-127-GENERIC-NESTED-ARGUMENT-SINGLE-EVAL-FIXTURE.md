---
Status: Current
Date: 2026-05-28
Scope: add a generic MIR correctness fixture for nested argument single evaluation.
Blocker: GENERIC-NESTED-ARGUMENT-SINGLE-EVAL-FIXTURE-296X-001
Related:
  - docs/development/current/main/design/nested-argument-single-evaluation-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-126-HAKO-ALLOC-FACADE-REASON-DUPLICATE-EVAL-GUARD.md
---

# 296x-127 Generic Nested Argument Single Eval Fixture

## Purpose

Row126 fixed the hako-alloc facade duplicate-eval guard surface. Before fixing
MIR builder lowering, add a generic fixture that proves the correctness invariant
outside allocator reason getters:

```hako
return me.wrap(Side.tick())
```

`Side.tick/0` must lower exactly once.

## Required Output

```text
output_contract=generic-nested-argument-single-eval-fixture-v0
input_contract=hako-alloc-facade-reason-duplicate-eval-guard-v0
fixture=generic_nested_argument_single_eval
nested_call_symbol=NestedArgumentSide.tick/0
expected_nested_call_count=1
actual_nested_call_count
selected_next=mir_builder_nested_argument_single_eval_owner_fix
summary=ok
```

## Stop Line

Do not fix MIR builder lowering in this row.
