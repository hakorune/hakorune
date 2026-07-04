# 2802 MIRBUILDER-DIRECT-STEP-PLACEMENT-CLASSIFIER-HAKO-ADOPTION-DECISION-001

Status: Completed
Date: 2026-07-04

## Decision

Adopt `direct_step_placement_classifier` as a narrow HakoAdopted Rust-oracle parity pilot owner after the green 4-row `.hako` EXE parity gate.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-direct-step-placement-classifier-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/direct_step_placement_classifier.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_direct_step_placement_classifier_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-direct-step-placement-classifier-hako-adoption-decision-v0.json
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Rust bootstrap/oracle remains retained.
- Generic-loop direct step placement classification remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-142`
