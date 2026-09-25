# MIR-CALL-R7-CALLER-ZERO-D0 — caller-zero retirement selection

Status: closed__nosafeslice__2026-09-25
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

## Decision (closed 2026-09-25): NoSafeSlice

No bounded deletion set satisfies writer/reissuer/reader zero. The
`LegacyCallV0` type, its `func` slot, and every repair path are kept
alive by two deliberately quarantined ingress minters:

- `emit_value_unified` → `Callee::Value` under
  `NYASH_BUILDER_UNIFIED_CALL=0`
  (`calls/unified_emitter/compat_entrypoints.rs:12-27`,
  `emit.rs:189-193`, flag `call_unified.rs:12-19` — supported opt-out).
- `mir_json_v0` `boxcall` → `Callee::Method` receiverless
  (`mir_json_v0/module.rs:439-475`; strict/dev pre-rejects at
  `selfhost/json.rs:101-146`, release/default keeps it and
  `json_artifact/mir_loader.rs` ingests unconditionally).

Per-candidate verdicts (worker census):

- A (Global variant arms): writers/reissuers caller-zero, but ≥5
  production owner files read legacy-Global specifically
  (`global_call_route_plan`, `ordered_map_origin_plan`,
  `generic_method_route_plan/*`, route metadata) plus all the named
  stops — deleting Global-specific arms converts fail-fast stops into
  silent fallthroughs. Contract change, not cleanup.
- B (`func` slot): load-bearing for the Value minter, `callee:None`
  rendering/validation, published-view named carrier errors.
- C (repair paths): `callsite_canonicalize` (runs on every compiled
  module, 4+ schedule sites), `array_element_write`, `builder_emit`,
  `edge_rematerialization`, `joinir_id_remapper*` — all live/shared.
- D (`emit_legacy_call`): live compat dispatcher emitting canonical
  instructions; only its Value arm reaches the minter. Not dead code.

Reopen trigger (observable): when BOTH quarantined ingress minters are
retired (unified-off corridor closed AND v0-boxcall release-mode
ingress stopped), the reader census flips — rerun this D0 and bounded
deletion slices become safe. Until then R7 aggregate deletions remain
frontier-paused; do not reopen by removing live readers.

Surviving required readers (by owner): compat ingress writers
(`compat_entrypoints`, `mir_json_v0/module`), reissuers
(`callsite_canonicalize`, `array_element_write`, `builder_emit`,
`edge_rematerialization`, `joinir_id_remapper*`), analysis/metadata
(`global_call_route_plan`, `generic_method_route_plan/*`,
`ordered_map_origin_plan`, `same_module_body_shape`, string-corridor,
`query`, `value_consumer`, `value_representation_fact`, ssa),
contract/view (`published_backend_view`, `backend_capability`),
transport (`mir_json_emit`), backend stops, structural accessors.
~250 test files construct/match the variant — updated with the type
when it eventually dies, not production blockers.

## Exit

- [x] One accepted bounded deletion set with caller-zero evidence —
  or `NoSafeSlice` with reopen trigger. → NoSafeSlice recorded.
- [x] Per-asset classification: caller-zero vs still-minted vs
  compat-read.
- [x] Named next execution row: the next design row selects one
  bounded ingress-minter retirement boundary
  (`MIR-CALL-INGRESS-MINTER-RETIRE-D0`), the only path that unblocks
  R7.
