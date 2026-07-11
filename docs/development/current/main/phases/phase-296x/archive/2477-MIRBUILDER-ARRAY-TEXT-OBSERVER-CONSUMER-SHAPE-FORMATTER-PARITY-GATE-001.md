# 2477 MIRBUILDER-ARRAY-TEXT-OBSERVER-CONSUMER-SHAPE-FORMATTER-PARITY-GATE-001

Status: Completed
Date: 2026-07-04

## Decision

Add a dedicated `.hako` EXE parity gate for
`array_text_observer_consumer_shape_formatter`.

## Gate

`tools/checks/rust_lifecycle_mirbuilder_array_text_observer_consumer_shape_formatter_parity_gate.sh`

## Acceptance

The gate compiles a temporary `.hako` app through the EXE path and compares the
2 fixture rows against the Rust oracle labels.

## Next

`MIRBUILDER-ARRAY-TEXT-OBSERVER-CONSUMER-SHAPE-FORMATTER-HAKO-ADOPTION-DECISION-001`
