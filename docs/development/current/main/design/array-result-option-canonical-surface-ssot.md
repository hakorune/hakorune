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
- `ARRAY-002A` implements the canonical typed `Array<T>` local method contract:
  `push(value)`, `get(index)`, `set(index, value)`, and `length()`.
- `ARRAY-002B` implements direct element checks for typed `Array<T>` literal,
  `push`, and `set` values when the direct expression type is known.
- `ARRAY-002C` keeps unsupported `Array<T>` inference fail-fast, including
  unresolved generic element contexts such as `Array<T>`.
- `ARRAY-002D` guards ordinary `Array<T>` JSON v0 / ArrayBox lowering and
  keeps `PackedArray<T>` no-fallback behavior fixed.
- `len()` / `size()` aliases are not canonical typed `Array<T>` source.

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
| `ARRAY-001` | complete as `293x-313`; typed-context `Array<T>` literals lower with no inference and no PackedArray fallback | Stage1 typed collection |
| `ARRAY-002A` | complete as `293x-315`; typed `Array<T>` method contract for `push/get/set/length` | Stage1 typed collection |
| `ARRAY-002B` | complete as `293x-316`; typed local Array element checks | Stage1 typed collection |
| `ARRAY-002C` | complete as `293x-317`; unsupported Array inference fail-fast | Stage1 diagnostics |
| `ARRAY-002D` | complete as `293x-318`; ArrayBox JSON v0/backend guard and PackedArray no-fallback guard | Stage1/backend guard |
| `RESULT-001` | complete as `293x-314`; Result/Option prelude and explicit variant diagnostics | Stage1 enum/prelude |
| `RESULT-002A` | complete as `293x-319`; prelude enum missing-arm diagnostics | Stage1 diagnostics |
| `RESULT-002B` | complete as `293x-320`; prelude enum payload diagnostics | Stage1 diagnostics |
| `RESULT-002C` | known-enum exhaustiveness underscore rules | Stage1 diagnostics |
| `RESULT-002D` | generic enum expected-type diagnostics without inference | Stage1 diagnostics |
| `PACKED-001` | complete as `293x-293`; PackedArray eligibility gate and fail-fast policy | Stage1 CorePlan |

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
typed-context `Array<T>` literal JSON v0 bridge lowering
typed `Array<T>` method contract diagnostics
typed `Array<T>` direct element diagnostics
unsupported `Array<T>` inference fail-fast diagnostics
ordinary `Array<T>` JSON v0 / ArrayBox route guard
Result/Option diagnostics
Result/Option prelude enum registry
Result/Option prelude missing-arm diagnostics
Result/Option prelude payload diagnostics
PackedArray eligibility and backend fail-fast
```

## Stop Line

This SSOT fixes the canonical surface and task split. ARRAY-001 now implements
only the typed-context `Array<T>` literal slice. RESULT-001 now implements the
Result/Option prelude and dot-variant diagnostics. ARRAY-002A now implements
typed `Array<T>` method name and arity diagnostics. ARRAY-002B now implements
direct element diagnostics for known expressions. ARRAY-002C now keeps
unsupported Array inference fail-fast. ARRAY-002D guards the ordinary ArrayBox
route and the PackedArray no-fallback stop line. PackedArray literal/backend
implementation remains a separate row.
