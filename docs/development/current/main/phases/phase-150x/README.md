# Phase 150x: array string-store vertical slice

- Status: Current-next
- 目的: `array string-store` を contract-first vertical slice の第2本目として固定する

## Slice Order

1. `.hako` owner keeps array store semantics
2. MIR normalizes to `store.array.str`
3. Rust executes borrowed-slot retarget / source-store as executor only

## Exit Gate

- do not return to `phase-137x`
- after this slice, land one more lock:
  - current concrete lowering must visibly answer to canonical `store.array.str`
  - the same visibility rule must also hold for `const_suffix`

## Out Of Scope

- broad array store micro-hacks
- owner policy relocation into Rust
- LLVM-side semantic placement
