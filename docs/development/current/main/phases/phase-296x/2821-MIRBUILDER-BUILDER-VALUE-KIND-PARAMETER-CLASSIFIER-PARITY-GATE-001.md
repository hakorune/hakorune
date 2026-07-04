# 2821 MIRBUILDER-BUILDER-VALUE-KIND-PARAMETER-CLASSIFIER-PARITY-GATE-001

Status: Completed
Date: 2026-07-05

## Decision

Add a dedicated `.hako` EXE parity gate for
`builder_value_kind_parameter_classifier`.

## Gate

`tools/checks/rust_lifecycle_mirbuilder_builder_value_kind_parameter_classifier_parity_gate.sh`

## Acceptance

The gate compiles a temporary `.hako` app through the EXE path and compares
the 6 fixture rows against the Rust oracle strings.

## Next

`MIRBUILDER-BUILDER-VALUE-KIND-PARAMETER-CLASSIFIER-HAKO-ADOPTION-DECISION-001`
