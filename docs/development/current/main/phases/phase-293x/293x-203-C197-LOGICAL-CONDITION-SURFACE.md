# 293x-203: C197 Logical Condition Surface Hardening

Status: Complete

This row fixes the first proof/application surface row after M190. It does not
add a new language operator. It locks the existing ordinary `&&` / `||` route as
the accepted readable source shape for normal control-flow conditions.

## Decision

`&&` / `||` remain ordinary short-circuit boolean operators.

Parenthesized multiline conditions are accepted in normal `if`, `loop`, and
expression contexts. Leading logical operators on continuation lines are part of
the accepted readability style.

`check "name" { "label": expr }` remains a later C198 proof-list surface and is
not an alias for this row.

## Implementation

- Added `apps/logical-condition-surface-proof/`.
- Added parser regression coverage in
  `src/tests/parser_logical_condition_surface.rs`.
- Added `tools/checks/k2_wide_logical_condition_surface_guard.sh`.
- Updated the language EBNF notes with the C197 accepted decision.

## Acceptance

The proof app validates:

- parenthesized multiline `&&` / `||` source shape
- `if` and `loop` condition usage
- short-circuit RHS side-effect preservation
- no `check` block dependency

## Stop Line

- No eager proof-list semantics.
- No `all(...)` macro or variadic condition helper.
- No allocator-specific condition DSL.
- No backend route selector or `.inc` app matcher.

## Next

`C198 check block surface` may add eager labeled proof-list syntax. It must
lower independently from the ordinary `&&` / `||` short-circuit route fixed by
this row.
