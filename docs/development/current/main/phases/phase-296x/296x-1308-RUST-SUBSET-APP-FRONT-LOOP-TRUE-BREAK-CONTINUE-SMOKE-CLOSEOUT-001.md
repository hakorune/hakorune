# 296x-1308 RUST-SUBSET-APP-FRONT-LOOP-TRUE-BREAK-CONTINUE-SMOKE-CLOSEOUT-001

Status: closed
Date: 2026-06-19

## Purpose

Reopen the rust-subset-to-hako app-front smoke after `JsonParser.parse_array/0`
failed MIR JSON emission.

The blocker was a `loop(true)` body with:

- return-if for parse failure
- effectful continue branch (`log(); continue`)
- else-side break-or-return tree

## Decision

The owner is `loop_true_break_continue`, not `generic_loop_v1`.

`generic_loop_v1` remains a loop-variable route. It may derive loop variables
from an increment assignment under a boolean literal condition, but a
loop-variable-free parser loop must not be forced into generic-loop ownership.

## Implementation

- Let `loop_true_break_continue` use the recipe-first `ExitAllowed` body path
  even when a continue branch has effectful prelude statements.
- Add a focused unit test for the `parse_array`-class shape.
- Keep the generic-loop boolean-literal improvement narrow: only assignment
  increment candidates are used when the condition is a boolean literal and no
  condition-derived loop variable exists.

## Evidence

```bash
cargo test -q policy_exit_allowed_accepts_continue_prelude_with_else_exit_tree
cargo test -q loop_true_break_continue
cargo test -q loop_route
cargo test -q generic_loop
cargo check -q --lib
cargo build -q --release --bin hakorune
bash apps/rust-subset-to-hako/smoke.sh
```

Observed app-front result:

```text
summary=ok
```

## Non-Goals

- No `.hako` app workaround.
- No parser/source/function-name branch.
- No new named loop route.
- No route suppression deletion in this row.

## Next

Return to the rust-subset-to-hako app-front task sequence.

```text
next_blocker=RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-001
```
