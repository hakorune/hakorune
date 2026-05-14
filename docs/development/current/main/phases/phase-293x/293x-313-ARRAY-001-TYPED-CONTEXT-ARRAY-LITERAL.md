# 293x-313 ARRAY-001 typed context array literal

Status: complete

## Decision

Decision: accepted.

Array literals require an explicit local typed context. `Array<T>` literals may
lower through the JSON v0 bridge to ordinary `ArrayBox` construction. `PackedArray<T>`
literals must fail-fast until a dedicated packed literal/backend row exists.

## Scope

- Parse `[]` and `[expr, ...]` without a legacy sugar env gate.
- Require `local name: Array<T> = [...]` for Program JSON v0 array literal
  lowering.
- Emit `ArrayLiteral` Program JSON v0 metadata with `declared_type`,
  `element_type`, and lowered elements.
- Lower JSON v0 `ArrayLiteral` to `ArrayBox` plus `push` calls.
- Reject untyped array literals.
- Reject `PackedArray<T>` literals without falling back to `Array<T>`/`ArrayBox`.

## Non-goals

- No array literal type inference.
- No array element type checker.
- No typed `Array<T>` method semantics beyond literal construction.
- No `PackedArray<T>` runtime/backend auto-use.

## Guard

- `tools/checks/k2_wide_array_typed_context_literal_guard.sh`

## Local guard

- `bash tools/checks/k2_wide_array_typed_context_literal_guard.sh` passed locally.
