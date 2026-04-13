Status: ACTIVE
Owner: Codex
Phase: 266x

# Phase 266x

## Summary

- land the first actual SIMD widening cut for integer map loops
- keep the cut narrow: `int_map_candidate` only
- emit conservative `llvm.loop` vectorization hints on loop backedges under LoopSimdContract

## Current Cut

- actual widening is still hint-only; LLVM keeps profitability and target realization
- current accepted shape:
  - counted simple while candidate
  - straight-line arithmetic-only body
  - no reduction
  - integer-only
- reduction widening, fast-math, reassociation, and FMA remain out of scope

## Next

- widen integer sum reductions under the same LoopSimdContract
- keep compare/select expansion separate
