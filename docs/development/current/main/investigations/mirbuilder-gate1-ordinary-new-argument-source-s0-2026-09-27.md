# MIRBUILDER-GATE1-ORDINARY-NEW-ARGUMENT-SOURCE-S0

Status: landed
Date: 2026-09-27
Emission: `mirbuilder-gate1-ordinary-new-argument-source-d0-2026-09-27.md`
Decision — `BoundValue` provenance for call-result locals.
Selection proof: `MIRBUILDER-GATE1-ORDINARY-NEW-ARGUMENT-SOURCE-S0`
-> workstream row H.
MirBuilder goal row 1 (sole Facts issuance — `SelectedNewArgumentV1`
stays the only argument row authority), row 3 (sole admission
boundary — the issued-claim precheck, unchanged), row 4 (verified
admission — exact binding membership, not AST re-classification).

## Exact boundary

- `src/mir/resolved_semantics/home_new_prefix.rs`: in the
  uncovered-initializer branch (`PrefixNotCovered`), when the
  initializer site is present in
  `input.function().method_calls()` or
  `input.function().direct_call_observations()`,
  `locals.install_bound_value(binding)` first; the
  `PrefixNotCovered` unavailable state is set exactly as before
  (no prefix admission change).
- `src/mir/resolved_semantics/home_prefix_local_flow.rs`:
  `StoredLocal::BoundValue`; `observe` yields
  `OrdinaryObservation::BoundValue` for an installed,
  non-consumed binding; `install_bound_value` setter.
- `OrdinaryObservation::into_selected_argument` ->
  `SelectedNewArgumentKindV1::BoundValue { binding }`
  (`src/mir/resolved_semantics/selected_new_arguments.rs`).
- `convert_selected_new_arguments`
  (`ordinary_new_coseal_helpers.rs`) ->
  `OrdinaryNewTrivialArgumentKindV1::BoundValue { binding }`
  (`normal_callable_semantic_package/ordinary_new_arguments.rs`).
- `materialize_arguments`
  (`mir/builder/ordinary_new_admission/selected.rs`): `BoundValue`
  joins the existing `Local`/`Handle` arm —
  `value_for_exact_binding` + `observe_variable_site`, no new
  emitter.
- `emission_validation`: `BoundValue` passes through like
  `Local`/`Handle`; `physical_abi` `scalar_actual_kind`:
  `BoundValue` -> `Err(actual-kind-unavailable)` (same class as
  `Local`/`Handle`).
- `tools/checks/mirbuilder_qualified_route_scope_guard.sh`: pin
  the `BoundValue` vocabulary at the same sites the HANDLE-ARG-S0
  block pins `Handle`; the "non-trivial stays ArgumentNotTrivial"
  pin is unchanged (it now covers `me.field`/direct-call/nested
  shapes instead).

## Out of scope

- `me.field`, `me.method()`, BinOp, map-literal and other
  uncovered initializers — no `StoredLocal` change.
- Direct call-expression args (`new Y(f(x))`), `me.field` args
  (`new Y(me.f)`), nested `new` args — still `ArgumentNotTrivial`.
- `local x = <non-inventoried expr>` bindings — still uninstalled
  and unobservable (same `PrefixNotCovered` + `ArgumentNotTrivial`
  pair as today).
- Any home-prefix/`RetainedUnavailable`/Ordinary-route change.
- Any runtime/VM/legacy-path change.

## Pinned evidence (landed)

- `selected_new_arguments_admit_inventoried_call_result_local`
  (`brand_catalog_selected_new_argument_tests.rs`): `local h =
  Sizes.size(7); local p = new Page(h)` — the claim's
  `argument_rows` is `Ok` and ordinal 0 is
  `BoundValue { binding: h }` with `h`'s exact declaration
  binding proven against the initializers inventory.
- `selected_new_rejects_unbound_and_call_expression_arguments`:
  `local x = 1 + 2; new Page(x)` and `new Page(Sizes.size(7))`
  both stay `ArgumentNotTrivial` — non-inventoried bindings and
  call-expression arguments are not BoundValue scope.
- `selected_new` suite 5/5, `ordinary_new` suite 60/60,
  `normal_callable_semantic_package` 331 pass (3 pre-existing
  baseline failures confirmed identical on HEAD).
- `mirbuilder_qualified_route_scope_guard.sh`: green with the
  added BoundValue vocabulary pins.
- `mir_call_canonical_corridor_guard.sh`: stale pins realigned
  to current files (selected.rs ingress arm, new_expression.rs
  constructor map, calls_compat_v0.rs, six schedule callers,
  dead helpers.rs entry); green.
- `canonical_mir_emit_route_guard.sh` fails identically on HEAD
  (`ordinary-new/local-commit/artifact-root-completion-unavailable`
  on a New-free trivial app) — classified known baseline debt,
  not this slice.
- `apps/boxtorrent-mini/main.hako`: the
  `ordinary-new/argument-source-unavailable` terminal cleared —
  `put`'s claim declined to the designed Ordinary route. The
  observed next terminal is
  `[freeze:contract][ordinary-new/birth-global-legacy-stopped]`:
  a claim-lane `<Class>.birth/N` lowered without claim authority.
  A different family; recorded, not claimed.

## Negative/deletion proof

- No fallback: non-inventoried initializers still mint no
  `StoredLocal`; missing/consumed bindings still observe `None`;
  `Err` argument rows still freeze at the precheck.
- No silent decline: `BoundValue` only describes bindings the
  exact call inventories already located.
- No deletion-set membership: `issue_local_call`, `Home`/`Map`,
  and `Trivial` classes are untouched.
- Gate-1 or overall MirBuilder completion: not claimed.

## Next selected row

`MIRBUILDER-GATE1-ORDINARY-NEW-BIRTH-EDGE-AUTHORITY-D0` —
`ordinary-new/birth-global-legacy-stopped` census and design:
which claim authority co-seals a birth edge when the claim's
home prefix declined to the Ordinary route (the `put` shape),
without restoring the legacy carrier or widening the
admission boundary. The earlier bisect signal
(`callable-loop/route-not-front-selected` at
`BoxTorrentManifest.chunkListText`) remains queued behind it —
a different family, not selected.
