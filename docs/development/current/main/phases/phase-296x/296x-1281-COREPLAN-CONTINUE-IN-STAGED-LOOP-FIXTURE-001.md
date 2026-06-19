---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Capture the next continue-in-staged-loop compiler owner after multi-exit acceptance.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1280-COREPLAN-GENERIC-LOOP-MULTI-EXIT-RECIPE-001.md
  - apps/tests/phase29bq_selfhost_blocker_read_number_continue_staged_min.hako
  - tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv
---

# COREPLAN-CONTINUE-IN-STAGED-LOOP-FIXTURE-001

## Decision

The next real compiler owner is not a missing `continue` syntax route. Existing
continue fixtures are broadly green.

The failing shape is narrower:

```text
loop(cond):
  local classification
  if separator:
    cursor = cursor + 1
    continue
  if non_digit:
    break
  seen = 1
  accumulator = accumulator + 1
  cursor = cursor + 1
```

The critical detail is a partial loop-carried update:

```text
continue branch updates cursor
continue branch does not update accumulator/seen
fallthrough branch updates accumulator/seen
```

## Captured Fixture

```text
apps/tests/phase29bq_selfhost_blocker_read_number_continue_staged_min.hako
```

Expected current failure:

```text
[freeze:contract][mir/verify:dominator_violation]
fn=Main.count_digits_skip_sep/2
```

The gate pins this as an expected-fail contract:

```text
case_id=selfhost_read_number_continue_staged_min
allowed_rc=1
planner_tag=[flowbox/adopt box_kind=Loop features=break,continue via=shadow]
```

## First Owner

```text
owner=loop_cond_break_continue_partial_carrier_update
failure_mode=mir/verify:dominator_violation
route=flowbox/adopt break,continue
```

The suspected seam is the continue PHI argument path for loop carriers that are
not updated on the continue branch but are updated later on the fallthrough
path.

## Next Task

```text
COREPLAN-CONTINUE-PARTIAL-CARRIER-PHI-001
```

This next row should stay inside the loop-cond break/continue PHI or
conditional-update seam. It must not add method-name branches or change the
fixture source to avoid the compiler issue.

## Stop Lines

```text
do not special-case count_digits_skip_sep
do not remove the accumulator/seen carrier to make the fixture green
do not mix nested break/continue support into this row
do not change json_native source to avoid compiler acceptance
```

## Report

```text
output_contract=coreplan-continue-in-staged-loop-fixture-v0
fixture_added=1
expected_fail_contract_green=1
selected_owner=loop_cond_break_continue_partial_carrier_update
next_task=COREPLAN-CONTINUE-PARTIAL-CARRIER-PHI-001
summary=ok
```
