# MIR-CALL-R7-CANONICALIZE-LEGACY-GLOBAL-NOOP-ARM-DELETE-S9 — delete redundant Global arm

Status: landed__2026-09-25
Date: 2026-09-25
Parent: MIR-CALL-R7-CALLER-ZERO-D4 (accepted, bounded)
Owner card:
  docs/development/current/main/investigations/mir-call-compatibility-retire-r7-d0-2026-09-11.md
Authority: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
             Call/R7 frontier.

## Slice

`src/mir/passes/callsite_canonicalize/pass.rs`: delete the
`LegacyCallV0{callee: Some(Callee::Global(_))} => 0` arm (4 lines +
comment). The `LegacyCallV0{..} => 0` arm directly below catches the
same rows with identical observable behavior (`0` rewrites).

Keep: `NewClosure` externalization (NCL-1), the
`rewrite_cfg_stable_receiver_operands` call, the explicit
`LegacyCallV0{..} => 0` passthrough arm (documents that residual
legacy rows are never rewritten), and `Callee` import if still used.

## Fail-fast boundary

Residual `LegacyCallV0` rows are untouched by the pass and still hit
the interpreter `reject_legacy_call` stop and published-view named
errors. Add/keep a pin proving a residual `LegacyCallV0{Global}` row
is passed through unrewritten (`rewritten == 0` + row intact).

## Verification

- `cargo check --profile quick` clean.
- `callsite_canonicalize` suite green incl. new Global passthrough pin.
- `mir_call_d1b_active_surface_guard` row registered in r6 helpers.
- `git diff --check`; pointer/lifecycle guards; file <800 lines.

## Exit

- [x] Deletion set exactly matches this card.
- [x] Pins + focused suites green; new reds classified.
- [x] Guard row registered and green; card/pointer/workstream synced.

## Evidence

- `cargo check --profile quick` clean.
- `cargo test --profile quick --lib callsite_canonicalize`: 14/14
  green; existing `mcl5_*`/`mcl6_*` Global fixtures already pin the
  `rewritten == 0` passthrough for residual `LegacyCallV0{Global}`
  rows, so no new fixture was needed.
- `Callee` import removed from pass.rs (now unused); the explicit
  `LegacyCallV0{..} => 0` catch-all arm is retained to document the
  no-laundering contract.
- `mir_call_d1b_active_surface_guard.py`: S9 row delegated + ok.
- Pointer guard ok; `git diff --check` clean.
