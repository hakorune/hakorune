# MIR-CALL-R7-CALLER-ZERO-D0 — caller-zero retirement selection

Status: selected__2026-09-25
Date: 2026-09-25
Parent: MIR-CALL-R6S3-SELECTED-DYNAMIC-LEGACY-STOP-S3 (landed 2026-09-25)
Owner card:
  docs/development/current/main/investigations/mir-call-compatibility-retire-r7-d0-2026-09-11.md
Authority: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
             section "R6/R7 migration program" row R7.

## Task

R7 permits physical deletion of `LegacyCallV0`, `func`, optional
callee/receiver, repair, and family-only assets **only after production
writer/reissuer/reader zero**. Select exactly one bounded asset (or
asset family) whose writer/reissuer/reader census is provably zero on
production lanes, and name its deletion set — or record `NoSafeSlice`
with an observable reopen trigger. design_stop: census, premise audit,
six-line Decision only.

## Census surface

Live minters that still bound R7 scope:

- `emit_value_unified` (`Callee::Value`, `NYASH_BUILDER_UNIFIED_CALL=0`
  quarantined ingress).
- `mir_json_v0` `boxcall` (`Callee::Method` receiverless quarantined
  ingress).

Reader families to classify (from prior censuses):

- Production stops (already landed): mir_interpreter dispatch, WASM/AOT
  pre-codegen, `PublishedMirBackendView` Global arm, selected-Dynamic
  carrier scan, `validate_published_ingress`, v1 importer.
- Compat-tolerant analysis reads (do not issue canonical identity):
  `query.rs`, `value_consumer.rs`, `callsite_canonicalize`,
  `string_corridor_relation`, `value_representation_fact`,
  `joinir_id_remapper*`, accessors/display.
- Quarantined egress projections: `calls_compat_v0.rs` v0 wire shapes.
- `func` slot consumers: `emit_call_with_callee_v0` receiverless reuse,
  `reject_selected_dynamic` carrier scan, verifier/analysis reads.

Candidate bounded assets (to be proven or declined per callee-kind):

- `LegacyCallV0{Callee::Global}` variant support — no production minter
  since R6-S1; all production readers stop. Deletion of the Global
  variant's match arms/fixtures may be separable per-callee.
- `func` slot retirement for callee-carrying rows.
- Repair paths that exist only to reshape legacy rows.

## Constraints

Deletion requires: caller switch or named stop already landed, required
verification in place, target edge caller-zero evidence, and the actual
deletion set inside the approved scope. Do not widen a census finding
into an undeclared aggregate deletion. Quarantined ingress minters stay
until their own migration rows select them.

## Exit

- [ ] One accepted bounded deletion set with caller-zero evidence
  (writer/reissuer/reader per production lane), authority, and
  verification named — or `NoSafeSlice` with reopen trigger.
- [ ] Per-asset classification: caller-zero vs still-minted vs
  compat-read.
- [ ] Named next execution row, or an explicit pause.
