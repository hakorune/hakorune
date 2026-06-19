---
Status: Active
Decision: accepted
Date: 2026-06-19
Scope: Select the current route expectation for the retired scan_v0 comma/close
  and range scan fixtures.
Related:
  - docs/development/current/main/phases/phase-293x/293x-1013-COREPLAN-E1-005-SCAN-V0-RETIRE.md
  - docs/development/current/main/phases/phase-296x/296x-1301-COREPLAN-SCAN-METHODS-NESTED-LOOP-STATE-MACHINE-STEP-PLACEMENT-OWNER-SELECTION-001.md
---

# COREPLAN-SCAN-LOOP-COMMA-CLOSE-OWNER-SELECTION

## Decision

The current blocker was not a semantic compiler failure.

Both retired `loop_scan_v0` fixtures are semantic green:

```text
phase29bq_joinir_scan_loop_comma_close_min
phase29bq_joinir_scan_loop_range_lte_minus1_min
```

They produce the expected stdout `0` and rc `0`. The old gate expected:

```text
[joinir/planner_first rule=LoopCondBreak] label=LoopExitIfBreakContinue
```

The current stable observed route is:

```text
[joinir/planner_first rule=LoopSimpleWhile] label=LoopSimpleWhile
[flowbox/adopt box_kind=Loop features=break,continue via=shadow]
```

`loop_scan_v0` remains retired. The focused proof now records the actual
coverage route rather than preserving the historical replacement-owner wording.

## Implementation

Updated route expectations only:

```text
tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv
```

Updated the retired scan-v0 guard wording so it no longer claims
`loop_cond_break_continue` as the current replacement owner:

```text
tools/checks/coreplan_scan_v0_retire_guard.sh
docs/development/current/main/workstreams/compiler-foundation-current.md
```

No compiler code changed in this row.

## Evidence

Focused gates:

```bash
NYASH_BIN=target/debug/hakorune \
  bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh \
  --only scan_loop_v0_comma_close_min

NYASH_BIN=target/debug/hakorune \
  bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh \
  --only scan_loop_v0_lte_n_minus1_min
```

Result:

```text
pass
pass
```

Retire guard:

```bash
bash tools/checks/coreplan_scan_v0_retire_guard.sh
```

Result:

```text
pass
```

## Stop Lines

```text
do not reintroduce loop_scan_v0
do not broaden LoopCondBreak to satisfy historical tags
do not treat planner-first labels as semantic truth when stdout/rc and stable adoption are green
```

## Next

Continue the phase29bq gate from the next visible blocker after the retired
scan_v0 fixture pair:

```text
next_task=COREPLAN-SCAN-METHODS-LOOP-AMBIGUOUS-LOOPVAR-OWNER-SELECTION-001
fixture=phase29bq_selfhost_blocker_scan_methods_loop_min
failure=[plan/freeze:ambiguous] multiple loop_var candidates matched
observed_function=ParserBox.static_const_bitand/2
```

## Report

```text
output_contract=coreplan-scan-loop-comma-close-owner-selection-v0
compiler_code_changed=0
loop_scan_v0_reintroduced=0
route_expectation_updated=generic_loop_v1_plus_flowbox_adoption
focused_comma_close_gate_green=1
focused_lte_minus1_gate_green=1
scan_v0_retire_guard_green=1
full_phase_gate_progressed_to_next_blocker=phase29bq_selfhost_blocker_scan_methods_loop_min
summary=ok
```
