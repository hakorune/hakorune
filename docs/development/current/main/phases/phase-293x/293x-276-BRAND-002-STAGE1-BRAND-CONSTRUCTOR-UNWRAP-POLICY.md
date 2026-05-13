# 293x-276 BRAND-002 Stage1 Brand Constructor / Unwrap Policy

Status: complete
Date: 2026-05-14

## Scope

Define and lower explicit brand constructor/unwrap semantics without adding new
surface keywords.

## Landed changes

- `BrandName(value)` lowers to `BrandConstruct` when `BrandName` is declared by
  a top-level `brand` declaration.
- `BrandName.unwrap(value)` lowers to `BrandUnwrap`.
- Constructor and unwrap both require exactly one argument.
- Other brand-qualified static methods fail fast.
- Program JSON v0 lowering context now carries declared brand metadata.
- Added Stage1 Program JSON v0 tests and guard coverage.

## Non-goals

- No brand mismatch checker.
- No flow-sensitive type inference.
- No implicit conversion rejection beyond explicit constructor/unwrap arity.
- No verifier facts.
- No CorePlan brand typing semantics.

## Guard

```bash
bash tools/checks/k2_wide_brand_constructor_unwrap_guard.sh
```

## Next selected row

`BRAND-003 Stage1 brand mismatch checker`.

`LOOP-003 Stage1 LoopRange lowering` remains open and should be handled as a
JoinIR/CorePlan route, not as source-level range-loop desugar.
