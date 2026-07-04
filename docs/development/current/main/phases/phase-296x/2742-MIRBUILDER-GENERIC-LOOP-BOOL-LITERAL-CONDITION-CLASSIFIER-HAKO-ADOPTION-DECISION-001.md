# 2742 MIRBUILDER-GENERIC-LOOP-BOOL-LITERAL-CONDITION-CLASSIFIER-HAKO-ADOPTION-DECISION-001

Status: Completed
Date: 2026-07-04

## Decision

Adopt `generic_loop_bool_literal_condition_classifier` as a narrow HakoAdopted Rust-oracle parity pilot owner after the 3-row `.hako` EXE parity gate is green.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-generic-loop-bool-literal-condition-classifier-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/generic_loop_bool_literal_condition_classifier.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_generic_loop_bool_literal_condition_classifier_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-generic-loop-bool-literal-condition-classifier-hako-adoption-decision-v0.json
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Rust bootstrap/oracle remains retained.
- Generic-loop bool-literal condition classification remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-130`
