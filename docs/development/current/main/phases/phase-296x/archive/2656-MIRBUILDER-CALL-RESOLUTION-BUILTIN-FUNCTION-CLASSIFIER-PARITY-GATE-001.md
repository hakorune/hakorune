# 2656 MIRBUILDER-CALL-RESOLUTION-BUILTIN-FUNCTION-CLASSIFIER-PARITY-GATE-001

Status: Completed
Date: 2026-07-04

## Decision

Add a dedicated `.hako` EXE parity gate for `call_resolution_builtin_function_classifier`.

## Evidence

```text
parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_call_resolution_builtin_function_classifier_parity_gate.sh
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Call resolution remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-CALL-RESOLUTION-BUILTIN-FUNCTION-CLASSIFIER-HAKO-ADOPTION-DECISION-001`
