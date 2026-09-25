# MIR-CALL-INGRESS-MINTER-RETIRE-D0 — ingress minter retirement selection

Status: accepted__2026-09-25
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

## Decision (accepted 2026-09-25)

**Promote `emit_value_unified` to the canonical mint** — minter #1
retires its legacy product: `compat_entrypoints.rs` emits
`MirInstruction::call(dst, Callee::Value(func_val), args, IO)` instead
of `LegacyCallV0`. Full-corridor retirement is NOT bounded (the `=0`
flag's named owner is the `env.mirbuilder.emit` Phase-0 wire contract,
a product-level feature); retiring the legacy mint inside it is.

- Source authority + canonical issuer:
  `src/mir/builder/calls/unified_emitter/compat_entrypoints.rs:20-26`,
  sole caller `emit.rs:189-193` (`emit_legacy_call` Value arm); the
  canonical mint is `MirInstruction::call` — identical to the
  unified-on `CallTarget::Value` output.
- Wire parity: v0 emit is callee-driven
  (`calls_compat_v0.rs:141-145` — `Callee::Value` →
  `{"op":"call","callee":{"type":"Value","value":v}}`); `func` is never
  emitted when a callee exists → byte-identical v0/v1 wire.
- Fail-fast boundary: typed `Call{Value}` hits
  `[vm-reference/canonical-call] only Global targets are admitted`
  (interpreter) and `UnsupportedBeforeObject` (published view,
  `Some(Callee::Value(_)) if canonical_call`) — same terminal class as
  the legacy row; interpreter tag string changes, flagged residual.
- Finite callers: `emit.rs:189-193` only; reachable producer
  `effect_emission.rs:221` (`CoreEffectPlan::ValueCall`);
  `exprs_call.rs` already stops `CallTarget::Value` before descent
  under `=0`.
- `=0` production consumers: `env.mirbuilder.emit` guard +
  `vm_hako_caps` reference lane — neither exercises value calls; no
  suite manifest runs a `=0` value-call fixture.
- Smallest next slice:
  `MIR-CALL-UNIFIED-OFF-VALUE-MINT-PROMOTE-S4`.
- Non-claims: does NOT retire `=0`/`emit_legacy_call`/`func` slot/
  `LegacyCallV0` type — the boxcall ingress minter keeps them alive
  (R7 reopen trigger unchanged: boxcall is the only v0-ingestable
  method carrier, coupled to Phase-0 wire contract + vm-hako
  reference validators — a wire-format deprecation requiring a
  product decision, not a bounded slice).

## Exit

- [x] One accepted bounded ingress-minter retirement with source
  authority, caller-switch evidence, named terminal, finite callers,
  and verification — or `NoSafeSlice` with reopen trigger.
- [x] For each candidate: production-lane reachability evidence
  (which env/mode/format actually reaches it).
- [x] Named next execution row:
  `MIR-CALL-UNIFIED-OFF-VALUE-MINT-PROMOTE-S4`.
