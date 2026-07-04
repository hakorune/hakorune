# 2670 MIRBUILDER-CALL-LOWERING-CONSTRUCTOR-NAME-CLASSIFIER-HAKO-NATIVE-IMPLEMENTATION-001

Status: Completed
Date: 2026-07-04

## Decision

Implement `call_lowering_constructor_name_classifier` in hand-authored `.hako`.

## Evidence

```text
hako_source:
  lang/src/compiler/lib/call_lowering_constructor_name_classifier.hako
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Call lowering remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-CALL-LOWERING-CONSTRUCTOR-NAME-CLASSIFIER-PARITY-GATE-001`
