# Phase 149x: concat const-suffix vertical slice

- Status: Planned
- 目的: `concat const-suffix` を最初の contract-first vertical slice にする

## Slice Order

1. `.hako` owner policy keeps route authority
2. MIR normalizes to `thaw.str + lit.str + str.concat2 + freeze.str`
3. Rust executes through plan / freeze substrate

## Out Of Scope

- broad shape-specific fast path growth
- array store redesign
- LLVM-side semantic placement
