# MIRBUILDER-EXE-ACCEPTANCE-MAIN-IMPORT-ARITY-SCOPE-S0

Status: landed__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-MAIN-IMPORT-STRING-RESULT-D0
  (accepted — Option B arity scoping)
Owner: workstream row H / unified resume gate 1
Authority: D0 Decision; `capability.rs:278-279`
  (NormalMainQualifiedMethods = arity-0-only); same class as the
  landed QUALIFIED-PREFLIGHT-ROUTE-SCOPE-S0.

## Slice (landed)

1. `src/mir/normal_callable_semantic_package/model.rs`
   `issue_app_main_qualified_receiver_catalog_relation`: after the
   catalog-brand check, return `Ok(None)` when the app-main callable
   arity != 0. Rationale: the only row consumers are the arity-0
   canonical recipe port and the adapter handoff (unreachable under
   `inner.lower_body`); an arity!=0 main can never consume a row, so
   issuing it only produces `selected-header-*` errors for lanes that
   belong to the publication owner.
2. `src/mir/builder/normal_callable_semantic_loan_port/main_root.rs`
   diversion predicate (~:231): requires `parameter_count == 0` (the
   value already computed at :212-220) in addition to "any
   QualifiedUnbound method call", mirroring the canonical route's own
   arity contract.

## Pins (landed)

- Unit (`resolver_deferred_tests.rs`):
  `app_main_qualified_receiver_relation_skips_arity_bearing_main` —
  `main(args)` + qualified call returns `Ok(None)`; existing arity-0
  tests cover the retained rows.
- Compile-level (`normal_default_pipeline_arity_scope_tests.rs`,
  new sibling module — pipeline_tests.rs hit the 800-line hard
  boundary):
  `arity_bearing_main_with_qualified_call_stays_off_canonical_route`
  — `main(args)` + a qualified String-returning callee does NOT
  surface `main-import-view` (relation not issued, diversion not
  entered); the observed terminal is `entry-shape-mismatch`.
- Guard: `mirbuilder-qualified-route-scope` extended — model.rs
  `caller.arity() != 0` early return + main_root
  `parameter_count == 0` + both pin names.

## Evidence

- `cargo test --profile quick --lib resolver_deferred`: 22/22 pass
  (includes the new arity pin).
- `cargo test --profile quick --lib arity_scope`: 1/1 pass.
- `bash tools/checks/mirbuilder_qualified_route_scope_guard.sh`: ok.
- `git diff --check`: clean.

## Post-slice terminal (honest record, fresh suite receipt)

`real-apps-exe-boundary` with the quick binary: 4 pass / 7 fail —
count unchanged, but both main-import entries moved terminals:

- `json_stream_aggregator`: `main-import-view/selected-header-missing`
  -> `[callable-loop/parts] loop-cond-item-unsupported`
  (`ConditionalUpdateIf` shape inside a `JsonLine` static-child
  method body — lowered before the root body).
- `boxtorrent_mini`: `main-import-view/selected-header-missing`
  -> `ordinary-new/birth-global-legacy-stopped` (a static child's
  `birth` on the ordinary-new lane).
- binary_trees, mimalloc_lite (`birth-global-legacy-stopped`),
  allocator_stress (`NamedArray(TextSourceMissing)`), newbox_min
  (`no_lowering_variant` toolchain), untyped_field (inference
  panic): all unchanged.

Additionally, direct probes established the NEXT latent boundary
for arity-bearing app mains (not yet reached by any suite app —
static children lower first and stop earlier):

- `[callable-semantic-lowering/entry-shape-mismatch]`
  (`normal_callable_semantic_lowering_state.rs:477-481`).
- The wrapper `main()` opened by `open_module_main_wrapper`
  (`module_lifecycle.rs:450-466`) has 0 physical formals; source
  params are injected as LOCALS (`decls.rs:302-363`, `args` gets the
  script-args `ArrayBox`). The semantic ledger declares `args` as
  `Parameter{0}` and `install_entry_values` requires
  `entry.parameters().len()` == declared count — 0 vs 1.
- Reproduced with `main(args)` WITHOUT any qualified call -> the
  boundary is independent of the qualified-relation family; the
  installed lane had simply never reached it.
- Implication for the D0 premise: "main(args) stays on
  inner.lower_body + publication lane" is correct as ROUTE scope,
  but the wrapper route cannot yet ADOPT arity-bearing entries —
  the wrapper param materialization contract (injected locals vs
  declared Parameter bindings) needs its own authority decision.
  Follow-up: MIRBUILDER-EXE-ACCEPTANCE-MAIN-WRAPPER-PARAM-ENTRY-D0.

## Fail-fast boundary (unchanged)

- arity-0 `main()` + qualified call with non-i64/proofless result:
  `selected-header-missing` / `selected-header-not-exact-i64`.
- Publication lane: `static-result-ingress/target-only/{reason}`,
  `no-exact-static-target`, drain `StaticResultPublicationResidual`.
- No silent skip: sites still must be covered — by relation rows
  (arity-0) or the publication owner (which independently enforces
  complete consumption at drain).

## Non-claims

- No EXE green claim; json_stream/boxtorrent now stop at
  `entry-shape-mismatch` (arity-bearing app-main wrapper param
  adoption), not at `main-import-view`.
- The carded pin "result committed via the publication lane" is
  NOT yet exercised — `entry-shape-mismatch` precedes it; asserted
  instead at the route-scope level (no `main-import-view`) until
  the wrapper-param authority lands.
- No result ABI / signature widening; `ExactTrivialScalarAbiV1`
  stays i64-only.
- No change for arity-0 qualified route consumers.
