# 2737 MIRBUILDER-NORMALIZED-SHADOW-BOOL-TRUE-LITERAL-CLASSIFIER-HAKO-ADOPTION-DECISION-001

Status: Completed
Date: 2026-07-04

## Decision

Adopt `normalized_shadow_bool_true_literal_classifier` as a narrow HakoAdopted Rust-oracle parity pilot owner after the 3-row `.hako` EXE parity gate is green.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-normalized-shadow-bool-true-literal-classifier-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/normalized_shadow_bool_true_literal_classifier.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_normalized_shadow_bool_true_literal_classifier_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-normalized-shadow-bool-true-literal-classifier-hako-adoption-decision-v0.json
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Rust bootstrap/oracle remains retained.
- Normalized-shadow bool-true-literal classification remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-129`
