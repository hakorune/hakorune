# MIR-CALL-R7-CANONICALIZE-LEGACY-METHOD-ARM-DELETE-S6 — delete dead repair arm

Status: landed__2026-09-25
Date: 2026-09-25
Parent: MIR-CALL-R7-CALLER-ZERO-D1 (accepted, bounded)
Owner card:
  docs/development/current/main/investigations/mir-call-compatibility-retire-r7-d0-2026-09-11.md
Authority: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
             Call/R7 frontier.

## Slice

Delete the `callsite_canonicalize` `LegacyCallV0{Method}` rewrite arm
and the plumbing that exists only for it:

- `src/mir/passes/callsite_canonicalize/pass.rs`:
  - the `LegacyCallV0{Some(Callee::Method{receiver:Some})}` arm
    (`known_user_box_name_from_value` gate + `method_call` rewrite).
  - `value_types` clone at the function loop and the
    `value_types`/`known_user_boxes` parameters of
    `canonicalize_callsite_instruction` (used only by that arm).
  - `collect_known_user_boxes` call and now-unused imports
    (`method_call`, `TypeCertainty`, `CalleeBoxKind`, `MirType`,
    `BTreeSet`, helpers import) — prune to exactly what remains used.
- `src/mir/passes/callsite_canonicalize/helpers.rs`: both functions
  become unreachable — delete the file and its `mod helpers;` line.

Keep: `NewClosure` body-id arm, `rewrite_cfg_stable_receiver_operands`,
the `LegacyCallV0{Closure}` arm, the `LegacyCallV0{Global}` no-op arm,
and the catch-all — all out of this slice's deletion set.

## Fail-fast boundary

After removal a hypothetical `LegacyCallV0{Method}` row falls through
to `.. => 0` (no silent laundering) and remains rejected by the
existing named-stops (`backend_capability`, interpreter
`[vm-reference/legacy-call/method-stopped]`, selected-Dynamic
`call-legacy-carrier`). Add one pin: a synthetic module carrying
`LegacyCallV0{Method}` with populated `value_types`/`user_box_decls`
returns `0` rewrites and leaves the row untouched.

## Verification (landed)

- `cargo check --profile quick` clean; `cargo test --profile quick
  --lib callsite_canonicalize`: 14/14 green.
- New no-laundering pin green:
  `ucm1_residual_legacy_method_call_is_never_silently_rewritten`
  (fixture keeps `value_types`/`user_box_decls` populated; pass returns
  0 and leaves the legacy row untouched).
- `selfhost::json` 6/6 green — includes the S5 stale-expectation fix
  (`release_mode_keeps_legacy_callsite_compat` now asserts the
  canonical `Call{Method}` row; discovered red by the D1 census, it
  was not in the baseline failures manifest).
- `mir_call_d1b_active_surface_guard` row
  `MIR-CALL-R7-CANONICALIZE-LEGACY-METHOD-ARM-DELETE-S6` registered in
  r6 helpers and green; pointer guard green; `git diff --check` clean;
  lifecycle inventory re-pinned.

## Exit

- [x] Deletion set exactly matches this card (Method arm +
      `value_types`/`known_user_boxes` plumbing + `helpers.rs` +
      `mod helpers;` — Closure arm, Global no-op, `func` slot, and all
      named-stop readers untouched).
- [x] Pins + focused suites green; new reds classified (none beyond
      the S5 stale test fixed here).
- [x] Guard row registered and green; card/pointer/workstream synced.
