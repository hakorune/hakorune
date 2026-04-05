# Phase 151x: canonical lowering visibility lock

- Status: Current
- 目的: canonical MIR readings が docs 上だけでなく current concrete lowering に対しても追える状態を固定する

## Focus

- `const_suffix`
  - `.hako` owner route
  - canonical reading `thaw.str + lit.str + str.concat2 + freeze.str`
  - current concrete lowering / executor path
- `ArrayStoreString`
  - `.hako` owner route
  - canonical reading `store.array.str`
  - current concrete lowering / executor path
- `MapStoreAny`
  - `.hako` owner route
  - canonical reading `store.map.value`
  - current concrete lowering / executor path

## Landed-Target Reading

- `const_suffix`
  - owner route in `.hako`
  - canonical reading in docs/SSOT
  - concrete lowering in `hako_llvmc_ffi_string_chain_policy.inc`
  - Rust executor in `string_helpers.rs`
- `ArrayStoreString`
  - owner route in `.hako`
  - canonical reading `store.array.str`
  - concrete lowering in `hako_llvmc_ffi_generic_method_lowering.inc`
  - Rust executor in `array_runtime_store_array_string(...)` -> `array_slot_store_string_handle(...)`
- `MapStoreAny`
  - owner route in `.hako`
  - canonical reading `store.map.value`
  - concrete lowering in `hako_llvmc_ffi_generic_method_lowering.inc`
  - current LLVM-Python visibility helper in `collection_method_call.py`
  - Rust executor in `map_slot_store.rs`

## Exit Gate

- `phase-137x` を reopen してよいのは次が source-backed に読めるときだけ:
  - `.hako owner`
  - `MIR canonical reading`
  - `current concrete lowering`
  - `Rust executor`

- cleaner Rust helper names だけでは reopen しない
