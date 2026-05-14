---
Status: SSOT
Date: 2026-05-14
Scope: Canonical surface for `Array<T>`, `PackedArray<T>`, `Result<T,E>`, `Option<T>`, and enum variants.
Related:
  - docs/reference/language/EBNF.md
  - docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md
  - docs/development/current/main/design/generic-arity-checker-ssot.md
  - docs/development/current/main/design/enum-sum-and-generic-surface-ssot.md
---

# Array / Result / Option Canonical Surface SSOT

## Decision

Hakorune keeps collection, absence, failure, and enum-state syntax explicit and
small:

```text
Collection:
  Array<T>
  PackedArray<T>

Absence:
  Option<T>

Failure:
  Result<T, E>

Enum variant:
  Type::Variant

Control:
  guard
  match
```

Do not add parallel vocabulary such as `Vec<T>`, `List<T>`, `try`, `throw`, or
`?` for this surface.

## Array

Canonical:

```hako
local ids: Array<PageId> = []
ids.push(PageId(1))
local first = ids.get(0)
ids.set(0, PageId(2))
local n = ids.length()
```

Rules:

- `Array<T>` is the canonical ordinary typed collection spelling.
- `T[]` is non-canonical compatibility / low-level static-table spelling.
- `Vec<T>`, `List<T>`, `array<T>`, and `new Array<T>()` are not canonical.
- `[]` needs a typed context in canonical code.

Canonical empty array:

```hako
local ids: Array<PageId> = []
```

Non-canonical / fail-fast until a later inference row:

```hako
local ids = []
```

Non-empty literal inference is also deferred. Prefer:

```hako
local ids: Array<i64> = [1, 2, 3]
```

## PackedArray

Canonical:

```hako
local metas: PackedArray<Meta> = []
```

Meaning:

```text
Array<T>:
  semantic typed collection; compiler may choose storage.

PackedArray<T>:
  requests packed residence; unsupported cases fail-fast.
```

MVP rules:

- `PackedArray<T>` is not a silent alias for `Array<T>`.
- `PackedArray<T>` and `Array<T>` are separate types in MVP.
- No implicit subtype conversion from `PackedArray<T>` to `Array<T>`.
- No silent fallback to boxed storage when packed residence cannot be proven.

Initial allowed target:

```text
T is a record with supported scalar fields and no unsupported escape/materialization need.
```

Initial reject targets:

```text
ordinary box element
unresolved generic element
object/handle field when packed layout cannot support it
backend without packed lowering support
```

## Result and Option

Canonical public shape:

```hako
enum Option<T> {
    Some(T)
    None
}

enum Result<T, E> {
    Ok(T)
    Err(E)
}
```

No new exception-like family:

```text
no try
no throw
no ?
no nullable Result
```

Use `guard` and `match`:

```hako
allocate(size: Bytes): Result<Handle, AllocError> {
    guard size > 0 else {
        return Result::Err(AllocError::ZeroSize)
    }

    return Result::Ok(handle)
}
```

```hako
match result {
    Result::Ok(handle) => {
        return Result::Ok(handle)
    }
    Result::Err(reason) => {
        return Result::Err(reason)
    }
}
```

## Enum Variants

Canonical variant namespace uses `::`, not `.`:

```hako
Result::Ok(handle)
Result::Err(reason)
Option::Some(value)
Option::None
PageState::Active
AllocError::ZeroSize
```

Reason:

```text
.  = object field / method access
:: = type namespace / enum variant
```

Variant forms:

```hako
Enum::Unit
Enum::Tuple(value)
Enum::Record { field: value }
```

Non-canonical:

```hako
Result.Ok(handle)
Ok(handle)
```

Possible later sugar:

```hako
return .Ok(handle)
return .Err(reason)
```

This sugar requires an unambiguous expected type and is not part of the MVP.

## Expected Type Rule

Generic enum constructors require an expected type when type arguments cannot be
known locally.

Accepted:

```hako
allocate(size: Bytes): Result<Handle, AllocError> {
    return Result::Err(AllocError::ZeroSize)
}

local r: Result<Handle, AllocError> = Result::Err(AllocError::ZeroSize)
```

Rejected until a later inference row:

```hako
local r = Result::Err(AllocError::ZeroSize)
```

## Required Implementation Rows

| Row | Scope | Owner |
| --- | --- | --- |
| `ARRAY-RESULT-SSOT` | this docs-only canonical surface decision | docs/reference |
| `LOCALTYPE-001` | `local name: Type = expr` metadata capsule | Stage0 parse / transport |
| `ENUMVAR-001` | keep `Type::Variant` canonical and reject/avoid dot variants | Stage1 enum surface |
| `ARRAY-001` | typed-context array literals; `[]` requires expected type | Stage1 typed collection |
| `RESULT-001` | Result/Option prelude and explicit variant diagnostics | Stage1 enum/prelude |
| `PACKED-001` | PackedArray eligibility gate and fail-fast policy | Stage1 CorePlan |

## Stage Split

Stage0 may own:

```text
local type annotation parse and metadata transport
TYPE_REF metadata transport
qualified `Type::Variant` shape transport
array literal shape transport
```

Stage0 must not own:

```text
type inference
enum variant resolution
generic substitution
typed Array semantics
PackedArray eligibility
Result/Option prelude semantics
backend fallback policy
```

Stage1 owns:

```text
expected type propagation
generic enum constructor resolution
typed-context array literal lowering
Result/Option diagnostics
PackedArray eligibility and backend fail-fast
```

## Stop Line

This SSOT does not implement syntax or semantics. It fixes the canonical
surface and task split before the next implementation rows.
