# 2682 MIRBUILDER-CALL-RESOLUTION-EXTERN-FUNCTION-CLASSIFIER-HAKO-ADOPTION-DECISION-001

Status: Completed
Date: 2026-07-04

## Decision

Adopt `call_resolution_extern_function_classifier` as a narrow HakoAdopted Rust-oracle parity pilot owner after the 4-row `.hako` EXE parity gate is green.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-call-resolution-extern-function-classifier-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/call_resolution_extern_function_classifier.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_call_resolution_extern_function_classifier_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-call-resolution-extern-function-classifier-hako-adoption-decision-v0.json
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Rust bootstrap/oracle remains retained.
- Call resolution remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-118`
