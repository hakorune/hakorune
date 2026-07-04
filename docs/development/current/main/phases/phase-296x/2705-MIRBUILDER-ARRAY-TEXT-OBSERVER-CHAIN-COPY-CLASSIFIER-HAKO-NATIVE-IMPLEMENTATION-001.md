# 2705 MIRBUILDER-ARRAY-TEXT-OBSERVER-CHAIN-COPY-CLASSIFIER-HAKO-NATIVE-IMPLEMENTATION-001

Status: Completed
Date: 2026-07-04

## Decision

Implement `array_text_observer_chain_copy_classifier` in hand-authored `.hako`.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-array-text-observer-chain-copy-classifier-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/array_text_observer_chain_copy_classifier.hako
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Array-text observer route matching remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-ARRAY-TEXT-OBSERVER-CHAIN-COPY-CLASSIFIER-PARITY-GATE-001`
