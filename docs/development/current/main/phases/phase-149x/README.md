# Phase 149x: concat const-suffix vertical slice

- Status: Current
- 目的: `concat const-suffix` を最初の contract-first vertical slice にする

## Slice Order

1. `.hako` owner policy keeps route authority
2. MIR normalizes to `thaw.str + lit.str + str.concat2 + freeze.str`
3. Rust executes through plan / freeze substrate

## Current Concrete Path

- owner route:
  - `StringChainPolicyBox.concat_pair_route(...)->"const_suffix"`
- current compiler/runtime symbols:
  - `try_emit_string_concat_const_suffix_call(...)`
  - `nyash.string.concat_hs`
  - `concat_const_suffix_fallback(...)`

Goal:

- keep the owner route in `.hako`
- keep the canonical reading in MIR/docs
- demote direct helper naming to executor detail

## Out Of Scope

- broad shape-specific fast path growth
- array store redesign
- LLVM-side semantic placement
