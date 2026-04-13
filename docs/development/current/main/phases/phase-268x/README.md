Status: ACTIVE
Owner: Codex
Phase: 268x

# Phase 268x

## Summary

- land the next actual SIMD widening cut for compare/select candidates
- keep the cut narrow: `int_compare_select_candidate` only
- emit the same conservative `llvm.loop` vectorization hints for compare/select candidates under LoopSimdContract

## Current Cut

- actual widening is still hint-only; LLVM keeps profitability and target realization
- current accepted shape:
  - counted simple while candidate
  - straight-line arithmetic-only body
  - integer compare/select candidate
  - integer-only
- floating-point, fast-math, reassociation, and FMA remain out of scope

## Next

- decide numeric lane closeout after this cut
- keep floating-point reduction outside the current lane
