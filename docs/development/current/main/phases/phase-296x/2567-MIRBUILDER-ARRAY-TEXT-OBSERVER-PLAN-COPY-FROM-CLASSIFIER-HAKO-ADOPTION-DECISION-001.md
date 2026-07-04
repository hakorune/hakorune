# 2567 MIRBUILDER-ARRAY-TEXT-OBSERVER-PLAN-COPY-FROM-CLASSIFIER-HAKO-ADOPTION-DECISION-001

Status: Completed
Date: 2026-07-04

## Decision

Adopt `array_text_observer_plan_copy_from_classifier` as a narrow
HakoAdopted Rust-oracle parity pilot owner after the 4-row `.hako` EXE parity
gate is green.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-array-text-observer-plan-copy-from-classifier-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/array_text_observer_plan_copy_from_classifier.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_array_text_observer_plan_copy_from_classifier_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-array-text-observer-plan-copy-from-classifier-hako-adoption-decision-v0.json
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Rust bootstrap/oracle remains retained.
- Array-text observer route matching and observer contract handling remain
  Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-096`
