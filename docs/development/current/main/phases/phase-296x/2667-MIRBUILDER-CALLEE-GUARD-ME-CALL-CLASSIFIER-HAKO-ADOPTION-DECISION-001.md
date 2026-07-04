# 2667 MIRBUILDER-CALLEE-GUARD-ME-CALL-CLASSIFIER-HAKO-ADOPTION-DECISION-001

Status: Completed
Date: 2026-07-04

## Decision

Adopt `callee_guard_me_call_classifier` as a narrow HakoAdopted Rust-oracle parity pilot owner after the 4-row `.hako` EXE parity gate is green.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-callee-guard-me-call-classifier-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/callee_guard_me_call_classifier.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_callee_guard_me_call_classifier_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-callee-guard-me-call-classifier-hako-adoption-decision-v0.json
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Rust bootstrap/oracle remains retained.
- Call resolution remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-115`
