# 2482 MIRBUILDER-ARRAY-TEXT-OBSERVER-ARG-REPR-KIND-FORMATTER-PARITY-GATE-001

Status: Completed
Date: 2026-07-04

## Decision

Add a dedicated `.hako` EXE parity gate for
`array_text_observer_arg_repr_kind_formatter`.

## Gate

`tools/checks/rust_lifecycle_mirbuilder_array_text_observer_arg_repr_kind_formatter_parity_gate.sh`

## Acceptance

The gate compiles a temporary `.hako` app through the EXE path and compares the
2 fixture rows against the Rust oracle labels.

## Next

`MIRBUILDER-ARRAY-TEXT-OBSERVER-ARG-REPR-KIND-FORMATTER-HAKO-ADOPTION-DECISION-001`
