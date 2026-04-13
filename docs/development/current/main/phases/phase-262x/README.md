Status: LANDED
Owner: Codex
Phase: 262x

# Phase 262x

## Summary

- landed the first `numeric loop / SIMD` seam
- kept the initial policy narrow by centralizing LLVM vectorization knobs for numeric-loop codegen
- kept induction / reduction / fast-math widening separate from the first policy seam

## Current Cut

- closed out: numeric-loop codegen policy is centralized behind a dedicated helper
- current coverage:
  - `loop_vectorize` on when the opt level is at least 2
  - `slp_vectorize` on when the opt level is at least 2
- fast-math / FMA widening stayed out of scope for this cut

## Closeout

- the follow-on proof seam now lives in `phase-263x`
- keep induction normalization and reduction recognition as follow-on cuts, not part of the first seam
