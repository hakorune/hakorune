# 2817 MIRBUILDER-ARRAY-TEXT-LOOP-SESSION-BACKEND-SESSION-LOWERING-ALLOWED-CLASSIFIER-HAKO-ADOPTION-DECISION-001

Status: Completed
Date: 2026-07-05

## Decision

Adopt `array_text_loop_session_backend_session_lowering_allowed_classifier` as
a narrow HakoAdopted Rust-oracle parity pilot owner after the green 7-row
`.hako` EXE parity gate.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-array-text-loop-session-backend-session-lowering-allowed-classifier-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/array_text_loop_session_backend_session_lowering_allowed_classifier.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_array_text_loop_session_backend_session_lowering_allowed_classifier_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-array-text-loop-session-backend-session-lowering-allowed-classifier-hako-adoption-decision-v0.json
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Rust bootstrap/oracle remains retained.
- ArrayTextLoopSessionPlan backend lowering remains Rust.
- MIR mutation remains Rust.

## Next

`MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-145`
