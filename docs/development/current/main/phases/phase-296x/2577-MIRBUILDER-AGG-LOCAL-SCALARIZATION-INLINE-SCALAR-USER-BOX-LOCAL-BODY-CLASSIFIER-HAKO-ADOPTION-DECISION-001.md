# 2577 MIRBUILDER-AGG-LOCAL-SCALARIZATION-INLINE-SCALAR-USER-BOX-LOCAL-BODY-CLASSIFIER-HAKO-ADOPTION-DECISION-001

Status: Completed
Date: 2026-07-04

## Decision

Adopt `agg_local_scalarization_inline_scalar_user_box_local_body_classifier`
as a narrow HakoAdopted Rust-oracle parity pilot owner after the 4-row
`.hako` EXE parity gate is green.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-agg-local-scalarization-inline-scalar-user-box-local-body-classifier-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/agg_local_scalarization_inline_scalar_user_box_local_body_classifier.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_agg_local_scalarization_inline_scalar_user_box_local_body_classifier_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-agg-local-scalarization-inline-scalar-user-box-local-body-classifier-hako-adoption-decision-v0.json
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Rust bootstrap/oracle remains retained.
- Thin-entry route matching and observer contract handling remain Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-098`
