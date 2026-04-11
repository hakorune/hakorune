# 165x-90: escape barrier vocabulary SSOT

Status: SSOT
Date: 2026-04-11
Scope: cut MIR-side operand-role escape vocabulary for the current narrow escape-analysis widening.

## Goal

- keep escape authority in MIR
- keep `used_values()` as generic def-use only
- teach escape analysis which operand roles publish/capture values without leaking that policy into runtime or backend code

## Authority

1. `.hako owner / policy`
2. `MIR canonical contract`
3. `Rust implementation under src/mir/**`
4. `runtime / LLVM consumers`

This phase only touches step 2 and step 3.

## Current Vocabulary

The first cut should classify these operand-role barriers:

- `Return`
- `Throw`
- `Call`
  - method receiver
  - call args
- `StoreLike`
  - `Store.value`
  - `FieldSet.value`
- `PhiMerge`
  - `Phi.inputs`
- `Capture`
  - `NewClosure.captures`
  - `NewClosure.me`
- `DebugObserve`
  - `Debug.value`

## Explicit Non-Goals

- no generic `used_values()` rewrite
- no runtime/helper-local special cases
- no LLVM-side escape rediscovery
- no cross-block / object-graph escape reasoning

## Acceptance

- `src/mir/escape_barrier.rs` exists as the vocabulary/API seam
- `src/mir/passes/escape.rs` consumes that API instead of hand-rolled instruction matching
- unit tests pin at least:
  - method receiver as `Call`
  - `FieldSet.base` excluded while `FieldSet.value` is `StoreLike`
