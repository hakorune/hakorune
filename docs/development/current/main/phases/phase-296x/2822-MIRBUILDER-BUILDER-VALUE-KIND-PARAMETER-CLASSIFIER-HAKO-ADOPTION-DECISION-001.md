# 2822 MIRBUILDER-BUILDER-VALUE-KIND-PARAMETER-CLASSIFIER-HAKO-ADOPTION-DECISION-001

Status: Completed
Date: 2026-07-05

## Decision

Adopt `builder_value_kind_parameter_classifier` as a narrow HakoAdopted
Rust-oracle parity pilot owner after the green 6-row `.hako` EXE parity gate.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-builder-value-kind-parameter-classifier-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/builder_value_kind_parameter_classifier.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_builder_value_kind_parameter_classifier_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-builder-value-kind-parameter-classifier-hako-adoption-decision-v0.json
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Rust bootstrap/oracle remains retained.
- MirBuilder value-kind classification remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-147`
