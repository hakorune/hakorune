# Phase 151x: canonical lowering visibility lock

- Status: Planned
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

## Exit Gate

- `phase-137x` を reopen してよいのは次が source-backed に読めるときだけ:
  - `.hako owner`
  - `MIR canonical reading`
  - `current concrete lowering`
  - `Rust executor`

- cleaner Rust helper names だけでは reopen しない
