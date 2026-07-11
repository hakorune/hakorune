# 2386 MIRBUILDER-EMIT-GUARD-CALLEE-LABEL-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001

Status: Completed
Date: 2026-07-04

## Decision

Add the hand-authored `.hako` implementation for
`emit_guard_callee_label_formatter`.

## Boundary

The implementation accepts scalar kind/payload tokens and returns a single
Rust-oracle diagnostic label.

## Non-Claims

- No generated artifact as native edit authority.
- No emit-guard scope validation migration.

## Next

`MIRBUILDER-EMIT-GUARD-CALLEE-LABEL-FORMATTER-PARITY-GATE-001`
