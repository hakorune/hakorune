# 2531 MIRBUILDER-ARRAY-TEXT-OBSERVER-EXECUTOR-CARRIER-FORMATTER-PARITY-GATE-001

Status: Completed
Date: 2026-07-04

## Decision

Add a dedicated `.hako` EXE parity gate for
`array_text_observer_executor_carrier_formatter`.

## Gate

`tools/checks/rust_lifecycle_mirbuilder_array_text_observer_executor_carrier_formatter_parity_gate.sh`

## Acceptance

The gate compiles a temporary `.hako` app through the EXE path and compares
the 1 fixture row against the Rust oracle string.

## Next

`MIRBUILDER-ARRAY-TEXT-OBSERVER-EXECUTOR-CARRIER-FORMATTER-HAKO-ADOPTION-DECISION-001`
