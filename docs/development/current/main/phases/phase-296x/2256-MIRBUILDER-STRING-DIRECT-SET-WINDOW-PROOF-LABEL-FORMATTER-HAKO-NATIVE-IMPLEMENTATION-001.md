# 2256 MIRBUILDER-STRING-DIRECT-SET-WINDOW-PROOF-LABEL-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001

Status: Closed
Date: 2026-07-04

## Decision

Add the hand-authored `.hako` implementation for the string direct-set window
proof label formatter.

## Evidence

Implementation:
`lang/src/compiler/lib/string_direct_set_window_proof_label_formatter.hako`

## Boundary

The implementation formats scalar vocabulary only. It does not migrate string
direct-set window matching, string corridor matching, backend lowering, or MIR
mutation.

## Next

`MIRBUILDER-STRING-DIRECT-SET-WINDOW-PROOF-LABEL-FORMATTER-PARITY-GATE-001`
