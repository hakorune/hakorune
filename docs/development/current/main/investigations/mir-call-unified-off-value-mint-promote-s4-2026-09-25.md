# MIR-CALL-UNIFIED-OFF-VALUE-MINT-PROMOTE-S4 — unified-off Value mint promote

Status: landed__2026-09-25
Date: 2026-09-25
Parent: MIR-CALL-INGRESS-MINTER-RETIRE-D0 (accepted, boundary=Value mint promote)
Owner card:
  docs/development/current/main/investigations/mir-call-compatibility-retire-r7-d0-2026-09-11.md
Authority: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
             Call/R7 frontier (ingress-minter retirement lineage).

## Decision

`emit_value_unified` mints the canonical typed call — one bounded
producer-cohort promotion identical to R6-S1's Global promote. The
unified-off corridor and `emit_legacy_call` keep working; minter #1's
legacy product retires.

- Source authority + canonical issuer:
  `src/mir/builder/calls/unified_emitter/compat_entrypoints.rs`;
  `MirInstruction::call` is the canonical mint.
- Non-authority: no env/flag changes, no corridor retirement, no
  `emit_legacy_call` changes, no `func`/`LegacyCallV0` type changes.
- Fail-fast boundary: `[vm-reference/canonical-call]` (interpreter,
  non-Global `Call`) + `UnsupportedBeforeObject` (published view) —
  same terminal class as the legacy row.
- Non-claims: boxcall ingress stays (sole remaining `LegacyCallV0`
  minter); `func` slot stays for that minter.

## Scope

### In-series edits

1. `src/mir/builder/calls/unified_emitter/compat_entrypoints.rs`
   `emit_value_unified`: replace the `LegacyCallV0{func: func_val,
   callee: Some(Callee::Value(func_val))}` mint with
   `MirInstruction::call(dst, Callee::Value(func_val), args, IO)`.
2. Tests: add/adjust a pin that `emit_value_unified` under
   `NYASH_MIR_UNIFIED_CALL=0` emits `Call{Value}` (no `func` slot
   duplication); v0 wire parity — `Callee::Value` →
   `{"op":"call","callee":{"type":"Value",...}}` identical for both
   carriers (already covered by callee-driven emit tests; add an
   emitter-level parity pin if a focused seam exists).
3. Register `MIR-CALL-UNIFIED-OFF-VALUE-MINT-PROMOTE-S4` in
   `dispatch_extended_row` (r6 helper module).

### Named casualty

- `emit_value_unified` no longer constructs `LegacyCallV0`; the
  `func: func_val` duplication in the minted row is gone (never
  emitted — wire-verified).
- Interpreter-side tag for a `=0` value call changes from the legacy
  reject tag to `[vm-reference/canonical-call]` — same named-stop
  class, recorded residual.

### Verification

- `cargo check --profile quick` clean.
- Focused: emit-level pin for `Call{Value}` under `=0`; existing
  unified-emitter + emit tests green; interpreter named-stop pin;
  `runner::product` + `published_backend_view` focused set green.
- `mir_call_d1b_*` stable guard green for this row.
- Pointer/hygiene/lifecycle guards green.

## Exit

- [x] No production minter emits `LegacyCallV0{Callee::Value}`.
  `compat_entrypoints.rs` mints `MirInstruction::call`; remaining
  `callee: Some(Callee::Value(..))` sites are reader arms and test
  fixtures only.
- [x] Wire parity for `Callee::Value` rows across carriers — pinned
  byte-identical by
  `v0_value_call_wire_is_identical_across_typed_and_legacy_carriers`
  (emitters/calls.rs; both carriers converge on the callee-driven
  `emit_call` seam, `func` is ignored when a callee exists).
- [x] All verification green; new reds classified and recorded.

## Evidence (landed 2026-09-25)

- Mint pin: `ordinary_value_call_under_disabled_profile_mints_
  canonical_value_callee` — `emit_unified_call` under
  `NYASH_MIR_UNIFIED_CALL=off` emits one `Call{Value(42)}`, zero
  `LegacyCallV0`.
- Named stop pin: `canonical_value_call_rejects_at_global_only_
  boundary` — `Call{Value}` stops at `[vm-reference/canonical-call]
  only Global targets are admitted` before dispatch.
- Published view: `Call{Value}` → `UnsupportedBeforeObject` via the
  `canonical_call` arm (pinned by
  `published_same_module_instance_is_unsupported_before_object`,
  including exe-emission stop).
- Repaired casualties: `receipt_requirement_rejects_unified_disabled_
  without_legacy_fallback` and `static_global_receipt_rejects_disabled_
  unified_without_legacy_retry` — S1-era assertions pinned the retired
  legacy carrier; now assert the canonical `Call`/`Callee::Global`
  shape (both baseline-manifest-listed, now green).
- Guard: `mir_call_d1b_active_surface_guard` row registered via
  `dispatch_extended_row` → `helpers_r6.check_unified_off_value_
  mint_promote_s4`; green.
- Focused: `builder::calls` 187/189; `published_backend_view` +
  `runner::product` 100/107. Reds: `rejecting_routes_precede_children
  _and_typeop_uses_one_child`, `exact_target_child`, `index_none_
  direct_calls`, and the 6 published-view rows — all recorded in
  `cargo_lib_red_baseline.tests.txt`; no new regressions.
- Residual (non-claim): R7 stays `NoSafeSlice` — the v0 `boxcall`
  parser remains the sole `LegacyCallV0` minter; its retirement is a
  wire-format deprecation needing a product decision (Phase-0
  `env.mirbuilder.emit`, vm-hako validators, llvm_py consumers).
