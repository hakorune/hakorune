# 2487 MIRBUILDER-ARRAY-TEXT-OBSERVER-ARG-REPR-CONST-UTF8-CLASSIFIER-PARITY-GATE-001

Status: Completed
Date: 2026-07-04

## Decision

Add a dedicated `.hako` EXE parity gate for
`array_text_observer_arg_repr_const_utf8_classifier`.

## Gate

`tools/checks/rust_lifecycle_mirbuilder_array_text_observer_arg_repr_const_utf8_classifier_parity_gate.sh`

## Acceptance

The gate compiles a temporary `.hako` app through the EXE path and compares the
2 fixture rows against the Rust oracle booleans.

## Next

`MIRBUILDER-ARRAY-TEXT-OBSERVER-ARG-REPR-CONST-UTF8-CLASSIFIER-HAKO-ADOPTION-DECISION-001`
