---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: ARRAY-001 typed-context array literal lowering.
Related:
  - docs/development/current/main/design/array-result-option-canonical-surface-ssot.md
  - docs/development/current/main/design/local-type-annotation-metadata-capsule-ssot.md
  - docs/reference/language/EBNF.md
---

# Typed Array Literal Context SSOT

## Decision

`[]` and non-empty array literals are accepted only where Stage1 has an
explicit local typed context.

Canonical:

```hako
local ids: Array<PageId> = []
local values: Array<i64> = [1, 2, 3]
```

Rejected:

```hako
local ids = []
```

Rejected until a later PackedArray literal row:

```hako
local metas: PackedArray<Meta> = []
```

## Stage Split

Stage0 owns:

```text
array literal parse shape
local declared type metadata transport
```

Stage0 does not own:

```text
type inference
array element type checking
PackedArray planning
backend fallback policy
```

Stage1 owns:

```text
local typed-context requirement
Array<T> literal Program JSON v0 shape
Array<T> literal JSON v0 bridge lowering to ArrayBox construction
PackedArray<T> literal fail-fast with no Array<T> fallback
```

## Program JSON v0 Shape

`Array<T>` local typed context lowers the literal as metadata-rich expression:

```json
{
  "type": "ArrayLiteral",
  "declared_type": "Array<PageId>",
  "element_type": "PageId",
  "elements": []
}
```

The JSON v0 bridge lowers this expression to a new `ArrayBox` followed by
`push` calls for each element.

## Stop Lines

```text
no untyped [] inference
no element type checker in ARRAY-001
no PackedArray<T> literal fallback to ArrayBox
no Array<T> / PackedArray<T> subtype relation
no array iteration syntax
```

## Retire Condition

This Stage1 bridge row can retire when the selfhost parser and Stage1 typed
collection owner produce the same `Array<T>` literal shape and fail-fast
contracts without Rust-side special ownership.
