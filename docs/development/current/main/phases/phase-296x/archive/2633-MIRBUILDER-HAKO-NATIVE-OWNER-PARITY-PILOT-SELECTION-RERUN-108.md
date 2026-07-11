# 2633 MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-108

Status: Completed
Date: 2026-07-04

## Decision

Select `basic_block_empty_classifier` as the one-hundred-seventh narrow
Rust-oracle parity pilot owner.

## Scope

The selected owner covers only `BasicBlock::is_empty` from
`src/mir/basic_block.rs`.

## Non-Claims

- Source Selfhost remains unclaimed.
- BasicBlock control-flow behavior remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-BASIC-BLOCK-EMPTY-CLASSIFIER-RUST-ORACLE-FIXTURE-001`
