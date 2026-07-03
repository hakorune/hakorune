# 2251 MIRBUILDER-ARRAY-STRING-LEN-WINDOW-LABEL-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001

Status: Closed
Date: 2026-07-04

## Decision

Add the hand-authored `.hako` implementation for the array string len-window
mode/proof label formatter.

## Evidence

Implementation:
`lang/src/compiler/lib/array_string_len_window_label_formatter.hako`

## Boundary

The implementation formats scalar vocabulary only. It does not migrate array
string len-window matching, string corridor matching, backend lowering, or MIR
mutation.

## Next

`MIRBUILDER-ARRAY-STRING-LEN-WINDOW-LABEL-FORMATTER-PARITY-GATE-001`
