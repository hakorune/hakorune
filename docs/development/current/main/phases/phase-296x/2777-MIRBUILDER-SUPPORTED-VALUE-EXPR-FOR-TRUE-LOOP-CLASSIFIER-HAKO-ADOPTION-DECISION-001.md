# 2777 MIRBUILDER-SUPPORTED-VALUE-EXPR-FOR-TRUE-LOOP-CLASSIFIER-HAKO-ADOPTION-DECISION-001

Status: Completed
Date: 2026-07-04

## Decision

Adopt `supported_value_expr_for_true_loop_classifier` as a narrow HakoAdopted Rust-oracle parity pilot owner after the 4-row `.hako` EXE parity gate is green.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-supported-value-expr-for-true-loop-classifier-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/supported_value_expr_for_true_loop_classifier.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_supported_value_expr_for_true_loop_classifier_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-supported-value-expr-for-true-loop-classifier-hako-adoption-decision-v0.json
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Rust bootstrap/oracle remains retained.
- True-loop supported value-expression classification remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-137`
