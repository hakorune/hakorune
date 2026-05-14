# 293x-316 ARRAY-002B typed local Array element checks

Status: complete

## Decision

Decision: accepted.

Stage1 now validates direct element expressions for typed local `Array<T>`
contexts. This extends ARRAY-002A's method-surface contract without adding
Array inference or backend route semantics.

## Scope

- Validate typed array literal elements when the direct element type is known.
- Validate `push(value)` direct values for tracked `Array<T>` locals.
- Validate `set(index, value)` direct values for tracked `Array<T>` locals.
- Preserve unknown variables and method-return values for later inference rows.
- Emit `[array/element-type]` diagnostics for known mismatches.

## Non-goals

- No general type inference.
- No method-return type propagation.
- No generic substitution or constraints.
- No `local x = []` acceptance.
- No PackedArray fallback or backend proof.

## Guard

- `tools/checks/k2_wide_array_typed_element_checks_guard.sh`

## Local guard

- `bash tools/checks/k2_wide_array_typed_element_checks_guard.sh` passed locally.
