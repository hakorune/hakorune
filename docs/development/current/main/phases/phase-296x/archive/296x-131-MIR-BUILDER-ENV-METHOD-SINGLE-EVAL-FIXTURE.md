---
Status: Landed
Date: 2026-05-28
Scope: add a MIR correctness fixture for env method fallback single evaluation.
Blocker: MIR-BUILDER-ENV-METHOD-SINGLE-EVAL-FIXTURE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-130-MIR-BUILDER-NESTED-FIELD-SINGLE-EVAL-OWNER-FIX.md
---

# 296x-131 MIR Builder Env Method Single Eval Fixture

## Purpose

Linnaeus found that `try_handle_env_method()` lowers arguments before checking
whether the `env.<iface>.<method>` route is actually supported. If the env
receiver shape matches but the method spec is missing, the route returns `None`
and the normal method fallback can lower the same argument expression again.

Add a fixture that proves this current bad shape before changing the owner.

## Required Output

```text
output_contract=mir-builder-env-method-single-eval-fixture-v0
input_contract=mir-builder-nested-field-single-eval-owner-fix-v0
fixture=env_method_single_eval
nested_call_symbol=EnvArgSide.tick/0
expected_nested_call_count=1
actual_nested_call_count
selected_next=mir_builder_env_method_single_eval_owner_fix
summary=ok
```

## Stop Line

Do not fix env method lowering in this row.

## Evidence

Report:

```text
output_contract=mir-builder-env-method-single-eval-fixture-v0
input_contract=mir-builder-nested-field-single-eval-owner-fix-v0
fixture=env_method_single_eval
selected_method=EnvMethodProbe.run/0
nested_call_symbol=EnvArgSide.tick/0
expected_nested_call_count=1
actual_nested_call_count=2
selected_next=mir_builder_env_method_single_eval_owner_fix
winner_claim=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_env_method_single_eval_fixture_guard.sh
```
