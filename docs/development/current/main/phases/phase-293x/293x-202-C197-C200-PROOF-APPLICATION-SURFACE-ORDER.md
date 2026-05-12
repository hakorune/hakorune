# 293x-202: C197-C200 Proof/Application Surface Order

Status: Complete

This card locks the readable task order for the proof/application syntax rows
that follow the allocator API rows. It does not add syntax or change allocator
behavior by itself.

## Decision

Support both ordinary boolean chains and proof check blocks.

- Ordinary `&&` / `||` remain normal expression/control-flow operators with
  short-circuit semantics.
- `check "name" { "label": expr }` blocks are proof-list expressions with eager
  item evaluation.
- They are not aliases. Use boolean chains for production control flow; use
  `check` blocks when a proof app needs named assertions and all failures should
  remain observable.

## Row Order

### C197 logical condition surface hardening

Purpose:
make ordinary `&&` / `||` chains and parenthesized multiline conditions
reliable/readable in normal code.

Acceptance:
source examples parse and lower through the existing short-circuit route, and
RHS short-circuit behavior is preserved.

Stop line:
no eager proof-list semantics, no allocator DSL, no route selector.

### C198 check block surface

Status: Complete.

Purpose:
add `check "name" { "label": expr }` as a proof-list expression.

Acceptance:
every item is evaluated left-to-right, the result is scalar pass/fail, and
labels are source-level proof metadata.

Stop line:
no variadic `all(...)`, no macro expansion, no short-circuit behavior.

### C199 compound assignment surface

Status: Complete.

Purpose:
promote `+=`-style sugar where it lowers to the existing assignment form.

Acceptance:
field/local/index assignment sugar keeps the same value semantics as canonical
assignment.

Stop line:
no hidden overflow policy, no allocator-specific meaning.

### C200 guard else surface

Status: Complete.

Purpose:
add early-return guard syntax that lowers to `if !(cond) { ... }`.

Acceptance:
guard lowering is equivalent to explicit negative `if` and keeps Fail-Fast code
compact.

Stop line:
no exceptions, no fallback semantics.

## Why This Split

Proof apps and production logic need different evaluation contracts.

`&&` / `||` must short-circuit because normal control flow depends on not
evaluating unnecessary or unsafe RHS expressions.

`check` must not short-circuit because proof apps should report a full list of
failed facts instead of hiding everything after the first failure.

Keeping those contracts separate avoids turning proof syntax into a second
boolean language and avoids using boolean chains as a poor proof-reporting DSL.

## Implementation Rules

- Each row must update `docs/reference/**` with an explicit `Decision:` before
  parser/lowering changes land.
- Each row gets its own fixture or proof app plus a focused guard.
- Do not fold these rows into allocator algorithm rows.
- Unsupported backend behavior must be explicit. Do not silently fall back to a
  VM-only success path.

## Next

`C197 logical condition surface hardening`, `C198 check block surface`, `C199
compound assignment surface`, and `C200 guard else surface` are complete. Keep
future proof/application syntax rows separate from allocator algorithm rows.
