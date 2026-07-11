# Phase 265x SSOT — LoopSimdContract owner seam

Status: LANDED
Date: 2026-04-13
Scope: docs-first owner split for numeric-loop / SIMD widening.

## Decision

- keep the next `numeric loop / SIMD` cut docs-first.
- introduce `LoopSimdContract` as the owner seam that separates:
  - `proof`
  - `policy`
  - `lowering`
  - `diag`
- keep LLVM loop/vectorization metadata as lowering hints only.
- keep actual widening, fast-math, reassociation, FMA, predicated tail, scalable vectors, and floating-point reduction out of this seam.

## Current Cut

- `proof` owns counted-loop legality, induction, reduction, memory shape, and barrier exclusion.
- `policy` owns `off | auto_eligible | user_prefer` style mode plus future width/predication knobs.
- `lowering` owns LLVM-facing hints and attrs only after proof and policy are fixed.
- `diag` owns `accepted_class` / `reject_reason` without changing runtime behavior.
- current code seam:
  - `builders.loop_simd_contract.build_loop_simd_contract(...)` materializes the contract from the landed numeric loop proof
  - `FunctionLowerContext.loop_simd_contracts` stores that contract by loop header id

## Phase-1 Target

- first actual widening after this seam should prefer:
  - counted inner loop
  - straight-line body
  - affine unit-stride memory
  - no barrier
  - integer-only
- first widening follow-on should be integer map loops, not reduction or compare/select expansion.

## Guardrails

- do not let LLVM metadata become the semantic source of truth.
- do not emit `parallel_accesses`, `alias.scope`, or `noalias` unless the proof actually owns those facts.
- do not mix actual widening with this owner-seam cut.
