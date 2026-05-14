---
Status: SSOT
Date: 2026-05-14
Scope: ARRAY-002C unsupported Array inference fail-fast diagnostics.
Related:
  - docs/development/current/main/design/typed-array-element-checks-ssot.md
  - docs/development/current/main/design/array-result-option-canonical-surface-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-317-ARRAY-002C-UNSUPPORTED-ARRAY-INFERENCE-FAILFAST.md
---

# Typed Array Inference Fail-Fast SSOT

## Decision

Decision: accepted.

Stage1 does not infer ordinary `Array<T>` element types in the MVP. Unsupported
inference surfaces fail-fast instead of silently choosing dynamic ArrayBox or a
placeholder generic element.

## Owned diagnostics

- `local xs = []` remains rejected by typed-context literal rules.
- `local xs: Array<T> = []` fails with `[array/inference]` because `T` is an
  unresolved element type in Stage1 Program JSON v0.
- Direct mixed literals under a concrete `Array<T>` fail with
  `[array/element-type]`.

## Non-goals

- No homogeneous literal inference.
- No generic substitution for `Array<T>` locals.
- No method-return type inference.
- No PackedArray fallback or backend route proof.
