---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Capture and gate the read_number decimal/exponent staged-loop compiler fixture.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1276-COREPLAN-RECURSIVE-RECIPE-REAL-SHAPE-INTAKE-TASKBOARD-001.md
  - docs/development/current/main/phases/phase-296x/296x-1277-COREPLAN-REAL-SHAPE-FIXTURE-SIGN-BREAK-001.md
  - apps/tests/phase29bq_selfhost_blocker_read_number_decimal_exponent_min.hako
  - tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv
---

# COREPLAN-REAL-SHAPE-FIXTURE-DECIMAL-EXPONENT-001

## Decision

The `read_number()` decimal/exponent staged-loop fixture is captured and
planner-required green. No Recipe/CorePlan implementation is opened for this
shape.

## Captured Shape

Fixture:

```text
apps/tests/phase29bq_selfhost_blocker_read_number_decimal_exponent_min.hako
```

Shape:

```text
optional leading sign
integer digit loop
optional decimal stage
decimal validation early return
decimal digit loop
optional exponent stage
optional exponent sign
exponent validation early return
exponent digit loop
loop-carried scanner cursor across stages
```

Expected output:

```text
9
```

## Evidence

Gate entry:

```text
case_id=selfhost_read_number_decimal_exponent_min
expected=9
planner_tag=[joinir/planner_first rule=LoopSimpleWhile] label=LoopSimpleWhile
```

Command:

```bash
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only selfhost_read_number_decimal_exponent_min
```

Result:

```text
[PASS] phase29bq_fast_gate_cases:selfhost_read_number_decimal_exponent_min
[PASS] phase29bq_fast_gate_vm: PASS (mode=selfhost_read_number_decimal_exponent_min)
```

## Interpretation

The staged decimal/exponent loops are already accepted as repeated
LoopSimpleWhile regions. This proves the decimal/exponent scanner shape does
not need a new Recipe/CorePlan rule by itself.

The next remaining compiler-side intake is scanner multi-exit behavior:

```text
value exit
error/null exit
EOF exit
shared scanner state
```

## Stop Lines

```text
do not add Recipe/CorePlan code for this green fixture
do not infer full scanner multi-exit support from repeated LoopSimpleWhile
do not mix json_native token payload hardening into this compiler fixture row
do not add method-name branches
```

## Report

```text
output_contract=coreplan-real-shape-fixture-decimal-exponent-v0
fixture_added=1
gate_case_added=1
planner_required_green=1
new_recipe_acceptance_required=0
implementation_allowed=0
next_task=COREPLAN-REAL-SHAPE-FIXTURE-MULTI-EXIT-001
summary=ok
```
