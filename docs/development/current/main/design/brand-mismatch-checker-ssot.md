# BRAND-003 Stage1 Brand Mismatch Checker SSOT

Status: accepted
Date: 2026-05-14
Lane: phase-293x language minimal surface lane

## Decision

Add a conservative Stage1 brand mismatch checker before Program JSON v0 lowering.

The checker rejects calls where a parameter declared with one brand receives a
value known to have another brand, or receives an unbranded value.

```hako
brand PageId: i64
brand BlockId: i64

local page = PageId(7)
me.releaseLocal(page)  // reject: expected BlockId, got PageId
```

## Stage1 owns

- Collect brand declarations.
- Collect same-program free function and box method signatures.
- Track function/method parameters declared as brands.
- Infer local brand facts from explicit `BrandName(value)` constructors.
- Treat `BrandName.unwrap(value)` as unbranded underlying value.
- Check brand-typed call arguments for:
  - matching brand values
  - mismatched brand values
  - unbranded values passed where a brand is required
- Fail fast with `[brand/mismatch]`.

## Stage1 does not own yet

- Full flow-sensitive type inference.
- Field type propagation.
- Cross-module function/method resolution.
- Generic substitution.
- Return type checking.
- Assignment checking for future typed locals.
- Backend representation changes.

## Current accepted facts

```text
BrandName(value) -> value has brand BrandName
BrandName.unwrap(value) -> value is unbranded
function parameter declared as BrandName -> argument must be BrandName
unbranded scalar into brand parameter -> reject
other brand into brand parameter -> reject
```

## Fail-fast case

```text
[brand/mismatch]
```

## Stop lines

```text
no implicit conversion
no cross-module guessing
no field/receiver inference until the owner exists
no backend fallback
```

## Retire condition

Retire this Rust Stage1 checker once the selfhost semantic owner provides the
same same-program mismatch checks and diagnostics before Program JSON lowering.
