# 2710 MIRBUILDER-ARRAY-TEXT-OBSERVER-PLAN-SAME-SLOT-SET-CLASSIFIER-HAKO-NATIVE-IMPLEMENTATION-001

Status: Completed
Date: 2026-07-04

## Decision

Implement `array_text_observer_plan_same_slot_set_classifier` in hand-authored `.hako`.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-array-text-observer-plan-same-slot-set-classifier-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/array_text_observer_plan_same_slot_set_classifier.hako
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Array-text observer diagnostic routing remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-ARRAY-TEXT-OBSERVER-PLAN-SAME-SLOT-SET-CLASSIFIER-PARITY-GATE-001`
