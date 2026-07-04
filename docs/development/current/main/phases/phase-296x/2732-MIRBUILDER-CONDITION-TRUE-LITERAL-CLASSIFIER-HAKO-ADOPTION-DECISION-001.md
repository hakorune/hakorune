# 2732 MIRBUILDER-CONDITION-TRUE-LITERAL-CLASSIFIER-HAKO-ADOPTION-DECISION-001

Status: Completed
Date: 2026-07-04

## Decision

Adopt `condition_true_literal_classifier` as a narrow HakoAdopted Rust-oracle parity pilot owner after the 3-row `.hako` EXE parity gate is green.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-condition-true-literal-classifier-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/condition_true_literal_classifier.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_condition_true_literal_classifier_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-condition-true-literal-classifier-hako-adoption-decision-v0.json
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Rust bootstrap/oracle remains retained.
- Condition true-literal classification remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-128`
