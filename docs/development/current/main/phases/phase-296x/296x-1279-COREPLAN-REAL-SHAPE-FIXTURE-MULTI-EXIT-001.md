---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Capture the scanner multi-exit fixture and fix the current Recipe/CorePlan blocker contract.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1276-COREPLAN-RECURSIVE-RECIPE-REAL-SHAPE-INTAKE-TASKBOARD-001.md
  - docs/development/current/main/phases/phase-296x/296x-1278-COREPLAN-REAL-SHAPE-FIXTURE-DECIMAL-EXPONENT-001.md
  - apps/tests/phase29bq_selfhost_blocker_scanner_multi_exit_min.hako
  - tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv
---

# COREPLAN-REAL-SHAPE-FIXTURE-MULTI-EXIT-001

## Decision

The scanner multi-exit shape is the first remaining compiler Recipe/CorePlan
blocker in the current `read_next_number_literal` ladder.

Unlike the sign/break and decimal/exponent fixtures, this fixture is not green.
It is now fixed as an expected-fail gate so the next row can implement exactly
one acceptance shape.

## Captured Shape

Fixture:

```text
apps/tests/phase29bq_selfhost_blocker_scanner_multi_exit_min.hako
```

Shape:

```text
loop over scanner cursor
value exit by break when closing marker is found
error exit by return for escape
error exit by return for newline
EOF error after loop when no closing marker was found
shared loop-carried cursor
shared loop-carried body_end marker
post-loop validation branches
```

Expected current failure:

```text
[ERROR] ❌ MIR compilation error: [plan/freeze:unsupported] generic_loop_v1: cannot build recipe for body
```

## Evidence

Gate entry:

```text
case_id=selfhost_scanner_multi_exit_min
allowed_rc=1
planner_tag=[phase132/gate] StepTree root for 'Main.scan_string_body_end/2'
```

Command:

```bash
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only selfhost_scanner_multi_exit_min
```

Result:

```text
[PASS] phase29bq_fast_gate_cases:selfhost_scanner_multi_exit_min
[PASS] phase29bq_fast_gate_vm: PASS (mode=selfhost_scanner_multi_exit_min)
```

This is a PASS of the expected-fail contract, not a compiler acceptance pass.

## First Owner

```text
owner=generic_loop_v1_recipe_body
failure_mode=plan/freeze:unsupported
first_error=generic_loop_v1: cannot build recipe for body
```

The next implementation row should add one Recipe/CorePlan acceptance shape for
this body form. It must not special-case `scan_string_body_end`, `read_number`,
or json_native method names.

## Next Task

```text
COREPLAN-GENERIC-LOOP-MULTI-EXIT-RECIPE-001
```

Target shape:

```text
loop(cond) body:
  if value_condition:
    marker update
    cursor update
    break
  else:
    if error_condition:
      return error_value
    else:
      if error_condition:
        return error_value
      tail step

post-loop:
  marker validation
  additional validation
  return value
```

## Stop Lines

```text
do not add method-name branches
do not widen all generic loops
do not mix continue support into this row
do not change json_native source to avoid compiler acceptance
do not remove the expected-fail fixture until implementation flips it green
```

## Report

```text
output_contract=coreplan-real-shape-fixture-multi-exit-v0
fixture_added=1
gate_case_added=1
expected_fail_contract_green=1
new_recipe_acceptance_required=1
selected_owner=generic_loop_v1_recipe_body
next_task=COREPLAN-GENERIC-LOOP-MULTI-EXIT-RECIPE-001
summary=ok
```
