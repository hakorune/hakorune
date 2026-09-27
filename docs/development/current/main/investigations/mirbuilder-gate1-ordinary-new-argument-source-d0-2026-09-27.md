# MIRBUILDER-GATE1-ORDINARY-NEW-ARGUMENT-SOURCE-D0

Status: closed — Decision accepted 2026-09-27
Parent: workstream row H / Gate-1 internal owner series; follows
`MIRBUILDER-GATE1-STATIC-RESULT-CALLER-COVERAGE-D0`+`S0`+`S1`
(landed — `InstanceConstructor` arm cleared the
`static-result-ingress/foreign-lineage` terminal). Selected by the
caller-coverage S1: the observed boxtorrent terminal moved to
`[freeze:contract][ordinary-new/argument-source-unavailable]` at
`new ContentChunk(cid, data, alloc_handle)`
(apps/boxtorrent-mini/main.hako:78).

## Question

`local x = <call>` produces a binding whose value provably exists
at a later `new` site, yet the prefix-local flow never installs it
into `locals`, so `locals.observe(arg)` yields `ArgumentNotTrivial`
and the issued-claim precheck freezes. Which source-authorized
provenance may the shared ordinary-new admission inventory accept
for such bindings — without widening kinds into guessed meaning
or weakening the complete-or-reject boundary?

## Census (worker + direct verification)

### Owner chain and the firing gate

- `issue_ordinary_source_cohort_v1`
  (`normal_callable_semantic_package/ordinary_new_coseal_issue.rs:34`,
  called once from `issuer.rs:628`) mints
  `OrdinaryNewAdmissionClaimV1` per selected `new` destination;
  `argument_rows` (`ordinary_new_coseal.rs:157`) is a
  `Result<Box<[OrdinaryNewTrivialArgumentV1]>,
  SelectedNewArgumentUnavailableV1>` — rows carry source meaning
  only (Facts discipline).
- Rows are minted by `scan_new_home_flow`
  (`resolved_semantics/home_new_prefix.rs:644-662`): for each
  `[Body, Initializer]` `new` site, each argument is classified by
  `locals.observe(argument.site())
  .and_then(OrdinaryObservation::into_selected_argument)`;
  `None` -> `ArgumentNotTrivial { new_site, site }`.
  `convert_selected_new_arguments`
  (`ordinary_new_coseal_helpers.rs:18-52`) translates kinds 1:1
  into the package twin; a missing observation becomes
  `Err(SourceMismatch)`.
- The production firing gate is `new_expression.rs:104-109`:
  `try_take_ordinary_new_claim` returns `Some(claim)`, then
  `claim.argument_rows().is_err()` -> hard freeze
  `ordinary-new/argument-source-unavailable`. This precedes
  `prepare_ordinary_new_emission`, so an undescribed claim cannot
  even decline. D9 recorded that deleting this precheck was
  refuted (`fb6fd52886` let `new Page(1+2)` reach the Ordinary
  route — complete-or-reject is the designed boundary).
- `home_prefix` Err is deliberately softer:
  `ordinary_new_local_commit.rs:433-458` maps it to
  `RetainedUnavailable` -> Ordinary route. `put` is an instance
  method (`SourceBindingSiteV1::Receiver` ->
  `EntryDemandMissing`), so its claim can never be
  `home_prefix`-Ok — `RetainedUnavailable` -> Ordinary route is
  the designed path for `new ContentChunk`. The sole blocker is
  the `Err` argument rows, not the prefix.

### Why `cid`/`alloc_handle` are uninstalled

`observe` (`home_prefix_local_flow.rs:121-147`) reads literals and
`variable_ref` -> `locals.get(binding)`; `StoredLocal` entries come
only from `install_parameters` (contract kinds),
`install_i64_call_result` (sealed `issue_local_call` I64 proofs),
`install_map`, `install_selected_normal_home`, and
`install_observed` (alias chains).

- `local cid = ContentHash.digest(data)` (:65): `issue_local_call`
  requires a `direct_call_observations` row AND all-`Integer`
  literal arguments (`home_local_call_flow.rs:107-132`) — a
  qualified static call with an identifier argument fails both.
  Falls to `locals.observe(call_site)` -> `None` ->
  `PrefixNotCovered`; the binding is never installed.
- `local alloc_handle = me.allocator.allocate(data.length())`
  (:73): receiver `me.allocator` is `ResolvedMethodCallReceiver-
  SourceV1::Other` (`body_shape.rs:283-288`) — no callee
  resolution at all; same uninstalled outcome.
- `data` is fine: untyped param -> `OpaqueHandle` contract ->
  `StoredLocal::Handle(data)` -> `Handle` arg kind.
- Argument collection stops at ordinal 0 (`cid`);
  `alloc_handle` (ordinal 2) fails identically behind it.

### Downstream cost of a new kind

- `materialize_arguments` (`ordinary_new_admission/selected.rs:
  148-156`): `Local`/`Handle` share one arm —
  `value_for_exact_binding` + `observe_variable_site`. A new kind
  joins the same arm; nothing physical changes.
- `emission_validation.rs:222-223`: `Local`/`Handle` pass
  through unchanged.
- `physical_abi.rs:430-437` `scalar_actual_kind`: `Local`/`Handle`
  already return `Err(actual-kind-unavailable)` on the published
  lane — a new kind keeps the same rejection class.
- Expansion precedent `ORDINARY-NEW-HANDLE-ARG-S0` (guard block
  `mirbuilder_qualified_route_scope_guard.sh:188-220`): one kind
  across both enums + `into_selected_argument` + `materialize`
  + `emission_validation` + `scalar_actual_kind` + pins.

### Corpus demand

`apps/` argumented `new` sites are integer literals, param/plain
locals, and locals bound to `new` results. `lang/src/` adds:
locals bound to call results (`cid`, `alloc_handle` class), direct
call-expression args, `me.field` args, `me.method()` args, and
nested `new` args. A wider trivial-kind list cannot cover that
corpus honestly — `me.field` and direct-call args are different
provenance families, and callee-return-type classification cannot
reach `Receiver::Other` calls like `me.allocator.allocate(..)`.

## Six-line brief

```text
Decision: authorize call-result locals as `BoundValue` — a
  `local x = <inventoried call>` installs `StoredLocal::BoundValue`
  so `new` argument observation describes the binding; every other
  non-trivial shape stays `ArgumentNotTrivial` -> freeze.
Source authority + canonical issuer: `PrefixLocalFlow` sequential
  install (the sole flow owner) reading exact call inventories
  (`method_calls()` / `direct_call_observations()`);
  `SelectedNewArgumentV1` remains the sole argument Facts row.
Non-authority: callee return-type inference, AST re-classification,
  physical value types, generic-lane re-checks.
Fail-fast boundary: uninstalled bindings and `me.field`/direct-
  call/nested-`new` arguments stay `ArgumentNotTrivial` ->
  `argument-source-unavailable`; uncovered prefix statements stay
  `PrefixNotCovered` -> `RetainedUnavailable`.
Smallest next slice: S0 — `BoundValue` install + kind propagation
  + pins (the `new ContentChunk(cid, data, alloc_handle)` shape).
Non-claims: no home-prefix admission widening (`put` still goes
  `RetainedUnavailable` -> Ordinary route); no direct-call-arg,
  `me.field`-arg, `me.method()`-arg or nested-`new`-arg coverage;
  no Gate-1 or MirBuilder completion.
```

## Bounded S0 (emitted)

`MIRBUILDER-GATE1-ORDINARY-NEW-ARGUMENT-SOURCE-S0` — call-result
local provenance only:

- `home_new_prefix.rs`: in the uncovered-initializer branch
  (currently `PrefixNotCovered`), when the initializer site is
  present in `method_calls()` or `direct_call_observations()`,
  `locals.install_bound_value(binding)` — then `PrefixNotCovered`
  as before (prefix state is unchanged).
- `home_prefix_local_flow.rs`: `StoredLocal::BoundValue` +
  `observe` arm -> `OrdinaryObservation::BoundValue`
  (`Consumed`/missing stays `None`).
- `into_selected_argument` -> `SelectedNewArgumentKindV1::
  BoundValue { binding }`; package twin
  `OrdinaryNewTrivialArgumentKindV1::BoundValue { binding }` via
  `convert_selected_new_arguments`.
- `selected.rs` `materialize_arguments`: `BoundValue` joins the
  existing `Local`/`Handle` arm (exact binding + site observation).
- `emission_validation` arm + `physical_abi` `scalar_actual_kind`
  arm (same `Err` class as `Local`/`Handle`).
- Guard follow: `mirbuilder_qualified_route_scope_guard.sh` pin
  set gains the `BoundValue` vocabulary (its "non-trivial stays
  ArgumentNotTrivial" pin remains — `me.field`/direct-call/nested
  shapes still fail).
- Pins: seal/claim-level positive `local h = Q.m(x); new Y(h)`
  (qualified static call result -> `BoundValue` row ->
  `argument_rows` Ok); negative `me.field` arg and direct call
  arg sites -> still `ArgumentNotTrivial`; consumed binding ->
  still unavailable.
- Measurement: `apps/boxtorrent-mini/main.hako` —
  `ordinary-new/argument-source-unavailable` must clear; the next
  named terminal is recorded, not claimed.

## Non-claims

- No `StoredLocal` change for `me.field`/BinOp/map-literal or
  other uncovered initializers — call-inventoried sites only.
- No direct call-expression argument kind — `new Y(f(x))` stays
  `ArgumentNotTrivial`.
- No `me.field` argument kind — `new Y(me.f)` stays
  `ArgumentNotTrivial`.
- No home-prefix or `RetainedUnavailable` change — `put` keeps
  its designed Ordinary-route outcome.
- No Gate-1 or overall MirBuilder completion claim.
