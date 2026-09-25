# MIR-CALL-INGRESS-MINTER-RETIRE-D0 — ingress minter retirement selection

Status: selected__2026-09-25
Date: 2026-09-25
Parent: MIR-CALL-R7-CALLER-ZERO-D0 (closed NoSafeSlice 2026-09-25)
Owner card:
  docs/development/current/main/investigations/mir-call-compatibility-retire-r7-d0-2026-09-11.md
Authority: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
             section "R6/R7 migration program" row R7 reopen trigger.

## Task

R7 closed `NoSafeSlice`: the `LegacyCallV0` type and `func` slot stay
alive only because two quarantined ingress minters still mint it.
Select exactly ONE bounded ingress-minter retirement boundary with
caller-switch evidence and a named terminal — or record `NoSafeSlice`
with an observable reopen trigger. design_stop: census, premise audit,
six-line Decision only.

## Census surface

Candidate A — `NYASH_BUILDER_UNIFIED_CALL=0` opt-out corridor:

- Flag: `src/mir/builder/calls/call_unified.rs:12-19` (default ON).
- Minter: `compat_entrypoints.rs:12-27` (`Callee::Value` legacy row).
- Dispatcher: `emit_legacy_call` (`emit.rs:132-210`) — emits canonical
  instructions for every target except the Value arm reaching the
  minter.
- Questions: is the off-corridor exercised by any production/CI lane,
  smoke, or documented fallback? Who calls
  `is_unified_call_enabled()==false` today? Is the flag a supported
  product mode or a retired-in-practice escape hatch? Finite callers
  before any retirement.

Candidate B — v0 `boxcall` receiverless Method ingress:

- Minter: `src/runner/mir_json_v0/module.rs:439-475` (`"boxcall"` op →
  `LegacyCallV0{Method}`).
- Gates: strict/dev pre-reject at `selfhost/json.rs:101-146`
  (`callsite-retire:legacy-boxcall`); release/default accepts;
  `json_artifact/mir_loader.rs` ingests unconditionally.
- Reissuers downstream: `builder_emit.rs` receiver materialization and
  `callsite_canonicalize` convert Method rows to canonical `Call`.
- Questions: is receiverless-boxcall a wire-format compat feature with
  a named consumer, or a stale shape already converted downstream?
  Which consumers actually depend on the minted legacy Method row
  (vs the canonical reissue)? Is a release-mode stop a wire-format
  deprecation requiring product evidence?

## Constraints

- A retirement selects ONE minter, names its callers, proves each
  caller switched or stopped, and lands the named terminal before any
  deletion.
- v0 `boxcall` is wire-format ingress: its retirement must not widen
  into changing v0 `externcall` (already canonical) or other v0 ops.
- Unified-off retirement must not remove `emit_legacy_call`'s
  canonical emission paths.
- No fallback/retry; named stop only; finite caller list first.

## Exit

- [ ] One accepted bounded ingress-minter retirement with source
  authority, caller-switch evidence, named terminal, finite callers,
  and verification — or `NoSafeSlice` with reopen trigger.
- [ ] For each candidate: production-lane reachability evidence
  (which env/mode/format actually reaches it).
- [ ] Named next execution row, or an explicit pause.
