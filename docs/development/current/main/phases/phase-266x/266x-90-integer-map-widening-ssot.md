# Phase 266x SSOT — integer map widening

Status: ACTIVE
Date: 2026-04-13
Scope: first actual SIMD widening cut under LoopSimdContract.

## Decision

- first actual widening is `int_map_candidate` only.
- lowering emits conservative `llvm.loop` vectorization hints on loop backedges.
- profitability, VF/UF choice, epilogue shape, and target-specific vector lowering remain LLVM-owned.
- integer reduction candidates remain deferred for the next cut.

## Current Cut

- `builders.loop_simd_contract.apply_loop_simd_metadata(...)` lowers the current contract into `llvm.loop` hints.
- loop backedges in both regular while lowering and LoopForm lowering can now carry that metadata.
- `int_reduction_candidate` still stays deferred in lowering.

## Guardrails

- do not treat LLVM loop metadata as semantic truth.
- do not widen reduction candidates in this phase.
- do not add fast-math, reassociation, FMA, predicated tail, or scalable vectors here.
