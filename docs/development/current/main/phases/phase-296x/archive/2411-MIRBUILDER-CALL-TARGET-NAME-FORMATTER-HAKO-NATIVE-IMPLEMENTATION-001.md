# 2411 MIRBUILDER-CALL-TARGET-NAME-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001

Status: Completed
Date: 2026-07-04

## Decision

Add the hand-authored `.hako` implementation for `call_target_name_formatter`.

## Scope

The implementation maps CallTarget kind/name tokens to debug labels.

## Non-Claims

- Source Selfhost remains unclaimed.
- Call target resolution remains Rust.
- Closure capture handling, call lowering, and MIR mutation remain Rust.

## Next

`MIRBUILDER-CALL-TARGET-NAME-FORMATTER-PARITY-GATE-001`
