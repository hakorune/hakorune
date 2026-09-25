# MIR-CALL-R7-CANONICALIZE-LEGACY-CLOSURE-ARM-DELETE-S8 — delete dead closure arm

Status: landed__2026-09-25
Date: 2026-09-25
Parent: MIR-CALL-R7-CALLER-ZERO-D3 (accepted, bounded)
Owner card:
  docs/development/current/main/investigations/mir-call-compatibility-retire-r7-d0-2026-09-11.md
Authority: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
             Call/R7 frontier.

## Slice

`src/mir/passes/callsite_canonicalize/pass.rs`: delete the
`LegacyCallV0{Some(Callee::Closure)}` rewrite arm (~10 lines match on
`classify_closure_call_shape` -> `NewClosure`) and its now-unused
imports (`classify_closure_call_shape`, `ClosureCallShape`).

Keep: `NewClosure` body-id externalization arm,
`rewrite_cfg_stable_receiver_operands`, the `LegacyCallV0{Global}`
no-op arm, and the catch-all.

## Fail-fast boundary

A residual `LegacyCallV0{Closure}` row falls through to `.. => 0` and
is still rejected by the interpreter (`Closure creation in VM`) and
the backend named-stops. Flip
`ncl0_rewrites_call_closure_to_newclosure` into a no-laundering pin
asserting `rewritten == 0` and the untouched legacy row.

## Verification

- `cargo check --profile quick` clean.
- `callsite_canonicalize` tests: flipped ncl0 pin green; remaining 12
  tests green.
- `mir_call_d1b_active_surface_guard` row registered in r6 helpers.
- `git diff --check`; pointer/lifecycle guards; file <800 lines.

## Exit

- [x] Deletion set exactly matches this card.
- [x] Pins + focused suites green; new reds classified.
- [x] Guard row registered and green; card/pointer/workstream synced.

## Evidence

- `cargo check --profile quick` clean (warnings are pre-existing).
- `cargo test --profile quick --lib callsite_canonicalize`: 14/14
  green, incl. `ncl0_residual_legacy_closure_call_is_never_silently_rewritten`.
- `mir_call_d1b_active_surface_guard.py`: S8 row delegated + ok.
- `current_state_pointer_guard.sh`: ok; `git diff --check`: clean;
  lifecycle inventory regenerated.
