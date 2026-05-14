# 293x-317 ARRAY-002C unsupported Array inference fail-fast

Status: complete

## Decision

Decision: accepted.

Unsupported `Array<T>` inference surfaces now fail-fast in Stage1. This keeps
ordinary typed Array semantics explicit and avoids silently creating dynamic or
placeholder element contexts.

## Scope

- Reject unresolved local `Array<T>` element contexts such as `Array<T>`.
- Keep untyped `local xs = []` rejected by ARRAY-001 typed-context rules.
- Keep direct mixed literal mismatches rejected by ARRAY-002B element checks.
- Tag unresolved element diagnostics with `[array/inference]`.

## Non-goals

- No homogeneous literal inference.
- No generic substitution.
- No ArrayBox backend route proof.
- No PackedArray fallback.

## Guard

- `tools/checks/k2_wide_array_inference_failfast_guard.sh`

## Local guard

- `bash tools/checks/k2_wide_array_inference_failfast_guard.sh` passed locally.
