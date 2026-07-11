# 2715 MIRBUILDER-ARRAY-TEXT-RESIDENCE-SESSION-ADD-CONST-ONE-FROM-CLASSIFIER-HAKO-NATIVE-IMPLEMENTATION-001

Status: Completed
Date: 2026-07-04

## Decision

Implement `array_text_residence_session_add_const_one_from_classifier` in hand-authored `.hako`.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-array-text-residence-session-add-const-one-from-classifier-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/array_text_residence_session_add_const_one_from_classifier.hako
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Array-text residence-session derivation remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-ARRAY-TEXT-RESIDENCE-SESSION-ADD-CONST-ONE-FROM-CLASSIFIER-PARITY-GATE-001`
