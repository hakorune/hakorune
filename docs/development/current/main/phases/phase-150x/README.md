# Phase 150x: array string-store vertical slice

- Status: Current
- 目的: `array string-store` を contract-first vertical slice の第2本目として固定する

## Slice Order

1. `.hako` owner keeps array store semantics
2. MIR normalizes to `store.array.str`
3. Rust executes borrowed-slot retarget / source-store as executor only

## Current Concrete Path

- owner route:
  - `CollectionMethodPolicyBox.route_array_store_string() -> "ArrayStoreString"`
- current compiler/runtime symbols:
  - `classify_generic_method_set_route(...)`
  - `nyash.array.set_his`
  - `array_runtime_store_array_string(...)`
  - `array_string_store_handle_at(...)`

Goal:

- keep the owner route in `.hako`
- keep the canonical reading in MIR/docs as `store.array.str`
- demote `set_his` to ABI/executor detail

## Exit Gate

- do not return to `phase-137x`
- after this slice, land one more lock:
  - current concrete lowering must visibly answer to canonical `store.array.str`
  - the same visibility rule must also hold for `const_suffix`

## Out Of Scope

- broad array store micro-hacks
- owner policy relocation into Rust
- LLVM-side semantic placement
