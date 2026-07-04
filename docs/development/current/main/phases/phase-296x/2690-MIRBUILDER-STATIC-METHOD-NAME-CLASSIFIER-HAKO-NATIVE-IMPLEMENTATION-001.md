# 2690 MIRBUILDER-STATIC-METHOD-NAME-CLASSIFIER-HAKO-NATIVE-IMPLEMENTATION-001

Status: Completed
Date: 2026-07-04

## Decision

Implement `static_method_name_classifier` in hand-authored `.hako`.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-static-method-name-classifier-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/static_method_name_classifier.hako
```

## Non-Claims

- Source Selfhost remains unclaimed.
- MIR naming remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-STATIC-METHOD-NAME-CLASSIFIER-PARITY-GATE-001`
