# 2606 MIRBUILDER-ARRAY-TEXT-COMBINED-REGION-EFFECT-FORMATTER-PARITY-GATE-001

Status: Completed
Date: 2026-07-04

## Decision

Add a dedicated `.hako` EXE parity gate for `array_text_combined_region_effect_formatter`.

## Gate

`tools/checks/rust_lifecycle_mirbuilder_array_text_combined_region_effect_formatter_parity_gate.sh`

## Acceptance

The gate compiles a temporary `.hako` app through the EXE path and compares
the 4 fixture rows against the Rust oracle strings.

## Next

`MIRBUILDER-ARRAY-TEXT-COMBINED-REGION-EFFECT-FORMATTER-HAKO-ADOPTION-DECISION-001`
