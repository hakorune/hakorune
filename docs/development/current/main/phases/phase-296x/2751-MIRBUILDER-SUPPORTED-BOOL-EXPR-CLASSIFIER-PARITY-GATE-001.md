# 2751 MIRBUILDER-SUPPORTED-BOOL-EXPR-CLASSIFIER-PARITY-GATE-001

Status: Completed
Date: 2026-07-04

## Decision

Add a dedicated `.hako` EXE parity gate for `supported_bool_expr_classifier`.

## Evidence

```text
parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_supported_bool_expr_classifier_parity_gate.sh
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Supported bool-expression classification remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-SUPPORTED-BOOL-EXPR-CLASSIFIER-HAKO-ADOPTION-DECISION-001`
