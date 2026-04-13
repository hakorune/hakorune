Status: ACTIVE
Owner: Codex
Phase: 263x

# Phase 263x

## Summary

- land the first numeric-loop induction proof seam
- keep the proof narrow: annotate simple while plans only when the body is arithmetic-only and the loop condition is integerish
- keep reduction recognition and SIMD widening separate from this proof seam

## Current Cut

- simple while prepass now annotates numeric induction candidates
- current coverage:
  - arithmetic-only loop bodies
  - integerish loop conditions
  - conservative induction value IDs for later widening
- fast-math / FMA / reduction recognition are still out of scope for this cut

## Next

- only widen when a concrete numeric-loop or SIMD proof justifies a new knob
- keep reduction recognition and SIMD widening as follow-on cuts
