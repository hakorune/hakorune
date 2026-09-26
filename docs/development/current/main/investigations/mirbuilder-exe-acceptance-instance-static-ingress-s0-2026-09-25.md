# MIRBUILDER-EXE-ACCEPTANCE-INSTANCE-STATIC-INGRESS-S0

Status: landed__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D7
  (accepted — the `legacy-fallback-retired` stop for
  `JsonLine.stringField/2` inside `JsonStreamAggregator.ingestLine`
  is a consumption-gating gap, not a missing issuer: the publication
  owner already holds a `(caller, site)` row for the site)
Owner: workstream row H / unified resume gate 1
Authority: D7 Decision. `VerifiedStaticCallResultPublicationOwnerV1`
  stays the sole issuer; `lower_selected_static_result_publication_v1`
  stays the sole physical consumer. This slice only widens WHICH
  callers the existing claim-ingress admits — no new issuer, no new
  route, no new terminal.

## Slice

1. `static_result_publication_ingress.rs`
   `classify_source_context_v1`: gain `declarations` + the already-
   passed `(owner, method, argument_count)` parameters. Admission
   rule for a `Cataloged` caller:
   `caller.namespace() == StaticBoxMethod`
   `|| declarations.declaration_for(StaticBoxMethod, owner, method,
   argument_count).is_some()`.
   Non-StaticBoxMethod callers whose probed target resolves to a
   same-module static-box declaration classify `Cataloged`;
   everything else keeps `Unavailable`.
2. `RawInvocationChildPortV1::take_static_result_publication_ingress_v1`:
   pass `declarations`, `owner`, `method`, `argument_count` through to
   the classifier (the three params stop being ignored).
3. Callers unchanged: `member_route.rs:118` probe keeps passing the
   real `box_name`; `static_current_owner_policy.rs:43` me-call probe
   keeps `"<source-owned>"` (never resolves — DeclaredInstance
   sibling preserved by construction).
4. `RawLegacyChildLoweringPortV1` impl stays `Unavailable`; the
   `RawStructuredChildScopePortV1` forwarder needs no change beyond
   the existing pass-through (no signature change on the trait).
5. Update the stale test
   `declared_instance_cataloged_source_stays_outside_static_ingress`:
   the sibling protection now comes from `<source-owned>`
   non-resolution, not caller gating — pin that instead.

## Overlap analysis (required, source-read)

- Publication rows are keyed `(caller, site)`; issuance already walks
  all cataloged declarations including instance-method bodies
  (`whole_source_inventory.rs` `observe_all_calls`/`seal_qualified`,
  no caller-namespace filter). This slice consumes an existing row —
  nothing new is minted.
- The me-call probe (`static_current_owner_policy.rs:43`) is
  protected twice: `"<source-owned>"` never resolves
  `declaration_for`, and the `CurrentOwner` row family is
  `StaticBoxMethod`-caller-only at issuance
  (`whole_source_inventory.rs:343-354`), which the retained namespace
  clause still covers.
- `Math`/builtin owners never resolve `declaration_for`, so they keep
  `Unavailable` → `qualified_math_compatibility_owner` on every
  caller — no `NoExactStaticTarget` regression.
- `NoExactStaticTarget` for a declared-but-row-absent site stays a
  hard named terminal (observation gap surfaces honestly, e.g.
  bounded-unavailable `observe_method_calls_shadow_view_v0`).
- `RawStructuredChildScopePortV1` forwards the same ingress
  (`raw_structured_child_scope.rs:322-332`); the rule lives inside
  the classifier so the forwarder cannot leak the relaxed branch to
  the me-call probe.

## Pins (required before close)

- Positive A (unit, classifier level): `Cataloged` instance-method
  caller + `(owner, method, arity)` resolving to a same-module
  static-box declaration → `Cataloged` classification.
- Positive B (unit, classifier level): same caller + non-declared
  target (`Math`-shaped owner) → `Unavailable`.
- Positive C (unit, classifier level): `Cataloged` instance caller +
  `owner="<source-owned>"` (me-call probe shape) → `Unavailable`
  (sibling preserved).
- Positive D (route/integration if a cheap fixture exists): a
  cataloged instance method `local x = B.g(...)` lowers through the
  `StaticReceiver` plan to `Selected` → physical `GlobalCall`.
- Negative A: `Cataloged` instance caller + declared target whose
  site has no row → `NoExactStaticTarget` (named, honest).
- Negative B: `ScriptRoot`/`TopLevel`/`Main`/`InstanceConstructor`/
  `NestedBoxMethod` lineages keep `Unavailable` /
  `ForeignLineage`-respecting behavior unchanged.
- Real app: `json_stream_aggregator` advances past
  `static-call/legacy-fallback-retired` to the next honest terminal
  (candidates: `stringField` → `Selected` consumed → subsequent
  `JsonLine.find`/`intField`/`boolField` sites either issue or hit
  their own named row state).

## Guard

- Extend `tools/checks/mirbuilder_qualified_route_scope_guard.sh`
  (or sibling) to pin: the declaration-gated admission clause in
  `classify_source_context_v1`, the `"<source-owned>"` me-call probe
  contract, and the new/updated test names.

## Fail-fast boundary

- `NoExactStaticTarget` / `TargetOnly` / `ForeignLineage` /
  `SourceLocationLost` / `OwnerUnavailable` propagate unchanged.
- `Unavailable` still means "this ingress does not own the site" and
  falls through to the retired fallback for non-Math — the same
  named `legacy-fallback-retired` terminal as today.
- No silent downgrade of `NoExactStaticTarget` for declared same-
  module targets; no row is skipped — a `Selected`/`TargetOnly` row
  reached through this path is consumed by the existing bridge and
  remains covered by `StaticResultPublicationResidual` drain.

## Evidence (landed — filled at close)

Focused pins (`static_result_publication_ingress.rs` tests —
`cargo test --lib static_result_publication_ingress`: 7/7 green):

- `instance_caller_is_admitted_for_declaration_resolved_static_target`
  — `Cataloged` instance-method caller + `(JsonLine, stringField, 2)`
  resolving via `declaration_for` → `Cataloged` classification.
- `instance_caller_declines_non_declaration_targets` — same caller +
  `Math.floor/1` (builtin) and `JsonLine.undeclared/0` (missing method)
  → `Unavailable` (compatibility/retired lanes preserved).
- `me_call_probe_keeps_instance_callers_outside_static_ingress` —
  `owner="<source-owned>"` never resolves → `Unavailable`; the
  DeclaredInstance sibling keeps the site.
- `instance_caller_declines_when_declarations_are_unavailable` —
  unresolvable target stays `Unavailable` rather than guessing.
- `source_classification_enumerates_all_ingress_boundaries` updated
  for the new classifier signature; all prior boundaries kept.

Suites:

- `cargo test --lib static`: 6 failures — all reproduced identically on
  the stashed parent baseline (`source_bound_static_result_owner_...`,
  `selected_generic_static_script_box_...`, `script_partition_...`,
  `raw_invocation_port_collects_static_and_instance_box_methods`,
  `refresh_module_global_call_routes_...`,
  `main_static_child_port_consumes_all_role_rows_once`). Baseline debt.
- `cargo test --lib member_route`: 13/13.
- `cargo test --lib me_call`: 1 failure
  (`moved_same_call_args_is_fail_fast_in_strict_planner_required`) —
  reproduced on baseline (`raw-script-root-ordinary-retired`). Debt.

Real app (`json_stream_aggregator_exe_runtime_boundary.sh`,
`NYASH_BIN=target/debug/hakorune`):

- Before: `lexical scope body failed:
  [freeze:contract][static-call/legacy-fallback-retired]
  owner=JsonLine method=stringField arity=2`.
- After: `lexical scope body failed:
  [freeze:contract][static-result-ingress/target-only/
  StaticCallTargetAuthorityUnavailable] JsonLine.stringField/2` —
  the exact `(caller, site)` row exists and was consumed; the new
  boundary is the result-representation availability authority
  (`stringField`'s own return is not provable: its
  `line.substring` receiver fact is `unavailable_target`). Named
  honest terminal, strictly advanced.
- Suite: boxtorrent_mini / binary_trees / mimalloc_lite keep
  `SourceCallOutsideSelectedFamily`; allocator_stress keeps
  `NamedArray(TextSourceMissing)`; typed_object_newbox_min /
  untyped_field / string_substring_in_range keep their backend
  terminals — all verified byte-identical on the baseline binary
  (string_substring `mir_call_no_route callee_symbol=substring`
  reproduced on parent build).

## Non-claims

- Does not claim json green — `intField`/`boolField`/`find` sites in
  `ingestLine` and downstream families may hold their own named
  terminals.
- Does not reopen D5-sealed coverage forks or the parked
  DeclaredInstance selected-C lane.
- No new issuer/route/terminal; the blanket caller-namespace gate is
  reclassified as a declaration-resolution gate, not widened by
  wildcard.
