---
Status: SSOT
Date: 2026-05-14
Scope: ARRAY-002D ArrayBox JSON v0 / backend guard.
Related:
  - docs/development/current/main/design/typed-array-literal-context-ssot.md
  - docs/development/current/main/design/typed-array-inference-failfast-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-318-ARRAY-002D-ARRAYBOX-JSONV0-BACKEND-GUARD.md
---

# ArrayBox JSON v0 Backend Guard SSOT

## Decision

Decision: accepted.

Ordinary `Array<T>` typed-context literals lower through the existing JSON v0
`ArrayLiteral` bridge into the ArrayBox values route. `PackedArray<T>` remains a
packed-residence request and must not silently fall back to ordinary ArrayBox.

## Guarded contract

- `Array<T> = []` / `Array<T> = [values...]` emits JSON v0 `ArrayLiteral`.
- JSON v0 bridge owns `ArrayLiteral` input and routes values through the ArrayBox
  array-values lowering path.
- `PackedArray<T> = []` fails before backend fallback.
- No new source-level syntax or type inference is introduced in this row.

## Non-goals

- No packed record backend lowering.
- No `PackedArray<T>` literal support.
- No ordinary Array storage planner policy.
- No generic substitution or inferred array element type.
