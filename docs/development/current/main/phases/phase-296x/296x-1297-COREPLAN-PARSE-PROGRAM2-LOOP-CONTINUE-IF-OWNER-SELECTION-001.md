---
Status: Active
Decision: accepted
Date: 2026-06-19
Scope: Select the route owner for
  phase29bq_selfhost_blocker_parse_program2_loop_continue_if_min after the
  loop if-return-local owner row.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1296-COREPLAN-PARSE-PROGRAM2-LOOP-IF-RETURN-LOCAL-OWNER-SELECTION-001.md
---

# COREPLAN-PARSE-PROGRAM2-LOOP-CONTINUE-IF-OWNER-SELECTION

## Decision

`phase29bq_selfhost_blocker_parse_program2_loop_continue_if_min` is already
semantically green.

The old gate expected a stronger historical planner-first tag:

```text
[joinir/planner_first rule=LoopCondBreak] label=LoopExitIfBreakContinue
```

The current stable tag emitted by the accepted route is:

```text
[flowbox/adopt box_kind=Loop features=break,continue,nested_loop via=shadow]
```

The fixture includes a nested loop and a conditional continue branch. It
produces the expected output `1` with rc `0`. There is no semantic failure and
no evidence that forcing `loop_cond_break_continue` would make the compiler
cleaner. Therefore this row updates the gate expectation to the stable adoption
tag and leaves compiler code unchanged.

## Implementation

Updated route expectations only:

```text
tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv
tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv
```

No compiler code changed in this row.

## Evidence

Focused gate after the update:

```bash
NYASH_BIN=target/debug/hakorune \
  bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh \
  --only selfhost_parse_program2_loop_continue_if_min
```

Result:

```text
pass
```

Full phase gate after the update progresses to the next blocker:

```text
phase29bq_selfhost_blocker_parse_program2_guard_prog_numeric_parse_loop_min
failure=missing planner-first LoopCondBreak tag
observed stable tag=[flowbox/adopt box_kind=Loop features=break via=shadow]
stdout=5
rc=0
```

## Stop Lines

```text
do not route-change only to satisfy historical tags
do not broaden LoopCondBreak ownership without a semantic failure
do not treat planner_first labels as semantic truth when stable flowbox adoption proves the active route
```

## Next

Continue the fail-closed phase29bq gate and select the next real blocker.

```text
next_task=COREPLAN-PARSE-PROGRAM2-GUARD-PROG-NUMERIC-PARSE-LOOP-OWNER-SELECTION-001
```

## Report

```text
output_contract=coreplan-parse-program2-loop-continue-if-owner-selection-v0
route_expectation_updated=flowbox_adoption_tag
compiler_code_changed=0
semantic_output_already_green=1
focused_gate_green=1
full_phase_gate_progressed_to_next_blocker=phase29bq_selfhost_blocker_parse_program2_guard_prog_numeric_parse_loop_min
summary=ok
```
