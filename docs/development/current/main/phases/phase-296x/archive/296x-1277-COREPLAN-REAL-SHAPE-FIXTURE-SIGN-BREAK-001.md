---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Capture and gate the first full-ish read_number sign/break compiler fixture.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1276-COREPLAN-RECURSIVE-RECIPE-REAL-SHAPE-INTAKE-TASKBOARD-001.md
  - apps/tests/phase29bq_selfhost_blocker_read_number_sign_break_fullish_min.hako
  - tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv
---

# COREPLAN-REAL-SHAPE-FIXTURE-SIGN-BREAK-001

## Decision

The first stronger `read_next_number_literal`-family fixture is captured and
planner-required green. No Recipe/CorePlan implementation is opened for this
shape.

## Captured Shape

Fixture:

```text
apps/tests/phase29bq_selfhost_blocker_read_number_sign_break_fullish_min.hako
```

Shape:

```text
optional leading sign
bounded loop over text span
helper-based digit classification
conditional break on non-digit
loop-carried cursor
loop-carried accumulator
post-loop negative branch
```

Expected output:

```text
-123
```

## Evidence

Gate entry:

```text
case_id=selfhost_read_number_sign_break_fullish_min
expected=-123
planner_tag=[joinir/planner_first rule=LoopSimpleWhile] label=LoopSimpleWhile|[flowbox/adopt box_kind=Loop features=break via=shadow]
```

Command:

```bash
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only selfhost_read_number_sign_break_fullish_min
```

Result:

```text
[PASS] phase29bq_fast_gate_cases:selfhost_read_number_sign_break_fullish_min
[PASS] phase29bq_fast_gate_vm: PASS (mode=selfhost_read_number_sign_break_fullish_min)
```

## Interpretation

This proves the sign/break slice is already accepted by the current
LoopSimpleWhile plus break-adoption route. It does not prove the full
`JsonScanner.read_number()` shape with decimal/exponent stages or scanner
multi-exit behavior.

Next compiler-side intake should move to the decimal/exponent stage fixture.

## Stop Lines

```text
do not add Recipe/CorePlan code for this green fixture
do not infer full read_number support from this sign/break slice
do not mix json_native token payload hardening into this compiler fixture row
do not add method-name branches
```

## Report

```text
output_contract=coreplan-real-shape-fixture-sign-break-v0
fixture_added=1
gate_case_added=1
planner_required_green=1
new_recipe_acceptance_required=0
implementation_allowed=0
next_task=COREPLAN-REAL-SHAPE-FIXTURE-DECIMAL-EXPONENT-001
summary=ok
```
