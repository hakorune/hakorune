# 2722 MIRBUILDER-BASIC-BLOCK-TERMINATOR-CLASSIFIER-HAKO-ADOPTION-DECISION-001

Status: Completed
Date: 2026-07-04

## Decision

Adopt `basic_block_terminator_classifier` as a narrow HakoAdopted Rust-oracle parity pilot owner after the 5-row `.hako` EXE parity gate is green.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-basic-block-terminator-classifier-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/basic_block_terminator_classifier.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_basic_block_terminator_classifier_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-basic-block-terminator-classifier-hako-adoption-decision-v0.json
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Rust bootstrap/oracle remains retained.
- Basic-block terminator classification remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-126`
