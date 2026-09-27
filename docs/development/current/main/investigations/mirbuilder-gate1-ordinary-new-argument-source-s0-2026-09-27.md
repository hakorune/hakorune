# MIRBUILDER-GATE1-ORDINARY-NEW-ARGUMENT-SOURCE-S0

Status: selected — implementation not yet landed
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

## Pinned evidence (gate: focused test files only)

- Flow/catalog level: `local h = Q.m(x); new Y(h)` where `Q.m`
  sits in `method_calls()` or `direct_call_observations()` —
  the claim's `argument_rows` is `Ok` and ordinal 0 is
  `BoundValue { binding: h }`.
- Negative: `new Y(me.f)` and `new Y(f(x))` still produce
  `ArgumentNotTrivial`; a consumed `BoundValue` binding still
  observes `None`.
- Regression: existing `Integer`/`Bool`/`Local`/`Handle` pins
  unchanged; the qualified-route scope guard stays green with
  its updated pin set.
- `apps/boxtorrent-mini/main.hako`: the
  `ordinary-new/argument-source-unavailable` terminal must clear
  (`put`'s claim declines to the designed Ordinary route); the
  next named terminal is recorded, not claimed.

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

Determined at S0 closeout by the next observed terminal on the
boxtorrent measurement path (bisect already showed
`callable-loop/route-not-front-selected`
`SourceCallOutsideSelectedFamily` at `BoxTorrentManifest.
chunkListText` behind the `new` terminal — a different family).
