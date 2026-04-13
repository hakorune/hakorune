# Phase 264x SSOT — numeric loop reduction recognition proof seam

Status: ACTIVE
Date: 2026-04-13
Scope: conservative reduction candidate recognition for simple while plans.

## Decision

- keep the next numeric-loop / SIMD cut narrow.
- annotate `simple while` plans with reduction candidates only when:
  - the body is arithmetic-only,
  - the loop condition is integerish,
  - the header exposes accumulator-like φ values, and
  - those carriers are not part of the compare operands.
- keep SIMD widening and fast-math separate from this proof seam.

## Current Cut

- `prepass.loops.annotate_numeric_loop_plan(...)` adds numeric reduction hints to simple while plans.
- `FunctionLowerContext.numeric_loop_plans` caches the annotated plan by loop header id.
- the existing while lowering path still does not change behavior.

## Next

- widen only when a concrete numeric-loop or SIMD proof justifies it.
- keep SIMD widening as the next follow-on slice after this proof seam.
