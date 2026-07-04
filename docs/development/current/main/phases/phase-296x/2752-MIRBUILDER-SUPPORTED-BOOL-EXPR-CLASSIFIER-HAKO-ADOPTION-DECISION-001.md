# 2752 MIRBUILDER-SUPPORTED-BOOL-EXPR-CLASSIFIER-HAKO-ADOPTION-DECISION-001

Status: Completed
Date: 2026-07-04

## Decision

Adopt `supported_bool_expr_classifier` as a narrow HakoAdopted Rust-oracle parity pilot owner after the 4-row `.hako` EXE parity gate is green.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-supported-bool-expr-classifier-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/supported_bool_expr_classifier.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_supported_bool_expr_classifier_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-supported-bool-expr-classifier-hako-adoption-decision-v0.json
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Rust bootstrap/oracle remains retained.
- Supported bool-expression classification remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-132`
