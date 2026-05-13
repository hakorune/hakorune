# BRAND-002 Stage1 Brand Constructor / Unwrap Policy SSOT

Status: accepted
Date: 2026-05-14
Lane: phase-293x language minimal surface lane

## Decision

Use existing call syntax for explicit brand construction and explicit unwrap.

```hako
brand PageId: i64

local page = PageId(7)
local raw = PageId.unwrap(page)
```

`BrandName(value)` constructs a branded scalar. `BrandName.unwrap(value)` extracts
the underlying scalar. Both are explicit and Stage1-owned.

## Stage1 owns

- Recognize `BrandName(value)` when `BrandName` is declared by a top-level
  `brand` declaration.
- Require exactly one constructor argument.
- Recognize `BrandName.unwrap(value)` for declared brands.
- Require exactly one unwrap argument.
- Reject other brand-qualified static methods.
- Lower to Program JSON v0 semantic nodes:
  - `BrandConstruct`
  - `BrandUnwrap`

## Stage1 does not own yet

- Flow-sensitive type inference.
- Brand mismatch checking at function call boundaries.
- Rejecting all implicit assignments/conversions between underlying scalars and
  branded values.
- Verifier/CorePlan brand facts.

Those remain `BRAND-003` and later verifier rows.

## Program JSON v0 shapes

```json
{
  "type": "BrandConstruct",
  "brand": "PageId",
  "underlying_type": "i64",
  "value": { "type": "Int", "value": 7 }
}
```

```json
{
  "type": "BrandUnwrap",
  "brand": "PageId",
  "underlying_type": "i64",
  "value": { "type": "Var", "name": "page" }
}
```

## Fail-fast cases

```text
[brand/constructor-arity]
[brand/unwrap-arity]
[brand/unsupported-static-method]
```

## Stop lines

```text
no implicit brand conversion
no generic unwrap function
no Stage0 brand semantics
no mismatch checker in this row
```

## Retire condition

Retire this Rust Stage1 lowering once the selfhost Stage1 owner emits equivalent
`BrandConstruct` / `BrandUnwrap` semantics and fail-fast diagnostics.
