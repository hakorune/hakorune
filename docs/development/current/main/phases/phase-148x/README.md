# Phase 148x: borrowed text and sink contract freeze

- Status: Landed
- 目的: borrowed text access と sink execution を Rust helper ではなく MIR contract 観点で固定する

## Focus

- `.hako` owner-side route vocabulary を読む
- MIR canonical op の最小集合を固定する
- Rust helper は internal executor protocol としてだけ扱う

## Candidate Contract

- `thaw.str`
- `lit.str`
- `str.concat2`
- `freeze.str`
- `store.array.str`
- `store.map.value`

## Landed Reading

- `.hako` route vocabulary remains the owner:
  - `const_suffix`
  - `ArrayStoreString`
  - `MapStoreAny`
- docs/SSOT now freeze the intended canonical MIR readings:
  - `thaw.str + lit.str + str.concat2 + freeze.str`
  - `store.array.str`
  - `store.map.value`
- current compiler/runtime code is still on concrete helper / extern symbols:
  - `nyash.string.concat_hs`
  - `nyash.array.set_his`
  - `nyash.map.slot_store_hhh`
