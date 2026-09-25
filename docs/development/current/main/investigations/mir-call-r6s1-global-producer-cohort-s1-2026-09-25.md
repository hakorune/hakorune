# MIR-CALL-R6S1-GLOBAL-PRODUCER-COHORT-S1 — Global cohort migration

Status: selected__2026-09-25
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

- [ ] No production path mints `LegacyCallV0{Global}` (grep-verified).
- [ ] Stop terminal fires with the named tag on the raw-lane birth
  edge; ordinary-new claimed path (`Callee::BirthConstructor`)
  unchanged.
- [ ] Delete-set removed; no dangling references.
- [ ] All verification green; new reds classified and recorded.
