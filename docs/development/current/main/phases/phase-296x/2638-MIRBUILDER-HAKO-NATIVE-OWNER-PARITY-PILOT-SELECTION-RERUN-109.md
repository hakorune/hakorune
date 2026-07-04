# 2638 MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-109

Status: Completed
Date: 2026-07-04

## Decision

Select `basic_block_terminated_classifier` as the one-hundred-eighth narrow
Rust-oracle parity pilot owner.

## Scope

The selected owner covers only `BasicBlock::is_terminated` from
`src/mir/basic_block.rs`.

## Non-Claims

- Source Selfhost remains unclaimed.
- BasicBlock control-flow behavior remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-BASIC-BLOCK-TERMINATED-CLASSIFIER-RUST-ORACLE-FIXTURE-001`
