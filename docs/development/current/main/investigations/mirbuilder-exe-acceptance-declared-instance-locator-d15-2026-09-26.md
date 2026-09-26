# MIRBUILDER-EXE-ACCEPTANCE-DECLARED-INSTANCE-LOCATOR-D15

Status: design_stop__2026-09-26
Date: 2026-09-26
Parent: workstream row H (unified resume, gate 1);
  COND-SUBSTRING-COVERAGE-S2 landed
Mode: design_stop — census and Decision only.

## Blocking observation

```text
EXE (pure-first, debug binary target/debug/hakorune):
  [freeze:contract][mir/callable-semantic-package/port]
  DeclaredInstanceLocatorNotConsumed
  (package complete(): selected coverage + ordinary-new
  coverage already consumed; declared-instance call
  locator rows remain)

VM (debug binary): unchanged known terminal —
  loop winner selection declined: zero selected family
  candidates fn=JsonStreamAggregator.ingest/1 [Body(2)]
  (ledger-less instance method → route_loop spine;
  separate authority, recorded non-claim).
```

`trim/1`'s condition-position `s.substring` coverage is
now armed (S2); every selected callable lowers through
the package port and `complete()` reaches the
declared-instance locator count check. Some locator rows
were never consumed during lowering.

## Census answers

Instrumented run (`HAKO_D15_CENSUS`, reverted):

```text
take  site=[Body(5), Initializer(0)] owner=slot 26   <- ingestLine:
                                                  local stats = me.statsFor(user)
consumed={0} total=2
```

- Issuer: `issue_declared_instance_call_package_locator_v1`
  (issuer.rs:710) seals every resolver DeclaredInstanceCall
  relation row for selected InstanceBoxMethod callers —
  including `me.method` sites inside loop bodies.
- Install: `cataloged_instance_scope.rs` arms
  `DeclaredInstanceCallLocatorScopeV1` on the raw frame for
  each selected cataloged instance child — observed working
  (row 0 consumed via body lane).
- Body-lane consumer: `resolve_me_call*` ->
  `take_declared_instance_receiver_value_v1` ->
  `take_declared_instance_receiver_value_inner_v1`
  (recursive_child_lowering.rs:448) -> `CanonicalInstance`
  -> `emit_canonical_instance_value_terminal_v1`
  (`Callee::SameModuleInstance`).
- Gap (root cause): the callable-loop source lane uses
  `CallableLoopSourceExpressionPortV1`, which carries only
  `&Rc<RefCell<CallableSemanticLoweringState>>` — the armed
  locator on `RawInvocationChildPortV1`
  (`declared_instance_locator`) never reaches it. Loop-body
  `me.method` flows
  `lower_opaque_stmt -> lower_simple_effect_stmt_input ->
  lower_method_call_statement_input` (Me arm,
  loop_body_lowering_associated_input.rs:243-270) ->
  `exact_source_receiver_value` -> `CoreEffectPlan::MethodCall`
  (dynamic `runtime_method_call`). Same gap for
  value-position `me.method`
  (helpers_value/lower.rs:339-351).
- Semantic divergence: besides leaving the row unconsumed,
  `me.method` emits `Callee::SameModuleInstance` (canonical
  direct call) on the body lane but `CoreEffectPlan::
  MethodCall` (dynamic dispatch) on the loop lane — two
  physical meanings for one source form.
- Route check: `ingest` is lowered through
  `lower_cataloged_instance_box_method` ->
  `with_cataloged_callable_source_scope` — the correct
  cataloged route; `with_main_static_child_lowering_input`
  is NOT used. The completion accounting
  (`declared_instance_consumed` vs `row_count`) is correct —
  it caught a real bypass, not an accounting bug.

## Decision

`me.method` call sites inside selected callable loop bodies
are covered by the same DeclaredInstance locator authority.
Thread the armed `DeclaredInstanceCallLocatorScopeV1` into
the callable-loop source lane so loop-body `me.method`
consumes its exact row and emits the canonical
`Callee::SameModuleInstance` — never the dynamic
`CoreEffectPlan::MethodCall` on the armed lane.

Source authority + canonical issuer: package-owned
  `DeclaredInstanceCallLocatorScopeV1::take_exact_relation`
  (same take as the body lane); physical emitter stays
  `emit_canonical_instance_value_terminal_v1` (sole
  SameModuleInstance emitter).
Non-authority: `CoreEffectPlan::MethodCall` dynamic
  dispatch on locator-armed lanes; `GlobalCall` name
  projection (no receiver slot).
Fail-fast boundary: armed locator + `me.method` site with
  no locator row -> `take_exact_relation` error propagates
  (SiteUnavailable/DuplicateSite/AlreadyTaken) — no silent
  dynamic fallback.
Smallest next slice: S3 — scope Copy-ification
  (`&RefCell<BTreeSet>` consumed set), `Option<Scope>`
  threaded `lower_loop` ->
  `lower_v1_with_root_scope_and_callable_ledger` ->
  `lower_v1_with_optional_root_scope` -> every
  `into_physical_input` (loop_cond/loop_true/loop_break/
  composite) -> `CallableLoopSourceExpressionPortV1` (still
  `Copy`); one `LoopPlanExpressionPortV1` hook
  `exact_source_declared_instance_call_v1` (default
  `Ok(None)`); `Me` arms in
  `lower_method_call_statement_input` and
  `lower_value_input` consult it and emit a new
  `CoreEffectPlan::DeclaredInstanceCall` variant whose
  emission delegates to `emit_canonical_instance_
  value_terminal_v1`.
Non-claims: VM `ingest/1` ledger-less spine (unchanged);
  `ConditionalUpdateIf` F2 boundary; static-publication
  gaps; Gates 2-4; overall completion.

## Boundary of this census

- Covers: `DeclaredInstanceLocatorNotConsumed` at package
  `complete()` on the json_stream_aggregator EXE lane,
  from locator issue to consumption marking.
- Excludes: VM ledger-less lane (`ingest` — recorded
  non-claim); `ConditionalUpdateIf` F2 parts boundary;
  static publication gaps (binary_trees / mimalloc).

## Exit

- [ ] Unconsumed locator rows identified by call site and
  intended consumer.
- [ ] Existing owner vs unclaimed gap classified.
- [ ] One bounded S-card emitted, or NoSafeSlice with the
  missing authority named.
