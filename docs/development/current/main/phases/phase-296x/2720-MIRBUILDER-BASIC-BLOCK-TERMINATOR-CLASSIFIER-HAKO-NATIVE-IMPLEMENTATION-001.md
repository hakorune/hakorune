# 2720 MIRBUILDER-BASIC-BLOCK-TERMINATOR-CLASSIFIER-HAKO-NATIVE-IMPLEMENTATION-001

Status: Completed
Date: 2026-07-04

## Decision

Implement `basic_block_terminator_classifier` in hand-authored `.hako`.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-basic-block-terminator-classifier-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/basic_block_terminator_classifier.hako
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Basic-block terminator classification remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-BASIC-BLOCK-TERMINATOR-CLASSIFIER-PARITY-GATE-001`
