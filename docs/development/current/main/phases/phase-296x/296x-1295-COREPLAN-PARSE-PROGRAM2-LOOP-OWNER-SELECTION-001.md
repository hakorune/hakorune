---
Status: Active
Decision: accepted
Date: 2026-06-19
Scope: Select the route owner for
  phase29bq_selfhost_blocker_parse_program2_loop_min after the skip-ws blocker.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1294-COREPLAN-PARSE-STMT-SKIPWS-OWNER-SELECTION-001.md
---

# COREPLAN-PARSE-PROGRAM2-LOOP-OWNER-SELECTION

## Decision

`phase29bq_selfhost_blocker_parse_program2_loop_min` is already semantically
green through `generic_loop_v1`.

Debug observation shows the current semantic route:

```text
[plan/trace:loop_legacy_selected] route=generic_loop_v1
```

The old gate expected a stronger historical planner-first tag:

```text
[joinir/planner_first rule=LoopCondBreak] label=LoopExitIfBreakContinue
```

Normal fast-gate runs do not enable the debug-only `loop_legacy_selected`
trace, so the stable acceptance tag is the flowbox adoption tag:

```text
[flowbox/adopt box_kind=Loop features=break,nested_loop via=shadow]
```

Output is correct (`1`, rc `0`). There is no evidence that forcing
`loop_cond_break_continue` would make the compiler cleaner; it would only
preserve a stale historical route label. Therefore the selected route remains
`generic_loop_v1`, and the gate expectation is updated to the stable adoption
tag.

## Implementation

Updated route expectations only:

```text
tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv
tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv
```

No compiler code changed in this row.

## Evidence

Observed before the update:

```text
[plan/trace:loop_legacy_observer] decision=allow:generic_loop_v1
[plan/trace:loop_legacy_selected] route=generic_loop_v1
stdout=1
rc=0
```

Focused gate after the update:

```bash
NYASH_BIN=target/debug/hakorune \
  bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh \
  --only selfhost_parse_program2_loop_min
```

Result:

```text
pass
```

Full phase gate after the update progresses to the next blocker:

```text
phase29bq_selfhost_blocker_parse_program2_loop_if_return_local_min
failure=missing planner-first LoopCondBreak tag
observed stable tag=[flowbox/adopt box_kind=Loop features=return via=shadow]
```

## Stop Lines

```text
do not route-change only to satisfy historical tags
do not treat planner_first labels as semantic truth when actual selected route is available
do not broaden LoopCondBreak ownership without a semantic failure
```

## Next

Continue the fail-closed phase29bq gate and select the next real blocker.

```text
next_task=COREPLAN-PARSE-PROGRAM2-LOOP-IF-RETURN-LOCAL-OWNER-SELECTION-001
```

## Report

```text
output_contract=coreplan-parse-program2-loop-owner-selection-v0
selected_owner=generic_loop_v1
route_expectation_updated=flowbox_adoption_tag
compiler_code_changed=0
semantic_output_already_green=1
full_phase_gate_progressed_to_next_blocker=phase29bq_selfhost_blocker_parse_program2_loop_if_return_local_min
summary=ok
```
