---
Status: SSOT
Date: 2026-05-14
Scope: ARRAY-002B typed local Array<T> direct element checks.
Related:
  - docs/development/current/main/design/typed-array-method-contract-ssot.md
  - docs/development/current/main/design/array-result-option-canonical-surface-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-316-ARRAY-002B-TYPED-ARRAY-ELEMENT-CHECKS.md
---

# Typed Array Element Checks SSOT

## Decision

Decision: accepted.

Stage1 validates direct element expressions for locals declared as `Array<T>`
when the direct expression type is known without general inference.

## Owned checks

```hako
local ids: Array<PageId> = [PageId(1)]
ids.push(PageId(2))
ids.set(0, PageId(3))
```

The row checks:

- typed `Array<T>` literal elements
- `push(value)` direct values
- `set(index, value)` direct values

Known direct expression types include:

- scalar literals such as integer, string, bool, and float
- brand constructors such as `PageId(1)`
- record literals and tracked record locals
- known enum constructors when the enum owner is known

## Fail-fast diagnostics

Mismatches fail with `[array/element-type]`.

Example:

```hako
brand PageId: i64
local ids: Array<PageId> = [1]
```

The raw integer does not satisfy `PageId`; use `PageId(1)`.

## Non-goals

- No general local type inference.
- No method-return type inference.
- No generic substitution or `where` constraints.
- No untyped `local x = []` acceptance.
- No PackedArray fallback or backend route proof.
