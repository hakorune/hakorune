Status: ACTIVE
Owner: Codex
Phase: 262x

# Phase 262x

## Summary

- land the first `numeric loop / SIMD` seam
- keep the policy narrow: centralize LLVM vectorization knobs for numeric-loop codegen
- keep induction / reduction / fast-math widening separate from the first policy seam

## Current Cut

- numeric-loop codegen policy is centralized behind a dedicated helper
- current coverage:
  - `loop_vectorize` on when the opt level is at least 2
  - `slp_vectorize` on when the opt level is at least 2
- fast-math / FMA widening is still out of scope for this cut

## Next

- widen only when a concrete numeric-loop or SIMD proof justifies it
- keep induction normalization and reduction recognition as follow-on cuts, not part of the first seam
