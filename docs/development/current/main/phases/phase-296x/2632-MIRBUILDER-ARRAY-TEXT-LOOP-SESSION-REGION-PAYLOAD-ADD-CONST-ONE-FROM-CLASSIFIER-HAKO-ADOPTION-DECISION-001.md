# 2632 MIRBUILDER-ARRAY-TEXT-LOOP-SESSION-REGION-PAYLOAD-ADD-CONST-ONE-FROM-CLASSIFIER-HAKO-ADOPTION-DECISION-001

Status: Completed
Date: 2026-07-04

## Decision

Adopt `array_text_loop_session_region_payload_add_const_one_from_classifier`
as a narrow HakoAdopted Rust-oracle parity pilot owner after the 4-row
`.hako` EXE parity gate is green.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-array-text-loop-session-region-payload-add-const-one-from-classifier-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/array_text_loop_session_region_payload_add_const_one_from_classifier.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_array_text_loop_session_region_payload_add_const_one_from_classifier_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-array-text-loop-session-region-payload-add-const-one-from-classifier-hako-adoption-decision-v0.json
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Rust bootstrap/oracle remains retained.
- Array-text loop-session planning remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-108`
