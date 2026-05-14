---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: GEN-001 Stage0 generic type annotation metadata capsule.
Related:
  - docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md
  - docs/development/current/main/design/stage0-stage1-feature-responsibility-split-ssot.md
  - docs/development/current/main/design/language-feature-implementation-order-ssot.md
---

# GEN-001 Generic Type Annotation Metadata Capsule SSOT

## Decision

Generic type references are accepted as Stage0 metadata in declaration type
positions.

The canonical type-reference text shape is:

```text
TYPE_REF := IDENT ('.' IDENT)* ('<' TYPE_REF (',' TYPE_REF)* '>')? ('[' ']')*
```

## Canonical examples

```hako
type PageList = Array<PageId>

record MetaStore<T> {
    metas: PackedArray<T>
}

box Store {
    metas: PackedArray<Meta<PageId>>
    weak view: Span<PageId>
}

process(items: Array<PageId>): Result<PageId, Error> {
    return PageId(0)
}
```

## Owner split

Stage0 owns:

```text
parse generic type annotation text
preserve type parameters on box/record/enum declarations
preserve type references on fields, params, returns, aliases, brands, and enum payloads
transport metadata through AST, AST JSON, and Program JSON v0
```

Stage0 does not own:

```text
generic arity checking
constraint solving
where clauses
Array<T> semantics
PackedArray<T> eligibility/planner
Span<T> no-escape semantics
backend fallback policy
unknown type resolution
```

Stage1 owns:

```text
generic arity diagnostics
substitution metadata
container semantics
PackedArray fail-fast eligibility
Span/view semantics
```

## Stop lines

```text
no generic solver in Stage0
no PackedArray planner in Stage0
no Array<T> runtime behavior in GEN-001
no silent PackedArray fallback
no backend capability decision
```

## Retire condition

Retire this capsule when the Stage1/selfhost parser and metadata transport emit
the same generic type-reference shape without relying on Rust parser ownership.
