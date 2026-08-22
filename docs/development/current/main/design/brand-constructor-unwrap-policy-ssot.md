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

### Namespace and result identity

- The effective top-level Brand declaration set is program-wide after build-gate
  pruning; source order does not limit later or earlier constructor use.
- Two effective declarations with the same Brand name are rejected before
  function resolution or argument effects with `[brand/duplicate-declaration]`.
  Map overwrite order is not language meaning.
- For a bare identifier call whose name is a declared Brand, Brand construction
  owns the site. It is not also a FreeStatic, TypeOp, Math, `str`, or other
  compatibility call. Canonical explicit externcall has a dedicated syntax and
  does not collide; dotted names are not legal Brand declarations.
- Construction produces a semantic Brand identity consisting of the exact
  declaration identity, Brand name, and underlying type. A physical backend may
  reuse the underlying scalar representation, but it must not erase the
  semantic identity before verified use or explicit unwrap.
- Constructor and unwrap relations are issued from one AST-free effective Brand
  declaration catalog at exact source sites. Resolver and lowering consume that
  relation; they do not re-pair a call name with a copied map or recover Brand
  meaning from a FreeStatic miss.

The grammar registry owns only the `brand Name: Type` declaration spelling.
Constructor and unwrap reuse existing call syntax; their contextual Brand
meaning belongs to this semantic policy and must not be duplicated as new
grammar productions.

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

These deferred checks do not permit construction to lose its Brand identity.
They only defer validation of implicit conversions and cross-call mismatches.

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
