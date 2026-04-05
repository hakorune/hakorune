# Phase 150x: array string-store vertical slice

- Status: Planned
- 目的: `array string-store` を contract-first vertical slice の第2本目として固定する

## Slice Order

1. `.hako` owner keeps array store semantics
2. MIR normalizes to `store.array.str`
3. Rust executes borrowed-slot retarget / source-store as executor only

## Out Of Scope

- broad array store micro-hacks
- owner policy relocation into Rust
- LLVM-side semantic placement
