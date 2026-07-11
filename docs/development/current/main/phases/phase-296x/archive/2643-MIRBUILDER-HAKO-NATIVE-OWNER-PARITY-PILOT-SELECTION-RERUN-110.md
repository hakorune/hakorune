# 2643 MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-110

Status: Completed
Date: 2026-07-04

## Decision

Select `basic_block_sealed_classifier` as the one-hundred-ninth narrow
Rust-oracle parity pilot owner.

## Scope

The selected owner covers only `BasicBlock::is_sealed` from
`src/mir/basic_block.rs`.

## Non-Claims

- Source Selfhost remains unclaimed.
- BasicBlock control-flow behavior remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-BASIC-BLOCK-SEALED-CLASSIFIER-RUST-ORACLE-FIXTURE-001`
