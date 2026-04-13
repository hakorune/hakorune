Status: ACTIVE
Owner: Codex
Phase: 265x

# Phase 265x

## Summary

- pin the LoopSimdContract owner seam for `numeric loop / SIMD`
- split proof, policy, lowering, and diagnostics before any actual SIMD widening
- keep integer-map widening as the first follow-on slice after this contract seam

## Current Cut

- introduce one owner vocabulary for numeric-loop SIMD eligibility
- `builders.loop_simd_contract.build_loop_simd_contract(...)` now owns the contract shape
- `FunctionLowerContext.loop_simd_contracts` caches the contract by loop header id
- keep LLVM loop metadata as lowering hints, not the semantic source of truth
- keep actual widening, fast-math, reassociation, and FMA out of scope

## Next

- land the first actual widening slice only after the LoopSimdContract seam is fixed
- preferred follow-on: integer map loop widening under a counted / straight-line / unit-stride / int-only contract
