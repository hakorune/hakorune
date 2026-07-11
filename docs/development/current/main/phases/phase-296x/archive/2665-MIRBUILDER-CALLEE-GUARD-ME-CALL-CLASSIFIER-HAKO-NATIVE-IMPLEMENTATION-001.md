# 2665 MIRBUILDER-CALLEE-GUARD-ME-CALL-CLASSIFIER-HAKO-NATIVE-IMPLEMENTATION-001

Status: Completed
Date: 2026-07-04

## Decision

Implement `callee_guard_me_call_classifier` in hand-authored `.hako`.

## Evidence

```text
hako_source:
  lang/src/compiler/lib/callee_guard_me_call_classifier.hako
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Call resolution remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-CALLEE-GUARD-ME-CALL-CLASSIFIER-PARITY-GATE-001`
