Status: LANDED
Owner: Codex
Phase: 264x

# Phase 264x

## Summary

- land the first numeric-loop reduction recognition proof seam
- keep the proof narrow: annotate simple while plans with conservative reduction candidates only when the body is arithmetic-only and the header carries a non-compare accumulator
- keep SIMD widening and fast-math separate from this proof seam

## Current Cut

- simple while prepass now annotates conservative reduction candidates
- current coverage:
  - arithmetic-only loop bodies
  - integerish loop conditions
  - header-carried accumulator candidates that are not part of the compare operands
- SIMD widening / fast-math / FMA remain out of scope for this cut

## Closeout

- reduction-recognition proof seam is closed out at this boundary
- next follow-on is the LoopSimdContract owner seam
- actual SIMD widening remains a separate cut

## Next

- only widen when a concrete numeric-loop or SIMD proof justifies a new knob
- keep actual SIMD widening under the next LoopSimdContract-owned slice
