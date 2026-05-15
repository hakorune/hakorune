# 293x-407 CLEAN-STAGE1-LOWERING-002 Statement Lowering Split

Status: landed
Date: 2026-05-15

## Decision

Split the large `statement_to_json_v0` dispatcher into statement-family helpers
without changing Program(JSON v0) output.

This is a BoxShape cleanup row. The Stage1 lowering entry remains in
`lowering.rs`; this row only separates local, assignment, print, return, branch,
loop, range, throw, try, and expression-statement responsibilities.

## TODO

- [x] Keep `statement_to_json_v0_many` as the multi-output statement owner.
- [x] Move `Local` expansion into a helper.
- [x] Keep `statement_to_json_v0` as the thin single-output dispatcher.
- [x] Move single-output statement families into helpers.
- [x] Add a guard that rejects helper removal and runs representative Stage1
  Program(JSON v0) tests.

## Scope

- Add behavior-preserving helper functions under the existing Stage1 lowering
  module.
- Keep all JSON tags, local type tracking, expected-type checks, and scoped
  clone behavior unchanged.
- Keep `expression_to_json_v0` and expression helper behavior unchanged.

## Stop Lines

- Do not change Program(JSON v0) schema.
- Do not change parser output or AST shapes.
- Do not move the Stage1 lowering module yet.
- Do not mix this row with MIMAP behavior.

## Required Evidence

```text
bash tools/checks/k2_wide_clean_stage1_lowering_stmt_split_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Implementation

- Added `local_statement_to_json_v0_many` for multi-output local lowering.
- Kept `statement_to_json_v0` as a thin single-output dispatcher.
- Added named helpers for assignment, print, return, if, loop, LoopRange,
  throw, try/catch, and expression statements.
- Added a representative Stage1 Program(JSON v0) statement-family fixture.

## Evidence

```text
bash tools/checks/k2_wide_clean_stage1_lowering_stmt_split_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
