# 2271 MIRBUILDER-EXACT-SEED-BACKEND-ROUTE-LABEL-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001

Status: Closed
Date: 2026-07-04

## Decision

Add the hand-authored `.hako` implementation for exact seed backend route
tag/source-field formatting.

## Evidence

Implementation:
`lang/src/compiler/lib/exact_seed_backend_route_label_formatter.hako`

## Boundary

The implementation formats scalar vocabulary only. It does not migrate exact
seed backend route selection, exact seed payload route matching, backend
lowering, or MIR mutation.

## Next

`MIRBUILDER-EXACT-SEED-BACKEND-ROUTE-LABEL-FORMATTER-PARITY-GATE-001`
