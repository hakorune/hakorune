# 2787 MIRBUILDER-SUPPORTED-NESTED-LOOP-CONDITION-CLASSIFIER-HAKO-ADOPTION-DECISION-001

Status: Completed
Date: 2026-07-04

## Decision

Adopt `supported_nested_loop_condition_classifier` as a narrow HakoAdopted Rust-oracle parity pilot owner after the 5-row `.hako` EXE parity gate is green.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-supported-nested-loop-condition-classifier-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/supported_nested_loop_condition_classifier.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_supported_nested_loop_condition_classifier_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-supported-nested-loop-condition-classifier-hako-adoption-decision-v0.json
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Rust bootstrap/oracle remains retained.
- True-loop nested condition classification remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-139`
