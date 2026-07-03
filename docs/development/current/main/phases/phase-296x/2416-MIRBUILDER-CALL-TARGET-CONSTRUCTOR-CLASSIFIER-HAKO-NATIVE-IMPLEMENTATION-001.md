# 2416 MIRBUILDER-CALL-TARGET-CONSTRUCTOR-CLASSIFIER-HAKO-NATIVE-IMPLEMENTATION-001

Status: Completed
Date: 2026-07-04

## Decision

Add the hand-authored `.hako` implementation for
`call_target_constructor_classifier`.

## Scope

The implementation maps CallTarget kind tokens to constructor boolean labels.

## Non-Claims

- Source Selfhost remains unclaimed.
- Call target resolution remains Rust.
- Closure capture handling, call lowering, and MIR mutation remain Rust.

## Next

`MIRBUILDER-CALL-TARGET-CONSTRUCTOR-CLASSIFIER-PARITY-GATE-001`
