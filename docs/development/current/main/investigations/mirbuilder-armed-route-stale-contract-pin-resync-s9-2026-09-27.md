# MIRBUILDER-ARMED-ROUTE-STALE-CONTRACT-PIN-RESYNC-S9

Status: landed
Date: 2026-09-27
Parent: workstream row H (unified resume, gate 1);
  SUITE-RED-DISPOSITION-D21 follow-up (baseline debt surfaced
  during D21 evidence runs)
Mode: fast — one responsibility, zero production edges.

## Responsibility

Re-sync two stale contract pins on the armed lane's test surface
to the landed contract vocabulary. This slice changes test
fixtures/assertions only; it admits nothing, retires nothing, and
touches no production authority.

## Boundary

- Test-only (BoxShape): no production code, no contract change,
  no new accepted shape, no suite membership change.
- Each repaired pin keeps its original assertion intent — a
  genuinely unsupported ancestor stays `Unarmed`, and an
  uncovered static call site stays a fail-fast ingress terminal.
- Gate-1 suite state is unchanged: 4/11 (probe included);
  `typed_object_newbox_min` stays environment/toolchain debt
  (verified green under `PATH=/usr/lib/llvm-18/bin`).

## Decision

Decision: resync stale pins to the landed vocabulary instead of
treating them as live failures.
Source authority + canonical issuer: the landed contract itself —
`portable_path_v1`'s closed ancestor grammar
(`resolved_source_adapter.rs`) and the
`StaticResultPublicationIngressV1` terminal vocabulary
(`static_result_publication_ingress.rs`).
Non-authority: the stale test expectations carry no authority
over the contract; no production file changes.
Fail-fast boundary: `Unarmed` stays a typed disposition (not a
fallback); `NoExactStaticTarget` stays a hard freeze, not a
descent.
Smallest next slice: this resync, then back to D21's externally
blocked disposition.
Non-claims: no Gate-1 progress; no claim the two pins covered the
same class before (see drift evidence).

## Drift evidence

1. `source_loop_bridge::tests` — `nested_scope_loop` and
   `mixed_loop_function` fixtures nest the unarmed loop inside an
   `if` body. `440c16216d`
   (`feat(mir): retain conditional source paths`,
   loopbreak-composite-source-package-d0) admitted
   `SourcePathSegmentV1::IfThen`/`IfElse` into the closed portable
   ancestor grammar (`portable_path_v1`,
   `resolved_source_adapter.rs:289-294`), so an `if`-nested loop
   is now *armed*, not `Unarmed`. The pins drifted stale. The
   fixtures move the nested loop to a `fastmem Contract { }` body —
   `FastMemBody` is outside the portable grammar and still
   produces `LoopRootSourceBindingRejectV1::UnsupportedAncestor`,
   keeping each pin's intent: unsupported-ancestor loops stay
   `Unarmed`, armed roots keep the bridge armed, uncataloged sites
   remain contract violations. (`try` was evaluated first and
   rejected: `try` requires an explicit `Compat2025` grammar
   profile — `parser/try_reserved`; `match` arms are rejected at
   resolution — `UnsupportedExpression MatchExpr`. `fastmem` is a
   parse-only capsule the resolver accepts and maps to
   `FastMemBody` segments via `source_path_policy`.)
2. `normal_default_root_catalog_lifecycle_tests::
   parser_scan_package_passes_callable_source_handoff_without_fallback`
   pinned `static-result-ingress/target-unavailable` (written at
   `95b65e4081`, 2026-08-22). `3e7f4a1d7f`
   (`mir: retain exact ordinary static targets`, 2026-08-30)
   replaced that terminal with the designed
   `StaticResultPublicationIngressV1` vocabulary
   (`Selected`/`TargetOnly`/`NoExactStaticTarget`/`Unavailable`);
   `target-unavailable` no longer exists in the source tree. The
   fixture's failing calls are the qualified
   `ParserCommonUtilsBox.i2s` sites — a `using`-imported box whose
   target is not same-module, so the publication owner holds no
   exact-target row and `NoExactStaticTarget` is the honest next
   blocker. The pin now asserts that terminal; the
   `missing-variable-site` no-fallback guard is unchanged.

## Acceptance

1. `cargo test --profile quick -p nyash-rust
   mir::builder::normal_callable_semantic_lowering_state::source_loop_bridge`
   — 5/5 green (verified).
2. `cargo test --profile quick -p nyash-rust
   mir::builder::normal_default_root_catalog_lifecycle`
   —
   `parser_scan_package_passes_callable_source_handoff_without_fallback`
   green (verified); residual module reds classified below.
3. Pointer guard green; no production diff in this slice.

## Residual baseline debt (verified on parent HEAD)

The same `normal_default_root_catalog_lifecycle` filter reports
nine reds on parent HEAD and eight with this diff — the delta is
exactly `parser_scan_package`. All eight residuals reproduce
identically without this diff (`git stash` + rerun), so they are
`known baseline debt`, not current-change failures:

- `loop_scope_tests::source_backed_loop_keeps_invocation_scope_and_ledger_route`
  — `LoopCondRouteRejected(SourceItemsMissing)` on `Scan.run/2`.
- `normal_default_root_catalog_merged_route_tests::
  merged_parser_program_source_stops_at_named_publication_boundary`
  — `SourceCallOutsideSelectedFamily` on
  `ParserStringUtilsBox.i2s/1` (same designed freeze family as the
  D5-sealed Gate-1 coverage reds; the armed route now front-loads
  the freeze before the pinned publication boundary).
- `variable_accum_tests::
  accepted_variable_accum_callable_completes_source_backed_mir_lifecycle`
  — `VariableAccumRecurrenceOverlap { routes: [LoopCondBreakContinue] }`
  on `main/0`.
- `variable_accum_tests::
  late_var_failure_discards_lowered_invocation_and_fresh_call_succeeds`
  — expects the terminal substring `failure-after-source-consume`;
  the current error no longer carries it (likely the same stale-
  vocabulary class as this slice's pins; needs its own audit).
- `normal_default_root_catalog_lifecycle_tests::
  actual_string_helpers_general_result_row_reaches_its_first_loop_carrier`,
  `source_bound_static_result_owner_reaches_the_raw_terminal`,
  `source_backed_app_main_direct_call_consumes_affine_loan`,
  `source_backed_package_failure_is_terminal_before_builder_effects`
  — armed-route lifecycle fixtures rejected or stage-mismatched;
  several look like the same contract-progression family
  (`route-not-front-selected` fronting freezes that landed after
  the fixtures were written).

These are a disposition family, not part of this slice — they
need their own census row (stale fixture premise vs. live armed-
route regression per case), and they are recorded here so the
next scheduler can pick them up.

## Non-claims

- No Gate-1 suite progress claim; the seven suite reds and their
  owners are unchanged (D21 census stands).
- No claim that `if`-nested loops were wrongly admitted — the
  admission is the designed conditional-path contract; the tests
  were stale.
- No claim that `NoExactStaticTarget` coverage is new — the
  terminal existed and was reachable; only the pinned string was
  stale.
