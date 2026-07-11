# 2660 MIRBUILDER-CALL-RESOLUTION-ENV-INTERFACE-CLASSIFIER-HAKO-NATIVE-IMPLEMENTATION-001

Status: Completed
Date: 2026-07-04

## Decision

Implement `call_resolution_env_interface_classifier` in hand-authored `.hako`.

## Evidence

```text
hako_source:
  lang/src/compiler/lib/call_resolution_env_interface_classifier.hako
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Call resolution remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-CALL-RESOLUTION-ENV-INTERFACE-CLASSIFIER-PARITY-GATE-001`
