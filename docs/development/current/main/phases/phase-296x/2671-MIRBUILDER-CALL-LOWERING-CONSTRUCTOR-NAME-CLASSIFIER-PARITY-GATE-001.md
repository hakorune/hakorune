# 2671 MIRBUILDER-CALL-LOWERING-CONSTRUCTOR-NAME-CLASSIFIER-PARITY-GATE-001

Status: Completed
Date: 2026-07-04

## Decision

Add a dedicated `.hako` EXE parity gate for `call_lowering_constructor_name_classifier`.

## Evidence

```text
parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_call_lowering_constructor_name_classifier_parity_gate.sh
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Call lowering remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-CALL-LOWERING-CONSTRUCTOR-NAME-CLASSIFIER-HAKO-ADOPTION-DECISION-001`
