# MIRBUILDER-EXE-ACCEPTANCE-TARGET-ONLY-EMISSION-S0

Status: landed__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D8
  (accepted — `TargetOnly` (exact target proven, result
  representation unavailable) is not a dead-end: the plain
  `lower_target_only_static_result_publication_v1` physical lowerer
  is the designed consumer, and its `with_expected_sites` sibling is
  already production for the qualified-main lane)
Owner: workstream row H / unified resume gate 1
Authority: D8 Decision. The `StaticCallResultTargetOnlyV1` row is the
sole source of the exact target; the physical emission claims no
result representation. No new issuer, route, or terminal family.

## Slice

1. `calls/mod.rs`: add `lower_target_only_static_result_publication_v1`
   to the existing `pub(in crate::mir::builder)` re-export list.
2. `member_route.rs` `StaticReceiver` arm `TargetOnly(target)` match
   arm (`:134-140`): replace the named error with
   `lower_target_only_static_result_publication_v1(
   self, &mut descent, target.target().clone(), arguments.len())`
   using the same `AssociatedMethodCallArgumentsV1` descent the
   `Selected` arm uses.
3. Nothing else changes: `Selected` keeps the publication bridge,
   `NoExactStaticTarget` stays a hard named terminal, `Unavailable`
   keeps the retired-fallback/Math paths, the me-call probe's
   `TargetOnly` arm (`static_current_owner_policy.rs:61-67`) stays a
   named error (D8 census item 5 — deliberately excluded sibling).

## Overlap analysis (required, source-read)

- `lower_target_only_static_result_publication_v1` reuses
  `descent.lower_all` + `emit_static_global_target_value_terminal_v1`
  — the same terminal the Selected bridge uses minus the result
  publication commit. The row is already consumed by `take_for_source`
  (`static_call_result_publication_owner.rs:381-384`) before this arm
  runs, so `UnconsumedTargetOnly` drain is unaffected.
- `TargetOnly` rows can also appear under the me-call probe for
  `CurrentOwner` static targets in `StaticBoxMethod` callers; that
  arm stays an error — no production site needs it, the sibling
  (DeclaredInstance) already owns the `me` receiver family, and one
  edge stays one edge.
- `arguments.len()` is `source_argument_count`; the bridge pins
  `static-target-only/source-arity` on mismatch — no silent
  truncation.

## Pins (required before close)

- Positive A (route/unit if cheap): a `TargetOnly` row consumed at
  the `StaticReceiver` arm emits `emit_static_global_target_value_-
  terminal_v1` for the row's exact target — not a publication
  commit (result `Unknown`).
- Positive B: `Selected` still reaches
  `lower_selected_static_result_publication_v1` unchanged.
- Negative A: `NoExactStaticTarget` keeps
  `[static-result-ingress/no-exact-static-target]`.
- Negative B: me-call probe `TargetOnly` keeps
  `[static-result-ingress/target-only/{reason}]`.
- Real app: json_stream_aggregator advances past
  `target-only/StaticCallTargetAuthorityUnavailable` —
  `JsonLine.stringField/2` emits `GlobalCall`; the next honest
  terminal (callee `line.substring`, `intField`, `boolField`, or a
  later stage) is whatever it is.

## Guard

- Extend `tools/checks/mirbuilder_qualified_route_scope_guard.sh`:
  pin the re-export, the `TargetOnly` → `lower_target_only_...`
  arm in `member_route.rs`, and the unchanged
  `static_current_owner_policy` error arm.

## Fail-fast boundary

- `static-target-only/source-arity`, `target-projection`,
  `physical-arity` propagate unchanged.
- `NoExactStaticTarget` and `Unavailable` keep their current
  semantics; no fallback to `handle_static_method_call_with_descent`
  is reintroduced.

## Evidence (landed — filled at close)

- `cargo build` dev profile: green.
- Focused: `cargo test --lib member_route` 13/13; `target_only` 4/4;
  `static_result_publication` 16/16 — Selected/NoExactStaticTarget/
  me-call probe arms unchanged (existing suites, zero drift).
- Real app `json_stream_aggregator` (NYASH_BIN=debug):
  `[freeze:contract][static-result-ingress/target-only/...]` is
  retired — `JsonLine.stringField/2` lowers through
  `lower_target_only_static_result_publication_v1` and the app
  advances to the next honest terminal:
  `[freeze:contract][ordinary-new/argument-source-unavailable]`
  (different authority — ordinary-new argument source admission,
  tracked as the next gate-1 blocker).
- 11-app acceptance suite: 4 pass / 7 fail — identical to baseline;
  all fails reproduce the same pre-existing terminals
  (`route-not-front-selected` ×4, `NamedArray(TextSourceMissing)`,
  `mir_call_no_route`, typed-object emit debts). No regression.
- Guards:
  `tools/checks/mirbuilder_qualified_route_scope_guard.sh` extended
  with the TARGET-ONLY-EMISSION-S0 section (re-export, member_route
  arm, bridge symbol, unchanged me-call error arm) — green;
  `current_state_pointer_guard.sh` — green.

## Non-claims

- Does not prove `stringField`'s result representation (D8-H2b
  stays open as a separate authority).
- Does not wire the me-call probe `TargetOnly` arm.
- Does not claim json green — callee bodies (`line.substring`,
  `StringHelpers.to_i64`) may hit their own named terminals.
