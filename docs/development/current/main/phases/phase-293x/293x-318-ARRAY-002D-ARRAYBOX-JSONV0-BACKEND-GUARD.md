# 293x-318 ARRAY-002D ArrayBox JSON v0/backend guard

Status: complete

## Decision

Decision: accepted.

The ordinary `Array<T>` literal route is now guarded as JSON v0 `ArrayLiteral`
into the existing ArrayBox values backend path, while `PackedArray<T>` still
fails fast instead of falling back to ArrayBox.

## Scope

- Guard Program JSON v0 `ArrayLiteral` output for typed `Array<T>` literals.
- Guard JSON v0 bridge ownership of `ArrayLiteral`.
- Guard ArrayBox values lowering route.
- Guard `PackedArray<T> = []` no-fallback diagnostics.

## Non-goals

- No new lowering node.
- No PackedArray literal/backend support.
- No storage planner change.
- No element type inference.

## Guard

- `tools/checks/k2_wide_arraybox_json_v0_backend_guard.sh`

## Local guard

- `bash tools/checks/k2_wide_arraybox_json_v0_backend_guard.sh` passed locally.
