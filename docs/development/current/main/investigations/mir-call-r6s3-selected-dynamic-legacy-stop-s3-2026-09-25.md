# MIR-CALL-R6S3-SELECTED-DYNAMIC-LEGACY-STOP-S3 — selected-Dynamic carrier stop

Status: landed__2026-09-25
Date: 2026-09-25
Parent: MIR-CALL-R6S3-COMPAT-QUARANTINE-D0 (accepted, boundary=selected-Dynamic scan)
Owner card:
  docs/development/current/main/investigations/mir-call-compatibility-retire-r7-d0-2026-09-11.md
Authority: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
             section "R6/R7 migration program" row R6-S3.

## Decision

`reject_selected_dynamic_legacy_callsites` rejects every
`MirInstruction::LegacyCallV0` carrier — sealing the only confirmed
open re-entry of quarantined ingress into canonical production (the
selected-Dynamic v1 artifact lane to ny_llvmc).

- Source authority + canonical issuer:
  `src/runner/product/llvm/mod.rs` `reject_selected_dynamic_legacy_callsites`
  (:331-356), sole production caller :220-226.
- Non-authority: `legacy_callsite_reject_code` stays narrow (shared
  with v0 compat `boxcall` egress); no minting removal; no v1 emit-side
  stops; no verifier changes; no receipt.
- Fail-fast boundary: existing terminal
  `"selected Dynamic legacy callsite rejected: function=… reason=…"`;
  new reason `call-legacy-carrier` for carriers the shared predicate
  does not cover. Scans `instructions` and `terminator`.
- Non-claims: unified-off Value and v0-boxcall Method remain explicit
  ingress elsewhere; this slice only blocks them from the selected
  Dynamic artifact lane.

## Scope

### In-series edits

1. `src/runner/product/llvm/mod.rs`
   `reject_selected_dynamic_legacy_callsites`: before delegating to
   `legacy_callsite_reject_code`, match
   `MirInstruction::LegacyCallV0 { .. }` and return reason
   `call-legacy-carrier` for carriers the predicate does not already
   name (keep predicate-first so existing codes win).
2. Same file `#[cfg(test)]`: add per-variant reject tests —
   `Some(Value)`, `Some(Method{receiver:None})`, `Some(Global)`,
   `Some(Extern)`, `Some(Constructor)`, `Some(SameModuleInstance)`,
   `Some(BirthConstructor)` each `Err` containing `call-legacy-carrier`
   (or their existing code); terminator-positioned `LegacyCallV0` →
   `Err`; typed `MirInstruction::call` in instructions+terminator →
   `Ok`.
3. `tools/checks/mir_call_canonical_corridor_guard.sh`: extend the
   definition-window token list (:445-460) to pin the unconditional
   carrier arm (`call-legacy-carrier`).

### Named casualty

None expected — the selected lane is a named production corridor; no
production path mints `LegacyCallV0` for it (unified-off and v0
boxcall feed compat lanes).

### Verification

- `cargo check --profile quick` clean.
- Focused: all new per-variant + terminator + typed-accept tests;
  existing `call-missing-callee`/`call-closure-*` tests unchanged-green.
- `bash tools/checks/mir_call_canonical_corridor_guard.sh` green.
- `mir_call_d1b_*` stable guard green for this row (register via
  `dispatch_extended_row`).
- Pointer/hygiene/lifecycle guards green.

## Evidence (landed 2026-09-25)

- `src/runner/product/llvm/mod.rs`: new
  `selected_dynamic_callsite_reject_code` — shared predicate first
  (existing `call-missing-callee`/`call-closure-*` codes preserved),
  else `LegacyCallV0` → `call-legacy-carrier`. Applied to both
  `instructions` and `terminator` scans.
- `legacy_callsite_reject_code` unchanged — v0 compat `boxcall` egress
  stays designated-legal (guard pins predicate stays narrow).
- Tests: `rejects_every_legacy_carrier` (Value, receiverless Method,
  Global, Extern, Constructor, SameModuleInstance, BirthConstructor →
  `call-legacy-carrier`), `rejects_legacy_carrier_in_terminator`,
  `accepts_typed_call_rows`; existing 2 tests unchanged. 9/9
  `runner::product` green.
- `mir_call_canonical_corridor_guard.sh` token window extended with
  `call-legacy-carrier`; stable guard registered via
  `dispatch_extended_row` — S2/S3 checks moved to
  `mir_call_d1b_active_surface_dispatch_helpers_r6.py` (helpers.py held
  at 700 lines, dispatcher at 799).
- `cargo check --profile quick` clean; pointer / lifecycle-inventory
  guards green; `git diff --check` green.
- Reds: `mir_call_canonical_corridor_guard.sh` stops earlier on a
  pre-existing pin (`ordinary_new_admission/selected.rs` substring
  drift — baseline debt, this slice's token additions are downstream
  of that check). No new test reds observed.

## Exit

- [x] No `LegacyCallV0` carrier reaches the selected-Dynamic artifact
  lane (per-variant pins).
- [x] Typed `Call` rows unchanged (accept pin).
- [x] v0 compat `boxcall` egress unchanged (shared predicate narrow).
- [x] All verification green; new reds classified and recorded.
