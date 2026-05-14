---
Status: SSOT
Date: 2026-05-14
Scope: GEN-002 Stage1 generic type argument arity checking.
Related:
  - docs/reference/language/EBNF.md
  - docs/development/current/main/design/generic-type-annotation-metadata-capsule-ssot.md
  - src/stage1/program_json_v0/generic_arity_checker.rs
  - tools/checks/k2_wide_generic_arity_checker_guard.sh
---

# GEN-002 Generic Arity Checker SSOT

## Decision

Stage1 checks generic type argument counts for known generic type names carried
by GEN-001 type-reference metadata.

The checker is intentionally narrow. It validates arity only; it does not
resolve unknown type names or implement generic substitution.

## Owner

Code owner:

```text
src/stage1/program_json_v0/generic_arity_checker.rs
```

Route owner:

```text
src/stage1/program_json_v0/authority.rs
```

The checker runs after strict/relaxed source parsing and before Program JSON v0
lowering.

## Known Generic Arity

Built-in/prelude surfaces:

| Type | Arity |
| --- | ---: |
| `Array` | 1 |
| `PackedArray` | 1 |
| `Span` | 1 |
| `Option` | 1 |
| `Result` | 2 |

Same-program declarations:

- `box Name<T...>`
- `record Name<T...>`
- `enum Name<T...>`

## Checked Type Metadata Slots

- box/record field declared types
- function/method/constructor parameter declared types
- function/method/constructor return types
- enum payload and record-payload field types
- brand underlying types
- type alias target types

## Fail-Fast

Mismatch tag:

```text
[generic/arity] type=<Name> expected=<n> actual=<m>
```

Examples:

```hako
ids: Array<PageId, BlockId>
```

fails because `Array` expects 1 argument.

```hako
record Meta<T> {
    value: T
}

metas: PackedArray<Meta<PageId, BlockId>>
```

fails because `Meta` expects 1 argument.

## Stop Line

GEN-002 does not add:

- type existence checking for unknown names
- alias expansion
- constraint solving
- `where` clauses
- generic substitution or specialization
- monomorphization
- `Array<T>` runtime semantics
- `PackedArray<T>` eligibility or backend lowering
- `Span<T>` no-escape semantics

Unknown type names remain metadata unless a later Stage1 row owns existence
checking.

## Guard

```bash
bash tools/checks/k2_wide_generic_arity_checker_guard.sh
```
