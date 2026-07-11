# 2571 MIRBUILDER-AGG-LOCAL-SCALARIZATION-INLINE-SCALAR-USER-BOX-LOCAL-BODY-CLASSIFIER-PARITY-GATE-001

Status: Completed
Date: 2026-07-04

## Decision

Add a dedicated `.hako` EXE parity gate for
`agg_local_scalarization_inline_scalar_user_box_local_body_classifier`.

## Gate

`tools/checks/rust_lifecycle_mirbuilder_agg_local_scalarization_inline_scalar_user_box_local_body_classifier_parity_gate.sh`

## Acceptance

The gate compiles a temporary `.hako` app through the EXE path and compares
the 4 fixture rows against the Rust oracle strings.

## Next

`MIRBUILDER-AGG-LOCAL-SCALARIZATION-INLINE-SCALAR-USER-BOX-LOCAL-BODY-CLASSIFIER-HAKO-ADOPTION-DECISION-001`
