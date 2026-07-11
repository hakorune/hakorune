# 2541 MIRBUILDER-ARRAY-TEXT-OBSERVER-EXECUTOR-CONSUMER-CAPABILITY-FORMATTER-PARITY-GATE-001

Status: Completed
Date: 2026-07-04

## Decision

Add a dedicated `.hako` EXE parity gate for
`array_text_observer_executor_consumer_capability_formatter`.

## Gate

`tools/checks/rust_lifecycle_mirbuilder_array_text_observer_executor_consumer_capability_formatter_parity_gate.sh`

## Acceptance

The gate compiles a temporary `.hako` app through the EXE path and compares
the 3 fixture rows against the Rust oracle strings.

## Next

`MIRBUILDER-ARRAY-TEXT-OBSERVER-EXECUTOR-CONSUMER-CAPABILITY-FORMATTER-HAKO-ADOPTION-DECISION-001`
