# MIRBUILDER-EXE-ACCEPTANCE-DECLARED-INSTANCE-LOOP-LOCATOR-S3

Status: landed__2026-09-26
Date: 2026-09-26
Parent: workstream row H (unified resume, gate 1);
  DECLARED-INSTANCE-LOCATOR-D15 decision accepted
Mode: fast — one responsibility, one production edge.

## Responsibility

`me.method` call sites inside selected callable loop bodies
consume their package-issued DeclaredInstance locator row
and emit the canonical `Callee::SameModuleInstance` — the
same physical meaning the body lane already produces via
`resolve_me_call` -> `CanonicalInstance` ->
`emit_canonical_instance_value_terminal_v1`.

## Boundary

- The locator scope becomes `Copy`
  (`consumed: &RefCell<BTreeSet<u32>>`, `take(&self)`), so
  `CallableLoopSourceExpressionPortV1` keeps its `Copy`
  shape while carrying `Option<DeclaredInstanceCallLocatorScopeV1>`.
- Armed locator threads
  `RawInvocationChildPortV1::lower_loop` ->
  `lower_v1_with_root_scope_and_callable_ledger` ->
  `lower_v1_with_optional_root_scope` -> every
  `into_physical_input` (loop_cond / loop_true / loop_break /
  composite) -> `CallableLoopSourceExpressionPortV1::new`.
- One `LoopPlanExpressionPortV1` hook
  `exact_source_declared_instance_call_v1` (default
  `Ok(None)` = unarmed). The port impl performs the same
  take as `take_declared_instance_receiver_value_inner_v1`:
  owner from the callable ledger, call site from the
  input's exact source context, receiver via
  `take_exact_receiver_value` (which already consumes the
  me receiver site — `exact_source_receiver_value` is NOT
  also run on the armed path).
- `Me`/`This` receiver arms in
  `lower_method_call_statement_input` (statement position)
  and `helpers_value/lower.rs` (value position) consult the
  hook first; on `Ready` they emit the new
  `CoreEffectPlan::DeclaredInstanceCall { source, dst, key,
  receiver, args }` variant instead of the dynamic
  `CoreEffectPlan::MethodCall`.
- Emission of `DeclaredInstanceCall` delegates to the sole
  canonical emitter (shared helper around
  `create_mir_call(Callee::SameModuleInstance)` +
  `emit_finalized_generic_call_v1`), so `dst: Some` plans
  bind the plan-allocated value id exactly like `GlobalCall`
  does.
- Every `match CoreEffectPlan` site gains the arm with the
  same dst/args semantics as `MethodCall`.

## Fail-fast boundaries

- Armed locator + `me.method` site with no exact row:
  `take_exact_relation` error propagates
  (`SiteUnavailable` / `DuplicateSite` / `AlreadyTaken`) —
  no dynamic-dispatch fallback on the armed lane.
- `key.name()`/`key.arity()` must equal the source
  `method`/argument count (mirrors
  `validate_prepared_me_arity_before_descent`).
- Unarmed (no locator) lanes keep the existing
  `exact_source_receiver_value` -> `MethodCall` path —
  compatibility/test ports are unaffected.

## Non-claims

- VM `ingest/1` ledger-less spine (unchanged, recorded).
- `ConditionalUpdateIf` F2 parts boundary.
- `variable_accum` lane `me.method` coverage if it bypasses
  the shared port.
- Gates 2-4; overall MirBuilder completion.

## Evidence

- `cargo check --lib` / `cargo check --tests`: clean.
- Focused pins
  (`callable_loop_source_declared_instance_tests.rs`, 3/3):
  - `armed_loop_body_me_call_consumes_the_exact_locator_row` —
    armed locator consumes the exact row by owned source site,
    emits `DeclaredInstanceCall` with the canonical
    `InstanceBoxMethod` key, exactly-once consumption, and
    `complete()` reports no residual row.
  - `armed_locator_rejects_a_foreign_method_key_without_fallback` —
    key/arity mismatch propagates a typed error; no dynamic
    `MethodCall` fallback on the armed lane.
  - `unarmed_port_projects_no_declared_instance_call` —
    compatibility ports keep the previous `Ok(None)` shape.
- `cargo test --lib declared_instance`: 25/25.
- `cargo test --lib callable_loop_source`: 67/67.
- Red classification — all reproduced on the pre-change HEAD,
  recorded as baseline debt, not S3 regressions:
  - `raw_loop_child` 16 passed / 2 failed:
    `unarmed_nested_loop_stops_at_source_facts_without_compatibility_reentry`,
    `candidate_without_callable_ledger_is_terminal_before_builder_effects`.
  - `normalizer` 36 passed / 1 failed:
    `demo_if2_valuejoin_emits_phi_and_return`.
  - `verifier` 90 passed / 1 failed:
    `public_resolved_route_produces_verifier_clean_mir`.
- EXE boundary smoke
  (`tools/smokes/v2/profiles/integration/apps/json_stream_aggregator_exe_runtime_boundary.sh`,
  debug binary): `DeclaredInstanceLocatorNotConsumed` no longer
  fires — every package-issued declared-instance row is consumed,
  including the loop-body `me.ingestLine(...)` site in
  `JsonStreamAggregator.ingest/1`. The terminal advanced to the
  next named residual:
  `StaticResultPublicationResidual(UnconsumedSelected{caller:
  JsonStreamAggregator.ingest/1, site: SourceExprSiteV1([Body(2),
  LoopBody(0), Initializer(0)]), target: JsonLine.find/3})` — a
  selected STATIC-box call inside a loop-body initializer; a
  separate coverage family outside this slice.
- VM lane: unchanged recorded non-claim — `ingest/1` stays on the
  ledger-less spine decline (`zero selected family candidates`).
- `cargo fmt --check`: clean on all touched files (remaining
  diffs are pre-existing drift in untouched test code).
- Source size: `helpers_value/lower.rs` 797 -> 790 lines after
  extracting the shared `me_this_method_call_effect` /
  `declared_instance_call_effect` into `normalizer/common.rs`;
  every touched file is below the 800 hard stop.
- Guards: `mirbuilder_qualified_route_scope_guard.sh` ok (S3
  section added; receiver-value pins repointed at the shared
  helper); `current_state_pointer_guard.sh` ok.
