# MIRBUILDER-EXE-ACCEPTANCE-ME-RECEIVER-SITE-S0

Status: active__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D6
  (accepted — the located `Me`/`This` MethodCall receiver carve-out is a
  receiver-site consumption gap, separable from the parked
  DeclaredInstance selected-C admission lane)
Owner: workstream row H / unified resume gate 1
Authority: D6 Decision. Sole consumption owner stays the callable
lowering ledger (`read_variable`/`consumed_variables`); the located
source port is the only access point. No new recipe type, no new route,
no Facts change.

## Slice

1. `LoopPlanExpressionPortV1` (`plan/expression_port.rs`): add
   `exact_source_receiver_value` — returns `Ok(Some(value))` when the
   `Me`/`This` receiver expression carries a registered variable site
   (consumes it via `read_variable`), `Ok(None)` when no site is
   registered (e.g. `this` in a static box). `RawLoopPlanExpressionPortV1`
   keeps the default `Ok(None)`.
2. `normal_callable_loop_source_port.rs`: implement
   `exact_source_receiver_value` — locate the receiver site, check
   registration via `source_read_binding` (existence check, not a name
   check), consume via `read_variable` when present.
3. `lower_method_call_statement_input`
   (`loop_body_lowering_associated_input.rs`) `Me | This` arm: when the
   port consumes a receiver site, emit `CoreEffectPlan::MethodCall`
   with the materialized receiver directly; otherwise keep
   `lower_me_this_method_effect` unchanged (static-box `this.method`
   GlobalCall path preserved).
4. `helpers_value/lower.rs` MethodCall `Me | This` arm: same — consume
   the site through the port when registered, emit `MethodCall` with
   `dst: Some(result_id)`; otherwise unchanged.

## Overlap analysis (required, source-read)

- `read_variable` enforces exactly-once consumption
  (`duplicate-variable-consumption`); a second `me.method` receiver in
  the same function is a *different* source site, so repeated calls stay
  legal.
- `lower_me_this_method_effect` stays the owner for the unregistered
  receiver shapes (static-box `this`, `env`-style edge cases). The new
  arm only intercepts when the ledger registered a site — no name-based
  fallback is added.
- The DeclaredInstance locator (`take_exact_receiver_value`) remains
  the raw-lane consumer; this slice does not wire the locator into the
  located path and does not change call emission kind.

## Pins (required before close)

- Positive A: `loop(...) { me.bump(i) }` — the `me` receiver site is
  consumed and the emitted `MethodCall` carries the materialized
  receiver (assert the site no longer drains at
  `incomplete-consumption`; a second `me` call in the same body still
  lowers — separate sites).
- Positive B (value position): `loop(...) { x = me.size() }` —
  receiver site consumed, `MethodCall` with `dst` emitted.
- Negative A: `this.method` receiver in a static-box context where no
  site is registered keeps the `GlobalCall` path (no
  `missing-variable-site` hard fail).
- Negative B: duplicate site consumption stays a named reject
  (covered by `read_variable` itself; pin at the port level if a
  cheap test exists).
- Real app: json_stream_aggregator advances past
  `incomplete-consumption` to the next honest terminal (expected:
  `UnconsumedSelected` for the `JsonLine.find` publication, F3b).

## Guard

- Extend `tools/checks/mirbuilder_qualified_route_scope_guard.sh`
  (or sibling) to pin: the `exact_source_receiver_value` port method,
  its located-port implementation, both `Me | This` call sites, and
  the new test names.

## Fail-fast boundary

- `missing-variable-site` / `duplicate-variable-consumption` /
  `variable-before-materialization` propagate unchanged — no silent
  skip. `Ok(None)` means "no registered site", which is only legal for
  receivers that never entered `variable_refs` (static-box `this`);
  `me` receivers without a registered site must surface the existing
  `me.method without bound receiver` error, not a new fallback.

## Evidence (landed — filled at close)

Focused pins (`callable_loop_source_items_tests.rs`,
`cargo test --lib receiver` slice — all green):

- `source_item_consumes_me_receiver_site_for_method_call` — bare
  `me.bump(7)` loop-body Stmt item lowers to `CoreEffectPlan::MethodCall`
  with the materialized receiver. The test builder never binds `me` in
  `variable_map`, so emission proves the site-consumption path (the
  name-based arm would emit `me.method without bound receiver`).
- `source_item_consumes_me_receiver_site_for_value_call` — `local x =
  me.size()` emits `MethodCall { dst: Some }` and binds `x`.
- `source_item_consumes_this_receiver_site_for_method_call` — `this`
  resolves to the receiver binding in a plain callable; its site is
  registered and consumed the same way.
- `port_declines_receiver_value_for_non_receiver_expression` —
  non-`Me`/`This` inputs return `Ok(None)` (existing path keeps owning
  them).
- `source_item_rejects_duplicate_me_receiver_site_consumption` —
  driving the same `me.bump(7)` item twice surfaces the named
  `duplicate-variable-consumption` reject.

Fixtures note: bare `me.method` statements are only locatable under
`Cataloged`/`TopLevel`/`InstanceConstructor` root lineage, so a
`cataloged_body_source()` helper was added to
`callable_loop_source_testkit.rs` (ScriptRoot rejects them by design,
matching `allows_bare_function_call_location`).

Suites:

- `cargo test --lib callable_loop_source`: 57/57.
- `cargo test --lib loop_cond`: 78/79 — the one failure is the known
  baseline debt `program_block_with_exit_signals_prefers_recipe_only`
  (reproduced on the parent baseline in the F2 slice).
- `cargo test --lib receiver`: 258 pass / 7 fail — all 7 failures
  reproduced identically on the stashed parent baseline
  (`field_receiver_provenance` real fixture, `direct_array_extent_fact`,
  `string_corridor_sink`, `owner_forest` upvar, `unified_members`,
  `user_box_method_determinism` ×2). Existing debt, not this slice.

Real app (`json_stream_aggregator_exe_runtime_boundary.sh`,
`NYASH_BIN=target/debug/hakorune`):

- Before: `[freeze:contract][callable-semantic-lowering/incomplete-consumption]`
  `missing_variables=[SourceNodeSiteV1([Body(2), LoopBody(2), Receiver])]`
  (`me.ingestLine` in `ingest`).
- After: `ingest`'s `Receiver` site is consumed and its finish passes;
  the app advances to the next honest terminal:
  `lexical scope body failed: [freeze:contract][static-call/legacy-fallback-retired]
  owner=JsonLine method=stringField arity=2` — `JsonLine.stringField`
  inside `ingestLine` (:122) has no issued static-call route. Different
  authority (selected static-call issuance), next census.
- Suite totals unchanged: 4 pass / 7 fail; boxtorrent_mini,
  binary_trees, mimalloc_lite keep `SourceCallOutsideSelectedFamily`;
  allocator_stress keeps `NamedArray(TextSourceMissing)`;
  newbox/untyped_field keep their backend/emit terminals.

## Non-claims

- Does not claim json green — downstream `UnconsumedSelected` (F3b)
  and any `me.` emission-coverage questions stay open.
- Does not reopen the D5-sealed coverage forks or the parked
  DeclaredInstance admission lane.
- No coverage-contract change; `SourceItemsMissing`,
  `SourceCallOutsideSelectedFamily`, `UnconsumedSelected` remain named
  terminals.
