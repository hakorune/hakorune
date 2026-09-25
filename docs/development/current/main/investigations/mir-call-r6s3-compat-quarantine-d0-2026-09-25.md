# MIR-CALL-R6S3-COMPAT-QUARANTINE-D0 — compatibility quarantine selection

Status: selected__2026-09-25
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

## Exit

- [ ] One accepted bounded quarantine boundary with source authority,
  canonical issuer, fail-fast boundary, finite callers, and
  verification named — or `NoSafeSlice` with reopen trigger.
- [ ] Re-entry classification for each crossing listed above:
  canonical re-entry vs explicit ingress vs compat-tolerant read.
- [ ] Named next execution row, or an explicit pause.
