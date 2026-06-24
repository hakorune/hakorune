---
Status: Landed
Date: 2026-05-28
Scope: add a MIR correctness fixture for nested field access single evaluation.
Blocker: MIR-BUILDER-NESTED-FIELD-SINGLE-EVAL-FIXTURE-296X-001
Related:
  - docs/development/current/main/design/nested-argument-single-evaluation-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-128-MIR-BUILDER-NESTED-ARGUMENT-SINGLE-EVAL-OWNER-FIX.md
---

# 296x-129 MIR Builder Nested Field Single Eval Fixture

## Purpose

Row128 fixed duplicate nested argument evaluation. A follow-up temporary probe
found the same correctness class in nested field access:

```hako
return NestedFieldSide.make().child.value
```

`NestedFieldSide.make/0` must lower exactly once. Current MIR emits it twice,
because field-origin / weak-field inference re-lowers the inner object after the
field access object has already been evaluated.

## Required Output

```text
output_contract=mir-builder-nested-field-single-eval-fixture-v0
input_contract=mir-builder-nested-argument-single-eval-owner-fix-v0
fixture=nested_field_single_eval
nested_call_symbol=NestedFieldSide.make/0
expected_nested_call_count=1
actual_nested_call_count
selected_next=mir_builder_nested_field_single_eval_owner_fix
summary=ok
```

## Stop Line

Do not fix field access lowering in this row.

## Evidence

Report:

```text
output_contract=mir-builder-nested-field-single-eval-fixture-v0
input_contract=mir-builder-nested-argument-single-eval-owner-fix-v0
fixture=nested_field_single_eval
selected_method=NestedFieldProbe.run/0
nested_call_symbol=NestedFieldSide.make/0
expected_nested_call_count=1
actual_nested_call_count=2
selected_next=mir_builder_nested_field_single_eval_owner_fix
winner_claim=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_nested_field_single_eval_fixture_guard.sh
```
