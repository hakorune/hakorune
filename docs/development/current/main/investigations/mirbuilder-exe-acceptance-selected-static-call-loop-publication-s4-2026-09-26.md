# MIRBUILDER-EXE-ACCEPTANCE-SELECTED-STATIC-CALL-LOOP-PUBLICATION-S4

Status: landed__2026-09-26
Date: 2026-09-26
Parent: workstream row H (unified resume, gate 1);
  SELECTED-STATIC-CALL-COVERAGE-D16 decision accepted
Mode: fast — one responsibility, one production edge.

## Responsibility

`Owner.method` static call sites inside selected callable loop
bodies consult the static-result-publication ingress, consume
their `Selected` publication row, and route through the sole
physical consumer `lower_selected_static_result_publication_v1`
— the same physical meaning the body lane produces via
`build_member_method_call_with_claim_ingress_v1` ->
`take_static_result_publication_ingress_v1` ->
`Selected(handoff)`.

## Boundary

The take->install->consume machinery already exists; this
slice widens its arming and lowers every armed callable-loop
route through the publication-aware emission port.

- `source_target_for_loop` (`raw_loop_child_port.rs`) takes
  each `Selected` handoff from
  `ModuleLoweringPortV1::take_static_result_publication_handoff`
  and installs it into the callable ledger
  (`install_source_static_result_publication`) — but only
  when `consume_publication` is armed. The gate widens from
  composite-candidate-only to every armed loop-source
  candidate (`source_loop_items(site).is_some()`), so
  LoopCond/LoopTrue/direct loop_break lanes take their rows.
- `CallableLoopSourceExpressionPortV1` stays untouched: the
  normalizer's existing `exact_source_method_call` ->
  `take_source_core_method_call` already returns the
  `static_publication` projection and emits
  `CoreEffectPlan::GlobalCall` with the canonical target and
  handoff result type — no new effect variant.
- `loop_break` / `LoopCondReady` / `LoopTrueReady` switch
  `PlanLowerer::lower` ->
  `PlanLowerer::lower_with_source_publication`, so the
  `SourcePublication` emission port consumes each installed
  handoff through the sole physical consumer
  `lower_selected_static_result_publication_with_arguments_
  and_destination_v1` and `finish()` enforces no pendings.
- `RawInvocationChildPortV1::lower_loop` gains a post-lower
  residual check: any taken-but-unemitted handoff fails with
  `[callable-loop/static-publication/residual-after-lower]`,
  covering routes that bypass the plan emission port
  (e.g. `variable_accum::lower`).
- `take_for_source` remains the sole consumption boundary;
  exactly-once is enforced by the owner's own
  `consumed_sites` ledger and the ledger's
  `consumed_source_static_result_publications` set.

## Fail-fast boundaries

- Armed lane + exact-target site with no `Selected` row:
  `[callable-loop/static-publication/no-selected-handoff]`;
  take/declaration errors propagate as
  `[callable-loop/static-publication/take|catalog-missing]`
  — never a silent `GlobalCall` that skips the row.
- Taken handoff left pending after lowering:
  `[callable-loop/static-publication/residual-after-lower]`.
- Unarmed lanes keep the existing
  `exact_source_method_call` -> `GlobalCall` path —
  compatibility/test ports are unaffected.

## Non-claims

- VM `ingest/1` ledger-less spine (unchanged, recorded).
- `ConditionalUpdateIf` F2 parts boundary.
- Statement-position static calls in loop bodies unless the
  same arm covers them; qualified/script-direct ingresses
  (separate handoff families).
- Gates 2-4; overall MirBuilder completion.

## Evidence

- `cargo check --lib` / `cargo check --tests`: clean.
- EXE boundary smoke
  (`tools/smokes/v2/profiles/integration/apps/json_stream_aggregator_exe_runtime_boundary.sh`,
  debug binary): `StaticResultPublicationResidual
  (UnconsumedSelected{JsonLine.find/3 @ [Body(2), LoopBody(0),
  Initializer(0)]})` no longer fires — the probe takes the
  Selected row on the armed LoopCond lane, installs it into
  the ledger, `exact_source_method_call` projects it, and the
  `SourcePublication` emission port consumes it. MIR
  compilation of every cataloged function now completes; the
  run reaches the emit-phase named-array retention gate and
  stops at the next named residual
  `[freeze:contract][named-array/retained-source-required]`
  (`me.users.push(user)` write-obligation markers in
  `statsFor` — a different authority family, surfaced only
  because compilation now reaches JSON emit).
- Suite (`real-apps-exe-boundary`, debug binary): 4 pass /
  8 fail — same count as baseline; `json` terminal advanced
  (publication residual -> named-array retention), the two
  typed_object fails are LLVM/mem2reg environment errors,
  boxtorrent/binary_trees/mimalloc stop at the previously
  recorded `SourceCallOutsideSelectedFamily`,
  allocator_stress at `CoreMethodSource` issue.
- VM lane: unchanged recorded non-claim — `ingest/1` stays on
  the ledger-less spine decline.
- Focused tests: `callable_loop_source` 67/67,
  `static_result_publication` 16/16, `loop_break_composite`
  9/9 (existing end-to-end take->install->consume coverage),
  `emission_port` 4/4.
- Red classification — all reproduced on the pre-change HEAD,
  baseline debt, not S4 regressions:
  - `raw_loop_child` 16 passed / 2 failed
    (`unarmed_nested_loop_stops_at_source_facts_without_
    compatibility_reentry`,
    `candidate_without_callable_ledger_is_terminal_before_
    builder_effects`).
  - `source_loop_bridge` 2 passed / 3 failed
    (`callable_state_reports_unarmed_disposition_for_nested_
    loop`,
    `nested_scope_loop_does_not_abort_callable_source_bridge`,
    `unarmed_nested_loop_take_is_a_typed_disposition`).
  - previously classified: `normalizer`
    `demo_if2_valuejoin_emits_phi_and_return`, `verifier`
    `public_resolved_route_produces_verifier_clean_mir`.
- `cargo fmt --check`: clean on all touched files (remaining
  diffs are pre-existing drift in untouched code).
- Guards: `mirbuilder_qualified_route_scope_guard.sh` ok (S4
  section: widened arming gate, >=4
  `lower_with_source_publication` routes, post-lower residual
  check); `current_state_pointer_guard.sh` ok.
