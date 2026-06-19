---
Status: Active
Decision: accepted
Date: 2026-06-19
Scope: Select and fix the owner for the existing
  phase29bq_selfhost_blocker_parse_stmt_skipws_min gate debt.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1293-COREPLAN-LOOP-MULTIDELTA-OWNER-SELECTION-001.md
---

# COREPLAN-PARSE-STMT-SKIPWS-OWNER-SELECTION

## Decision

`phase29bq_selfhost_blocker_parse_stmt_skipws_min` is an `if ... else {
break }` loop shape. The correct owner is `loop_cond_break_continue` with
`LoopCondBreakAcceptKind::ElseOnlyBreak`.

`generic_loop_v1` is not a safe owner for this shape. It selected the route and
produced `2` instead of the expected `1`, because it treated the loop step as
reachable after the else-break exit.

## Implementation

The facts builder now preserves already-built `loop_cond_break_continue` facts
for `ElseOnlyBreak`, instead of nulling them when `generic_loop_v1` also has a
recipe hint.

The earlier multidelta rule remains narrow:

```text
ConditionalUpdate -> keep LoopCondBreak
ElseOnlyBreak -> keep LoopCondBreak
multiple continue branches with assignment/local prelude -> keep LoopCondBreak
otherwise keep the previous generic-v1 hint behavior
```

This keeps `phase29bq_loop_cond_continue_if_else_fallthrough_min` on its
existing owner while moving only the else-break skip-ws shape to LoopCondBreak.

## Evidence

Focused gates:

```bash
NYASH_BIN=target/debug/hakorune \
  bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh \
  --only selfhost_parse_stmt_skipws_min

NYASH_BIN=target/debug/hakorune \
  bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh \
  --only loop_continue_only_multidelta_min
```

Result:

```text
both pass
```

Full phase gate progress:

```bash
NYASH_BIN=target/debug/hakorune \
  bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh
```

Result:

```text
phase29bq_selfhost_blocker_parse_stmt_skipws_min passed
next blocker=phase29bq_selfhost_blocker_parse_program2_loop_min
failure=missing planner-first LoopCondBreak tag
actual route=LoopSimpleWhile
```

## Stop Lines

```text
do not broaden LoopCondBreak retention to all continue-if shapes
do not delete registry suppression in this row
do not update fixture expectations from route mismatch alone
do not treat full gate as green; it now stops later
```

## Next

```text
next_task=COREPLAN-PARSE-PROGRAM2-LOOP-OWNER-SELECTION-001
```

## Report

```text
output_contract=coreplan-parse-stmt-skipws-owner-selection-v0
selected_owner=loop_cond_break_continue
selected_accept_kind=ElseOnlyBreak
generic_loop_v1_rejected_as_owner=1
focused_skipws_gate_green=1
multidelta_gate_still_green=1
full_phase_gate_progressed_to_next_blocker=phase29bq_selfhost_blocker_parse_program2_loop_min
summary=ok
```
