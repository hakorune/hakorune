# Phase 263x SSOT — numeric loop induction proof seam

Status: ACTIVE
Date: 2026-04-13
Scope: conservative numeric induction proof for simple while plans.

## Decision

- keep the first follow-on cut under `numeric loop / SIMD` narrow.
- annotate `simple while` plans with `numeric_kind=induction` only when:
  - the body is arithmetic-only,
  - the loop condition is integerish, and
  - the body exposes at least one integerish arithmetic carrier.
- keep reduction recognition / SIMD widening separate from this proof seam.

## Current Cut

- `prepass.loops.annotate_numeric_loop_plan(...)` adds numeric induction hints to simple while plans.
- `FunctionLowerContext.numeric_loop_plans` caches the annotated plan by loop header id.
- the existing while lowering path does not change behavior yet.

## Next

- widen only when a concrete numeric-loop or SIMD proof justifies it.
- keep reduction recognition as the next follow-on slice.
