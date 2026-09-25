# MIR-CALL-R6S2-PUBLISHED-VIEW-GLOBAL-STOP-S2 — published-view Global stop

Status: landed__2026-09-25
Date: 2026-09-25
Parent: MIR-CALL-R6S2-BACKEND-BOUNDARY-D0 (accepted, boundary=published-view admission)
Owner card:
  docs/development/current/main/investigations/mir-call-compatibility-retire-r7-d0-2026-09-11.md
Authority: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
             section "R6/R7 migration program" row R6-S2.

## Decision

`PublishedMirBackendView` stops consuming `LegacyCallV0{Callee::Global}`
on the published backend route — one admission boundary, matching the
R6-S1 Global producer cohort.

- Source authority + canonical issuer: `published_backend_view.rs`
  `try_new`/`try_new_selected_normal`; `MirCall` remains the sole
  canonical call carrier.
- Non-authority: no receipt, no new guard beyond the row registration
  in the existing stable guard, no `CallV2`, no JSON-emit boundary
  change, no analysis-reader sweep.
- Fail-fast boundary: `UnsupportedBeforeObject` (try_new) /
  `SelectedNormalUsesLegacyCallV0` (selected admission) — both
  pre-artifact, no fallback/retry.
- Non-claims: other legacy callee kinds (Method from `mir_json_v0`
  boxcall, Value from unified-off `emit_value_unified`) keep
  `ExplicitCompatibility`; JSON-emit residual crossing is a later S row.

## Scope

### In-series edits

1. `src/mir/compiler/normal_default_pipeline/published_backend_view.rs`
   `try_new` (~:299-345): for `MirInstruction::LegacyCallV0` with
   `callee` = `Some(Callee::Global(_))`, set
   `has_non_lifecycle_unsupported = true` and skip the validate/push
   into published call rows. Typed `Call` Global arm unchanged; other
   legacy callee kinds unchanged.
2. Same file `try_new_selected_normal` (~:440-470): a
   `LegacyCallV0{Global}` site raises
   `SelectedNormalUsesLegacyCallV0` unconditionally (not only on mixed
   shapes). Non-Global legacy sites keep existing mixed-only semantics.
3. `src/mir/function/published_backend_view_selected_admission_tests.rs:60`
   `selected_normal_admission_keeps_legacy_only_input_on_existing_route`
   → repurpose to expect `SelectedNormalUsesLegacyCallV0` (the fixture
   mints `LegacyCallV0{Global}` — the exact retired edge).
4. `src/mir/function/published_backend_view_tests.rs` legacy-Global
   fixtures → expect `UnsupportedBeforeObject` where they previously
   asserted consumed rows.
5. Register `MIR-CALL-R6S2-PUBLISHED-VIEW-GLOBAL-STOP-S2` in
   `dispatch_extended_row` (existing stable-guard extension point;
   pins the two admission invariants).

### Named casualty

None expected: no production path mints `LegacyCallV0{Global}` since
R6-S1 (grep-verified); remaining callers only see test fixtures.

### Verification

- `cargo check --profile quick` clean.
- Focused: flipped `:60` test; `published_backend_view_tests` legacy
  Global fixtures → `UnsupportedBeforeObject`; selected-normal typed
  + mixed tests unchanged-green; `normal_default_pipeline` and
  published-object tests green.
- `bash tools/checks/mir_call_d1b_*` stable guard green for this row.
- Pointer/hygiene/lifecycle guards green.

## Evidence (landed 2026-09-25)

- `try_new`: `LegacyCallV0` + `Some(Callee::Global)` + `func ==
  ValueId::INVALID` → `has_non_lifecycle_unsupported` (route
  `UnsupportedBeforeObject`); `func` carrier rows still fall through to
  the named `*UsesLegacyFunctionCarrier` errors.
- `try_new_selected_normal`: `LegacyCallV0{Global}` →
  `SelectedNormalUsesLegacyCallV0` unconditionally; other legacy kinds
  keep the mixed-shape rule.
- Fixture split: `static_function`/`free_function`/`builtin_print_function`
  now emit the production typed `MirInstruction::call` shape;
  `legacy_static_function`/`legacy_free_function`/
  `legacy_builtin_print_function` mint `LegacyCallV0` explicitly for
  carrier/stop tests.
- New pins green: `published_view_stops_legacy_global_before_object`,
  `selected_normal_admission_stops_legacy_only_global_input`;
  carrier-reject (`StaticCallUsesLegacyFunctionCarrier`,
  `FreeFunctionCallUsesLegacyFunctionCarrier`,
  `BuiltinPrintUsesLegacyFunctionCarrier`), arity/missing-definition,
  and compat-family tests unchanged-green (91 passed).
- Stable guard registered via `dispatch_extended_row`
  (`r6s2-published-view-global-stop`), primary dispatcher held at 799
  lines.
- `cargo check --lib --tests --profile quick` clean; pointer / hygiene /
  convergence / lifecycle-inventory guards green; `git diff --check`
  green.
- Reds classified: 6 baseline-debt failures recorded in
  `tools/checks/manifests/cargo_lib_red_baseline.*` (upstream
  `callable-main/qualified-preflight` stops, `callable-loop/facts-absent`,
  LLVM `no_lowering_variant` lane) — none touched by this slice.
- Behavior note: no production path mints `LegacyCallV0{Global}` since
  R6-S1; the flip only affects residual/stale rows and test fixtures.

## Exit

- [x] `LegacyCallV0{Global}` cannot enter `CanonicalTyped` route or
  selected admission (test-pinned).
- [x] Typed `Call{Global}` view output unchanged (byte-level rows).
- [x] All verification green; new reds classified and recorded.
