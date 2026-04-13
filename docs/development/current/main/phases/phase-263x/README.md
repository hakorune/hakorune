Status: LANDED
Owner: Codex
Phase: 263x

# Phase 263x

## Summary

- land the first numeric-loop induction proof seam
- keep the proof narrow: annotate simple while plans only when the body is arithmetic-only and the loop condition is integerish
- keep reduction recognition and SIMD widening separate from this proof seam

## Current Cut

- closeout: simple while prepass now annotates numeric induction candidates
- current coverage:
  - arithmetic-only loop bodies
  - integerish loop conditions
  - conservative induction value IDs for later widening
- fast-math / FMA / reduction recognition are still out of scope for this cut

## Closeout

- the follow-on proof seam now lives in `phase-264x`
- keep reduction recognition and SIMD widening as follow-on cuts
