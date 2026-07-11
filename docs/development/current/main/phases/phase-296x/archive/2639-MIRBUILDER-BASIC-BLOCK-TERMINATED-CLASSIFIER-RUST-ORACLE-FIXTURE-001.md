# 2639 MIRBUILDER-BASIC-BLOCK-TERMINATED-CLASSIFIER-RUST-ORACLE-FIXTURE-001

Status: Completed
Date: 2026-07-04

## Decision

Create the Rust-oracle fixture for `basic_block_terminated_classifier`.

## Evidence

```text
rust_source:
  src/mir/basic_block.rs

oracle_surface:
  BasicBlock::is_terminated boolean classification
```

## Non-Claims

- Source Selfhost remains unclaimed.
- BasicBlock control-flow behavior remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-BASIC-BLOCK-TERMINATED-CLASSIFIER-HAKO-NATIVE-IMPLEMENTATION-001`
