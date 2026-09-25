# MIR-CALL-R6S1-GLOBAL-PRODUCER-COHORT-S1 — Global cohort migration

Status: landed__2026-09-25
Date: 2026-09-25
Parent: MIR-CALL-R6S1-COHORT-SELECTION-D0 (accepted, cohort=Global)
Owner card:
  docs/development/current/main/investigations/mir-call-compatibility-retire-r7-d0-2026-09-11.md
Authority: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
             section "R6/R7 migration program" (~line 127) row R6-S1.

## Decision

Migrate the `Callee::Global` producer cohort: after this series, no
production edge can mint `LegacyCallV0{Global}`. One Promote, one
Stop, two deletes — all within the tuple accepted by the D0.

- Source authority + canonical issuer: `CanonicalGlobalTargetV1`
  catalog/builtin issuers; `MirCall` remains the sole carrier minted
  by `hakorune_mir_defs` schema.
- Non-authority: no receipt, no new guard, no `CallV2`, no semantic
  widening. OFF-mode (`NYASH_MIR_UNIFIED_CALL=0`) keeps ingress
  functionality via the typed carrier + compat transport.
- Fail-fast boundary: the ordinary-new legacy birth edge becomes a
  named `[freeze:contract]` terminal; malformed-arity or missing-claim
  shapes cannot reach typed emission.
- Smallest next slice: this row only.
- Non-claims: no parse-side (`mir_json_v0`), JoinIR, egress
  (`array_element_write`) or Value-arm changes — different scope
  (R6-S3 / separate cohorts).

## Scope

### In-series edits

1. **Promote** — `src/mir/builder/calls/emit.rs` `CallTarget::Global`
   arm (~:177-181): emit typed `MirInstruction::call(dst,
   Callee::Global(target), args, EffectMask::IO)` instead of
   `emit_global_unified`. Preserve `finalize_args` ordering; no
   name_const mint, no `annotate_call_result_from_func_name` (typed
   callee carries the identity).
2. **Stop** — `src/mir/builder/ordinary_new_admission.rs` (~:104):
   the `use_lowered` `emit_legacy_call(None, CallTarget::Global(target),
   argv)` edge becomes a named terminal
   `[freeze:contract][ordinary-new/birth-global-legacy-stopped]`.
   Rationale recorded in the D0: emitted shape is arity-malformed for
   typed Global and `Callee::BirthConstructor` needs
   `OrdinaryNewAdmissionClaimV1` absent here by construction.
3. **Delete** — `emit_global_unified`
   (`unified_emitter/compat_entrypoints.rs:13-34`) and
   `make_name_const_result` (`unified_emitter/name_const.rs:6`);
   remove the `name_const` module declaration if nothing else uses it.
4. **Guard re-pin** — `tools/checks/lib/mir_call_d1b_active_surface_rows.py:629`:
   flip the ordinary-new no-claim compatibility writer check from
   "required present" to "required absent".
5. **JoinIR caveat re-check** — confirm no path under `src/mir/join_ir`
   (or `joinir_convert` shell) mints `LegacyCallV0{Global}` before
   applying the delete-set; if one is found, record it and return to
   `design_stop` rather than widening the row.

### Named casualty

Raw-lane `new Foo()` programs where a lowered `<Class>.birth/N`
symbol exists in headers now fail fast at admission instead of
emitting a malformed-arity `LegacyCallV0`. Sole prior live consumer:
llvmlite-compat egress. Recorded per the migration-red rule: any test
that turns red records path/owner/reason/successor/first-red/expiry.

### Verification

- `cargo check --profile quick` clean.
- Focused pins: `published_builtin_print_rejects_legacy_function_carrier`,
  `published_normal_build_selects_ordinary_callee_before_global_lookup`,
  `SelectedNormalUsesLegacyCallV0` admission tests,
  `legacy_global_call_rejects_before_legacy_dispatch`,
  `me_method_canonical_cutover_tests` all-not-legacy asserts,
  phase0-vm-golden canonical asserts.
- `bash tools/checks/mir_call_d1b_*` guard suite green after re-pin.
- Pointer/hygiene/lifecycle guards green.

## Exit

- [x] No production path mints `LegacyCallV0{Global}` (grep-verified:
  `emit.rs` Global arm emits typed `MirInstruction::call`; the
  ordinary-new no-claim edge is a named stop; the v0 parser only mints
  `Callee::Method`/`Extern`; remaining `Callee::Global` mentions are
  match-arm readers, the WASM preflight rejector, or `#[cfg(test)]`
  fixtures — R6-S3/R7 territory).
- [x] Stop terminal fires with the named tag on the raw-lane birth
  edge; ordinary-new claimed path (`Callee::BirthConstructor`)
  unchanged.
- [x] Delete-set removed; no dangling references (`emit_global_unified`,
  `make_name_const_result`, `mod name_const` all grep-absent).
- [x] All verification green; new reds classified and recorded.

## Landed evidence (2026-09-25)

- `cargo check --lib --tests --profile quick` clean (baseline warnings
  only; none from touched files).
- Focused pins: `published_builtin_print_rejects_legacy_function_carrier`,
  `published_builtin_print_is_typed_and_has_no_definition_lookup`,
  `selected_static_method_rejects_legacy_function_carrier`,
  `legacy_global_call_rejects_before_legacy_dispatch` (vm-reference),
  `canonical_snapshot_preserves_runtime_static_global_target`,
  `me_method_canonical_cutover_tests` (4), unified-call temporal
  witness tests — all green. The card's
  `published_normal_build_selects_ordinary_callee_before_global_lookup`
  has no verbatim test symbol; the equivalent lookup-ordering pins
  above ran instead.
- Stop-terminal evidence: the migration-red test
  `headerport_birth_presence_matches_legacy_newbox_branch` was
  repurposed to
  `headerport_birth_presence_stops_both_raw_lanes_before_legacy_writer`
  (legacy port and invocation port produce the identical named
  `[freeze:contract][ordinary-new/birth-global-legacy-stopped]` error).
  Green.
- Guard: the d1b active-surface stable guard now recognizes this row
  via `dispatch_extended_row` in
  `tools/checks/lib/mir_call_d1b_active_surface_dispatch_helpers.py`
  (the primary dispatch chain sits at the 799-line hard stop, so the
  arm delegates without growing it). The dormant `check_ordinary_new_i0`
  pins were flipped to require the stop tag and reject the retired
  writer. `mir_call_d1b_cataloged_affine_loan_lifecycle_guard.sh` green.
- Pointer/facade (129 exports, 215 modules)/hygiene/lifecycle guards
  and `git diff --check` green; lifecycle inventory re-pinned for the
  deleted `name_const.rs`.

### Wire-shape note

v0 transport is unchanged for Global calls: the v0 emitter projects
`Callee::Global` to `{"type":"Global","name":...}` and never consulted
the `func` slot for Global callees. The only removed wire element is
the dead name-`const` op that `emit_global_unified` used to mint —
exactly the delete-set the D0 approved.

### Red classification (d1b suite and call lane)

- `mir_call_d1b_cataloged_affine_loan_lifecycle_guard.sh` — was
  `unsupported current row` under every R6 queue row; now green via the
  row registration above (current-change fix, in-card).
- `mir_call_d1b_exact_target_child_guard.sh` — baseline debt: pins
  `PreparedRawOrdinaryFunctionCompletionV1::AppMainTargeted` in
  `src/mir/builder/calls/build.rs`, renamed by `d6ee865368` (qualified
  Main method arguments); this row touches none of the guarded files.
- `mir_call_d1b_index_none_direct_calls_guard.sh` — baseline debt: pins
  `resolved_semantics/resolver.rs` last touched 2026-08-29
  (`99bdfc856c`); untouched here.
- `mir_call_d1b_selected_normal_duplicate_projection_guard.sh` — green.
- Non-d1b call-lane guards observed red and classified baseline debt
  (all pin files untouched by this row):
  `mir_call_canonical_corridor_guard.sh` (selected.rs, last touched
  2026-09-19), `mir_call_global_target_b0_machine_census_guard.sh`
  (card TOML family_rows 18 vs manifest 17, last touched 2026-08-30),
  `mir_call_ingress_schema_lifecycle_guard.sh` (pins `latest_card_path`
  to the retired `mir-call-d1b-root-lineage-exact-target-loan-d0` TOML;
  stale since the pointer moved past that card),
  `mir_call_joinir_remapper_isolation_guard.sh` (remapper file last
  touched 2026-09-15).
- `phase29bq_hako_mirbuilder_phase0_pin_vm.sh` — baseline debt: fails
  upstream of this row at
  `[freeze:contract][static-call/legacy-fallback-retired] owner=env
  method=get arity=1` (`method_call_handlers.rs`, landed earlier via
  `474e8518b0`); the named casualty of this row is a different edge.
- The swept `cargo test` group (`new_box birth raw_compat
  local_statement global_call emit_unified_call call_target`) reported
  366 passed / 9 failed; all 9 failures are recorded in
  `tools/checks/manifests/cargo_lib_red_baseline.*` and untouched by
  this row.

## Migration-red disposition

- `headerport_birth_presence_matches_legacy_newbox_branch`
  (`src/mir/builder/recursive_child_lowering_rawport_header_tests.rs`):
  owner = this row; reason = the named casualty edge; successor = the
  stop-parity assert `headerport_birth_presence_stops_both_raw_lanes_
  before_legacy_writer`; first-red = this row's local run; expiry =
  permanent (the stopped edge is not reopenable).
