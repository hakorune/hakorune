# 2800 MIRBUILDER-DIRECT-STEP-PLACEMENT-CLASSIFIER-HAKO-NATIVE-IMPLEMENTATION-001

Status: Completed
Date: 2026-07-04

## Decision

Implement `direct_step_placement_classifier` as a hand-authored `.hako` owner.

## Evidence

```text
hako_source:
  lang/src/compiler/lib/direct_step_placement_classifier.hako
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Rust bootstrap/oracle remains retained.
- Generic-loop direct step placement classification remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-DIRECT-STEP-PLACEMENT-CLASSIFIER-PARITY-GATE-001`
