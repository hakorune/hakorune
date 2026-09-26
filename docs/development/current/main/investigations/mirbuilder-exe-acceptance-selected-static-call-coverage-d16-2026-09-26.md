# MIRBUILDER-EXE-ACCEPTANCE-SELECTED-STATIC-CALL-COVERAGE-D16

Status: design_stop__2026-09-26
Date: 2026-09-26
Parent: workstream row H (unified resume, gate 1);
  DECLARED-INSTANCE-LOOP-LOCATOR-S3 landed
Mode: design_stop — census and Decision only.

## Blocking observation

```text
EXE (pure-first, debug binary target/debug/hakorune):
  [freeze:contract][mir/callable-collector/atomic-commit]
  [freeze:contract][normal-collector-drain]
  StaticResultPublicationResidual(UnconsumedSelected {
    caller: InstanceBoxMethod JsonStreamAggregator.ingest/1,
    site: SourceExprSiteV1([Body(2), LoopBody(0), Initializer(0)]),
    target: StaticBoxMethod JsonLine.find/3 })

VM (debug binary): unchanged known terminal —
  loop winner selection declined: zero selected family
  candidates fn=JsonStreamAggregator.ingest/1 [Body(2)]
  (ledger-less instance method → route_loop spine;
  separate authority, recorded non-claim).
```

S3 consumed every declared-instance locator row; the drain now
reaches the static-result-publication residual. A `Selected`
publication row issued for `JsonLine.find(...)` inside the
`ingest` loop-body initializer was never taken.

## Census answers

- Issuer: `VerifiedStaticCallResultPublicationOwnerV1` seals
  one row per selected static-call site keyed by exact
  (caller, site, target) identity
  (`static_call_result_publication_owner.rs`);
  `take_for_source` is the sole consumption boundary.
- Install: the owner lives on
  `ModuleDraftCollectorV1.static_result_publication_owner`;
  `take_static_result_publication_handoff`
  (`module_draft_collector/static_result_publication_owner.rs`)
  is the collector-facing take.
- Body-lane consumer: `build_member_method_call_with_claim_
  ingress_v1` (calls/member_route.rs:146) requires
  `StaticResultPublicationIngressPortV1` on the lowering port;
  `StaticReceiver` -> `take_static_result_publication_
  ingress_v1` -> `Selected(handoff)` ->
  `lower_selected_static_result_publication_v1`.
- Port coverage: `StaticResultPublicationIngressPortV1` is
  implemented only for `RawInvocationChildPortV1` (reaches
  `module_port`), `RawLegacyChildLoweringPortV1` (Unavailable),
  and `RawStructuredChildScope` (delegates to child). The
  callable-loop source lane lowers through
  `CallableLoopSourceExpressionPortV1`, which carries only
  `&Rc<RefCell<CallableSemanticLoweringState>>` (+ the S3
  declared-instance locator) — `module_port` never reaches it.
- Gap (root cause): inside the loop lane the same source call
  `JsonLine.find(...)` flows
  `lower_value_input` -> `exact_source_method_call` ->
  `CoreEffectPlan::GlobalCall`
  (helpers_value/lower.rs:241-246). The emitted call targets
  the canonical symbol, but the publication `Selected` row is
  never consumed — same structural bypass shape as D15's
  locator gap, one authority over.
- Consumer shape check: the loop port is `Copy`, so a
  `&mut ModuleDraftCollectorV1` cannot be threaded; the owner
  would need a shared-mutability scope (same pattern the S3
  locator adopted via `RefCell<BTreeSet>`) or a pre-resolved
  per-site handoff sealed at `into_physical_input` time.

## Decision

Static `Owner.method` call sites inside selected callable loop
bodies are covered by the same static-result-publication
authority. The loop-source lane must consult the publication
ingress for an armed cataloged site and route `Selected`
handoffs through the sole physical consumer
(`lower_selected_static_result_publication_v1`), never a
bare `GlobalCall` that leaves the row unconsumed.

Source authority + canonical issuer:
  `ModuleDraftCollectorV1::take_static_result_publication_handoff`
  -> `VerifiedStaticCallResultPublicationOwnerV1::
  take_for_source` (sole consumption boundary);
  physical consumer stays
  `lower_selected_static_result_publication_v1`.
Non-authority: loop-lane `CoreEffectPlan::GlobalCall`
  emission for a site that has a `Selected` publication row;
  ad-hoc row skipping.
Fail-fast boundary: armed cataloged site + take error ->
  `StaticResultPublicationIngressErrorV1` propagates —
  no silent `GlobalCall` fallback on the armed lane.
Smallest next slice: S4 — give the loop-source port a
  copyable publication-ingress scope (e.g. owner behind
  `RefCell` on the collector, mirroring the S3 locator
  shape, or a pre-resolved take issued at
  `into_physical_input`); one port hook consulted by the
  `exact_source_call` value-position arm before emitting;
  `Selected` -> `lower_selected_static_result_publication_
  v1`, `TargetOnly`/other decisions keep existing semantics.
Non-claims: VM `ingest/1` ledger-less spine (unchanged);
  `ConditionalUpdateIf` F2 boundary; statement-position
  static calls unless the same arm covers them; Gates 2-4;
  overall completion.

## Boundary of this census

- Covers: `StaticResultPublicationResidual(UnconsumedSelected)`
  at drain on the json_stream_aggregator EXE lane, from
  publication issue to consumption.
- Excludes: VM ledger-less lane (`ingest` — recorded
  non-claim); `ConditionalUpdateIf` F2 parts boundary;
  qualified/script-direct static handoffs (different
  ingresses, already consumed or Unavailable on this lane).

## Exit

- [x] Unconsumed publication rows identified by call site and
  intended consumer — `[Body(2), LoopBody(0), Initializer(0)]`
  `JsonLine.find/3`; intended consumer is the sole
  `take_static_result_publication_ingress_v1` ->
  `lower_selected_static_result_publication_v1` chain.
- [x] Existing owner vs unclaimed gap classified — owner exists
  (`VerifiedStaticCallResultPublicationOwnerV1` on
  `ModuleDraftCollectorV1`); the gap is port reachability, not
  an unclaimed authority.
- [x] One bounded S-card emitted — S4
  `MIRBUILDER-EXE-ACCEPTANCE-SELECTED-STATIC-CALL-LOOP-PUBLICATION-S4`.
