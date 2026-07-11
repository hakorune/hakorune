# 2675 MIRBUILDER-CALL-RESOLUTION-COMMONLY-SHADOWED-METHOD-CLASSIFIER-HAKO-NATIVE-IMPLEMENTATION-001

Status: Completed
Date: 2026-07-04

## Decision

Implement `call_resolution_commonly_shadowed_method_classifier` in hand-authored `.hako`.

## Evidence

```text
hako_source:
  lang/src/compiler/lib/call_resolution_commonly_shadowed_method_classifier.hako
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Call resolution remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-CALL-RESOLUTION-COMMONLY-SHADOWED-METHOD-CLASSIFIER-PARITY-GATE-001`
