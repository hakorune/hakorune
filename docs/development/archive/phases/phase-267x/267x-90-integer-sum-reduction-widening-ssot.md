# Phase 267x SSOT — integer sum reduction widening

Status: LANDED
Date: 2026-04-13
Scope: next actual SIMD widening cut under LoopSimdContract.

## Decision

- next actual widening is `int_reduction_candidate` only.
- lowering emits the same conservative `llvm.loop` vectorization hints on loop backedges.
- reduction candidates stay within integer-only add-style reductions.
- profitability, VF/UF choice, epilogue shape, and target-specific vector lowering remain LLVM-owned.

## Current Cut

- `builders.loop_simd_contract.build_loop_simd_contract(...)` now lowers `int_reduction_candidate` to hint-ready loop metadata.
- `builders.loop_simd_contract.apply_loop_simd_metadata(...)` now accepts integer reduction candidates.
- map widening remains landed and unchanged.

## Guardrails

- do not treat LLVM loop metadata as semantic truth.
- do not widen compare/select or floating-point reductions in this phase.
- do not add fast-math, reassociation, FMA, predicated tail, or scalable vectors here.
