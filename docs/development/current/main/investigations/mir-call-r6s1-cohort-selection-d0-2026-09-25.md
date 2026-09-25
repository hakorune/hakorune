# MIR-CALL-R6S1-COHORT-SELECTION-D0 — select the R6-S1 producer cohort

Status: accepted__2026-09-25__cohort=Callee::Global
Date: 2026-09-25
Parent: MIR-CALL-R6S0-TRANSPORT-EMITTER-SPLIT-S0 (landed; R6-S0 complete)
Owner card:
  docs/development/current/main/investigations/mir-call-compatibility-retire-r7-d0-2026-09-11.md
Authority: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
             section "R6/R7 migration program" (~line 127) row R6-S1.

## Decision

Bounded design row — design only. Select the exact R6-S1 cohort:

```text
R6-S1  one canonical producer cohort:
       existing source authority -> mandatory MirCall -> one typed consumer;
       delete that cohort's old writer/reissuer in the same series.
```

Acceptance requires a complete Promote/Stop/Delete tuple for the
selected cohort: the existing source authority (issuer), the mandatory
`MirCall` edge it already produces or must produce, the single typed
consumer, the cohort's old writer/reissuer to delete in-series, the
finite affected caller set, and the verification method — all named
from already-landed owners, no new authority.

This card selects the cohort; it does not implement the migration.

## Scope

### Inventory

Design-stop census within the call-producer surface only:
`src/mir/builder/calls/` writers (`emit_unified_call`,
`emit_finalized_generic_call_v1`, `emit_legacy_call`,
`create_legacy_call`), `MirJsonV0Loader` repair/canonicalize sites,
JoinIR lowering call producers, and their typed consumers. Do not
repeat the whole-repository Call/R7 census.

### Refusal

No production code, fixtures, fallback, guard, or receipt. No
`CallV2`. Do not reopen parked lanes. Do not select a cohort whose
consumer set is not finite or whose old writer has live production
callers outside the series.

### Reopening

If no cohort has a complete tuple, record `NoSafeSlice` per cohort
with the missing element and an observable reopen trigger.

## Exit

An accepted Decision naming the R6-S1 cohort (issuer, MirCall edge,
typed consumer, delete-set, finite callers, verification) or
`NoSafeSlice` with per-cohort missing elements and reopen triggers.
On acceptance, work_mode returns to `fast` for the bounded cohort
row and `next_execution_card` names it.

## Worker census result (2026-09-25, integrated)

Complete production `LegacyCallV0` writer inventory (non-test, src/):

1. `UnifiedCallEmitterBox::emit_global_unified`
   (`unified_emitter/compat_entrypoints.rs:13-34`) — Global old writer,
   reached by `ordinary_new_admission.rs:104` in DEFAULT mode plus 3
   ambient-OFF dispatches (`emit.rs:27/65/82`).
2. `emit_value_unified` (`compat_entrypoints.rs:37-52`) — Value old
   writer, ambient-OFF only (zero default-mode reach).
3. `project_module_to_legacy_calls` (`array_element_write.rs:251-323`)
   — llvmlite egress whole-module projection.
4. MirJsonV0 `"boxcall"` arm (`mir_json_v0/module.rs:439-475`) —
   ingress constructor, resolved-D0 retained compat (R6-S3 scope).
5. `joinir_id_remapper.rs:301-313` — id reissuer (reader, not source
   producer).

No `LegacyCallV0{Extern|BirthConstructor|SameModuleInstance|
Constructor|Closure}` writer exists anywhere — those classes are
canonical-only already. `create_legacy_call` has no definition or
callers (historical name).

## Accepted Decision — cohort `Callee::Global`

The only cohort with a complete tuple AND a live default-mode old
writer. Tuple:

1. **Source authority**: `CanonicalGlobalTargetV1` catalog/builtin
   issuers (`print_stmt.rs:289`, `emission_port.rs:217`,
   `effect_emission.rs:206`, `method_call_terminal.rs:332/387/403`,
   `static_result_publication.rs:205/236`, `physical_bridge.rs:121`,
   `function_call_preflight_route.rs:471-511` -> `build.rs:349-362`).
2. **Mandatory MirCall**: already unconditional for every Global
   issuer in default mode (`emit_finalized_generic_call_v1` ->
   `MirInstruction::call`, `physical_terminal.rs:91-93`; cataloged
   `build.rs:355`). The series makes it structural by deleting the
   last edge that can mint `LegacyCallV0{Global}`.
3. **One typed consumer**: `PublishedMirBackendView` Global validators
   (`published_backend_view.rs:511-695`) -> `PublishedStaticMethodCFrameV1`;
   VM reference `execute_global_target`
   (`handlers/mod.rs:171-179` -> `calls/global.rs:14-45`). Legacy
   carriers are already hard errors on this lane
   (`SelectedNormalUsesLegacyCallV0`, `*UsesLegacyFunctionCarrier`).
4. **In-series dispositions**:
   - Promote: `emit.rs` `CallTarget::Global` arm (:177-181) -> typed
     `MirInstruction::call(dst, Callee::Global(target), args, IO)`
     (consistent with sibling arms already emitting typed shapes;
     OFF-mode v0 wire output is preserved by the compat transport,
     which projects typed callees — verified landed in S0-B).
   - Stop: `ordinary_new_admission.rs:104` `use_lowered` birth edge ->
     named terminal. `Promote` impossible: emitted shape has argv=N+1
     vs declared arity N (would trip `StaticCallArityMismatch`), and
     the honest `Callee::BirthConstructor` shape requires
     `OrdinaryNewAdmissionClaimV1` authority absent by construction on
     this edge — new authority is prohibited. Named casualty: raw-lane
     `new Foo()` + lowered `<Class>.birth/N` loses the malformed legacy
     edge; sole live consumer was the llvmlite-compat egress lane.
   - Delete: `emit_global_unified` (`compat_entrypoints.rs:13-34`) and
     `make_name_const_result` (`name_const.rs:6`, sole caller at
     `compat_entrypoints.rs:21`). Shared
     `annotate_call_result_from_func_name` stays (other callers).
5. **Finite callers**: exactly 4 edges — `emit.rs:27`, `emit.rs:65`,
   `emit.rs:82` (ambient-OFF dispatch), `ordinary_new_admission.rs:104`
   (default).
6. **Verification**: existing pins —
   `published_builtin_print_rejects_legacy_function_carrier`,
   `published_normal_build_selects_ordinary_callee_before_global_lookup`,
   `SelectedNormalUsesLegacyCallV0` admission tests,
   VM `legacy_global_call_rejects_before_legacy_dispatch`,
   all-not-legacy asserts in `me_method_canonical_cutover_tests.rs`,
   phase0-vm-golden canonical asserts. Guard re-pin:
   `tools/checks/lib/mir_call_d1b_active_surface_rows.py:629`
   (`"emit_legacy_call ... not in admission_text"` -> expect removed;
   this is a re-pin of an existing row, not a new guard).

### Declined cohorts

- `Callee::Value`: tuple satisfiable but degenerate — zero
  default-mode legacy production; only ambient-OFF reach (R6-S3
  ingress); consumer is a reject terminal. Vacuous slice.
- `Callee::Method`: old writer lives only in outer-scope ingress
  (`mir_json_v0/module.rs`, R6-S3) + llvmlite egress projection —
  `NoSafeSlice` for S1.
- `Callee::Extern`/`BirthConstructor`/`Constructor`/`Closure`/
  `SameModuleInstance`: no legacy writer exists; delete-set empty —
  not slices.
- `Callee::Global` without the ordinary-new Stop: `NoSafeSlice` — the
  missing element would be an in-series disposition for the live edge.

### Caveat for the execution row

JoinIR `LegacyCallV0` production is documented at
`join_ir/mod.rs:206-219` but the actual constructor call site was not
located inside `src/mir/join_ir` — flag for the execution row to
re-check that no JoinIR path can mint `LegacyCallV0{Global}` before
the delete-set is applied.

Next execution row: `MIR-CALL-R6S1-GLOBAL-PRODUCER-COHORT-S1`.
