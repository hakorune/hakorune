Status: LANDED
Owner: Codex
Phase: 261x

# Phase 261x

## Summary

- land the first `escape / barrier -> LLVM attrs` seam
- keep the policy narrow: annotate known runtime helper declarations with conservative LLVM attrs
- keep MIR escape/barrier vocabulary separate from the LLVM attr application seam

## Current Cut

- runtime helper declarations are now the first attrs feed point
- builder finalization applies the policy after all helper declarations exist
- current coverage:
  - `readonly` on pure read-only query helpers
  - `nocapture` on pointer arguments for known non-capturing runtime bridges

## Closeout

- widen the attrs feed only when there is a concrete helper or barrier contract to justify it
- keep the later `numeric loop / SIMD` lane separate
