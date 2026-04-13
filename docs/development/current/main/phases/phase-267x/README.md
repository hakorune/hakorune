Status: ACTIVE
Owner: Codex
Phase: 267x

# Phase 267x

## Summary

- land the next actual SIMD widening cut for integer sum reductions
- keep the cut narrow: `int_reduction_candidate` only
- emit the same conservative `llvm.loop` vectorization hints for reduction candidates under LoopSimdContract

## Current Cut

- actual widening is still hint-only; LLVM keeps profitability and target realization
- current accepted shape:
  - counted simple while candidate
  - straight-line arithmetic-only body
  - integer reduction candidate
  - integer-only
- compare/select widening, fast-math, reassociation, and FMA remain out of scope

## Next

- widen compare/select only in a separate follow-on
- keep floating-point reduction outside the current lane
