# 2616 MIRBUILDER-ARRAY-STRING-LEN-WINDOW-MODE-FORMATTER-PARITY-GATE-001

Status: Completed
Date: 2026-07-04

## Decision

Add a dedicated `.hako` EXE parity gate for
`array_string_len_window_mode_formatter`.

## Gate

`tools/checks/rust_lifecycle_mirbuilder_array_string_len_window_mode_formatter_parity_gate.sh`

## Acceptance

The gate compiles a temporary `.hako` app through the EXE path and compares
the 3 fixture rows against the Rust oracle strings.

## Next

`MIRBUILDER-ARRAY-STRING-LEN-WINDOW-MODE-FORMATTER-HAKO-ADOPTION-DECISION-001`
