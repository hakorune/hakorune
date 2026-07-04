# 2767 MIRBUILDER-PURE-VALUE-EXPR-FOR-GENERIC-LOOP-CLASSIFIER-HAKO-ADOPTION-DECISION-001

Status: Completed
Date: 2026-07-04

## Decision

Adopt `pure_value_expr_for_generic_loop_classifier` as a narrow HakoAdopted Rust-oracle parity pilot owner after the 4-row `.hako` EXE parity gate is green.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-pure-value-expr-for-generic-loop-classifier-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/pure_value_expr_for_generic_loop_classifier.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_pure_value_expr_for_generic_loop_classifier_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-pure-value-expr-for-generic-loop-classifier-hako-adoption-decision-v0.json
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Rust bootstrap/oracle remains retained.
- Generic-loop pure value-expression classification remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-135`
