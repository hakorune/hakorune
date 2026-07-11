# 2691 MIRBUILDER-STATIC-METHOD-NAME-CLASSIFIER-PARITY-GATE-001

Status: Completed
Date: 2026-07-04

## Decision

Add a dedicated `.hako` EXE parity gate for `static_method_name_classifier`.

## Evidence

```text
parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_static_method_name_classifier_parity_gate.sh
```

## Non-Claims

- Source Selfhost remains unclaimed.
- MIR naming remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-STATIC-METHOD-NAME-CLASSIFIER-HAKO-ADOPTION-DECISION-001`
