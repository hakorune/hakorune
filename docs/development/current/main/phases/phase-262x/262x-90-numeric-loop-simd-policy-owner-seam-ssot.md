# Phase 262x SSOT — numeric loop / SIMD policy owner seam

Status: LANDED
Date: 2026-04-13
Scope: centralize the first LLVM vectorization knob seam for numeric-loop codegen.

## Decision

- numeric-loop / SIMD starts as a policy seam, not as a new lowering route.
- `loop_vectorize` and `slp_vectorize` stay behind one dedicated helper.
- fast-math / FMA promotion is explicitly out of scope for this first cut.

## Current Cut

- builder finalization now delegates numeric-loop pass policy to one helper.
- `opt_level >= 2` enables `loop_vectorize` and `slp_vectorize`.
- lower opt levels keep both disabled.

## Next

- the follow-on proof seam now lives in `phase-263x`.
- keep induction normalization / reduction recognition as follow-on slices.
