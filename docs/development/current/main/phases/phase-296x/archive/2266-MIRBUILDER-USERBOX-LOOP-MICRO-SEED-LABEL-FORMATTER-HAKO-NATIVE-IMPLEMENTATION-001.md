# 2266 MIRBUILDER-USERBOX-LOOP-MICRO-SEED-LABEL-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001

Status: Closed
Date: 2026-07-04

## Decision

Add the hand-authored `.hako` implementation for the UserBox loop micro seed
kind/proof label formatter.

## Evidence

Implementation:
`lang/src/compiler/lib/userbox_loop_micro_seed_label_formatter.hako`

## Boundary

The implementation formats scalar vocabulary only. It does not migrate UserBox
loop micro seed matching, thin-entry selection, backend helper emission, or MIR
mutation.

## Next

`MIRBUILDER-USERBOX-LOOP-MICRO-SEED-LABEL-FORMATTER-PARITY-GATE-001`
