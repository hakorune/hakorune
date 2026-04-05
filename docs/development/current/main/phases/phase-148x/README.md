# Phase 148x: borrowed text and sink contract freeze

- Status: Planned
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
