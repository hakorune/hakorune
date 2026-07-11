# Phase 268x SSOT — compare/select widening

Status: LANDED
Date: 2026-04-13
Scope: landed compare/select widening cut under LoopSimdContract.

## Decision

- next actual widening is `int_compare_select_candidate` only.
- lowering emits the same conservative `llvm.loop` vectorization hints on loop backedges.
- compare/select candidates stay within integer-only loop bodies.
- profitability, VF/UF choice, epilogue shape, and target-specific vector lowering remain LLVM-owned.

## Current Cut

- `prepass.loops.annotate_numeric_loop_plan(...)` now carries `numeric_select_value_ids`.
- `builders.loop_simd_contract.build_loop_simd_contract(...)` lowers compare/select candidates to hint-ready loop metadata.
- map widening and reduction widening remain landed and unchanged.

## Guardrails

- do not treat LLVM loop metadata as semantic truth.
- do not widen floating-point or fast-math dependent shapes in this phase.
- do not add reassociation, FMA, predicated tail, or scalable vectors here.
