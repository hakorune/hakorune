# MIR-CALL-R6S3-COMPAT-QUARANTINE-D0 — compatibility quarantine selection

Status: accepted__2026-09-25
Date: 2026-09-25
Parent: MIR-CALL-R6S2-PUBLISHED-VIEW-GLOBAL-STOP-S2 (landed 2026-09-25)
Owner card:
  docs/development/current/main/investigations/mir-call-compatibility-retire-r7-d0-2026-09-11.md
Authority: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
             section "R6/R7 migration program" row R6-S3.

## Task

Select exactly one bounded R6-S3 outer-compatibility boundary — where
quarantined ingress could re-enter canonical production — and name the
fail-fast terminal, or record `NoSafeSlice` with an observable reopen
trigger. design_stop: census, premise audit, six-line Decision only.

## Census surface (from the R6-S2 worker record)

Live `LegacyCallV0` minters after S1/S2:

- `emit_value_unified` (`Callee::Value`) under
  `NYASH_BUILDER_UNIFIED_CALL=0` — quarantined ingress.
- `mir_json_v0` `boxcall` (`Callee::Method`, receiverless) — quarantined
  ingress.
- `LegacyCallV0{Global}`: no production minter; admitted only as a
  residual/stale row.

Residual real crossings flagged for this stage:

- Canonical-v1 JSON emit serializes a residual `LegacyCallV0` callee as
  `mir_call` (`emitters/calls.rs` compat projection owner
  `calls_compat_v0.rs`) — a legacy row can cross into v1 wire shape.
- `reject_selected_dynamic_legacy_callsites` covers only `callee:None`
  + Closure — other dynamic legacy callsite shapes may pass.
- Analysis readers (`mirror.rs`, `body_facts`, `function_value_map`,
  `callable_loop_facts`, `value_representation_fact`, `portable_*`)
  are compat-tolerant by design — not a re-entry boundary unless one
  feeds canonical issuance.

## Constraints (from the migration program)

JSON/JoinIR/unified-off stay explicit ingress only; they must not
re-enter canonical production. One boundary per row; finite caller list
before implementation; no fallback/retry; no widening by guesswork.

## Decision (accepted 2026-09-25)

**Widen `reject_selected_dynamic_legacy_callsites` to reject every
`MirInstruction::LegacyCallV0` carrier** — the last Rust-side gate on
the only confirmed open re-entry into canonical production.

- Confirmed re-entry: a pure-legacy module (unified-off
  `LegacyCallV0{Value}` or v0-boxcall `Method`) classifies
  `CanonicalTyped` in `try_new`, passes strict verify (verifier accepts
  `LegacyCallV0` as `Kept`), passes the narrow callee:None/Closure-only
  scan, and is serialized as canonical-v1 `mir_call` by
  `emit_mir_json_for_selected_dynamic_candidate` into `ny_llvmc`
  production — laundered indistinguishable from a typed call.
- Source authority + canonical issuer:
  `src/runner/product/llvm/mod.rs:331-356`
  `reject_selected_dynamic_legacy_callsites`; sole production caller
  `:220-226` (after `into_verified_module` strict fence, before the
  artifact lane `BoundaryExecutorBox::try_execute_selected_dynamic` →
  `emit_selected_dynamic` → v1 export).
- Fail-fast boundary: existing named terminal
  `"selected Dynamic legacy callsite rejected: function=… reason=…"`;
  new reason `call-legacy-carrier` for carriers the shared predicate
  does not cover. Covers `instructions` and `terminator`.
- Shared predicate unchanged: `legacy_callsite_reject_code`
  (`allowlists.rs:12-25`) stays narrow — it is shared with the v0
  compat emit lane where `Method`→`boxcall` is designated-legal. The
  unconditional carrier arm lives only in the selected lane.
- Finite callers: `mod.rs:222-226` production site; tests
  `mod.rs:509-540`; guard pins
  `tools/checks/mir_call_canonical_corridor_guard.sh`.
- Verification: per-variant reject tests (Value, receiverless Method,
  Global, Extern, Constructor, SameModuleInstance, BirthConstructor),
  a terminator-positioned reject, and the existing missing-callee +
  typed-accept tests green; corridor guard extended to pin the
  all-carrier arm.
- Non-claims: no minting removal (`emit_value_unified`, mir_json_v0
  boxcall stay as explicit ingress); no v1 emit-side carrier stops on
  compat lanes; no verifier tag changes; no shared-predicate widening;
  compat-tolerant analysis readers untouched.
- Smallest next slice:
  `MIR-CALL-R6S3-SELECTED-DYNAMIC-LEGACY-STOP-S3`.

## Worker census record (read-only)

- v1 emit laundering exists (`emitters/mod.rs:347-355` +
  `calls.rs:13-33` project `LegacyCallV0` + `callee:Some(_)` to
  `mir_call`), but no Rust re-import loop reaches the compiler; the v1
  importer stops call-like legacy rows at
  `mir-json-v1/legacy-call-stopped`. Sealing the selected-Dynamic scan
  upstream also seals the only canonical-v1 export feeding ny_llvmc.
- Published backend route already closed (`validate_published_ingress`
  rejects any `LegacyCallV0`); C-frame emission closed; VM/WASM
  dispatch closed; verifier accepts `LegacyCallV0` as `Kept` by design
  (admission-level stops are the authority).

## Exit

- [x] One accepted bounded quarantine boundary with source authority,
  canonical issuer, fail-fast boundary, finite callers, and
  verification named — or `NoSafeSlice` with reopen trigger.
- [x] Re-entry classification for each crossing listed above:
  canonical re-entry vs explicit ingress vs compat-tolerant read.
- [x] Named next execution row:
  `MIR-CALL-R6S3-SELECTED-DYNAMIC-LEGACY-STOP-S3`.
