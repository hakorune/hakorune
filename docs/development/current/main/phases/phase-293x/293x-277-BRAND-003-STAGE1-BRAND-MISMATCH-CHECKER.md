# 293x-277 BRAND-003 Stage1 Brand Mismatch Checker

Status: complete
Date: 2026-05-14

## Scope

Reject same-program brand argument mismatches before Program JSON v0 lowering.

## Landed changes

- Added a Stage1 brand checker in the Program JSON v0 authority path.
- Tracks brand facts from function/method parameters and explicit brand
  constructors.
- Treats `BrandName.unwrap(value)` as unbranded.
- Checks calls to same-program free functions and box methods.
- Rejects mismatched brand arguments and unbranded values passed to brand-typed
  parameters.
- Added Stage1 tests and guard coverage.

## Non-goals

- No cross-module resolution.
- No field type propagation.
- No generic substitution.
- No return type checking.
- No backend representation changes.

## Guard

```bash
bash tools/checks/k2_wide_brand_mismatch_checker_guard.sh
```

## Next selected row

`TYPE-001 Stage0 type alias metadata capsule`.

`LOOP-003 Stage1 LoopRange lowering` remains open and should be handled as a
JoinIR/CorePlan route, not as source-level range-loop desugar.
