# 2488 MIRBUILDER-ARRAY-TEXT-OBSERVER-ARG-REPR-CONST-UTF8-CLASSIFIER-HAKO-ADOPTION-DECISION-001

Status: Completed
Date: 2026-07-04

## Decision

Adopt `array_text_observer_arg_repr_const_utf8_classifier` as a narrow
HakoAdopted Rust-oracle parity pilot owner after the 2-row `.hako` EXE parity
gate is green.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-array-text-observer-arg-repr-const-utf8-classifier-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/array_text_observer_arg_repr_const_utf8_classifier.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_array_text_observer_arg_repr_const_utf8_classifier_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-array-text-observer-arg-repr-const-utf8-classifier-hako-adoption-decision-v0.json
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Rust bootstrap/oracle remains retained.
- Array-text observer route matching and observer contract handling remain
  Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-078`
