# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-06-15
Scope: current lane / next lane / restart order only.

## Purpose

- root から active lane に最短で戻る
- landed history は phase docs / investigations を正本にする
- `CURRENT_TASK.md` 自体は ledger にしない

## Quick Restart Pointer

1. `docs/development/current/main/CURRENT_STATE.toml`
2. `docs/development/current/main/phases/phase-296x/296x-666-PARAM-DIRECT-CONSUMER-FORWARDING-CANDIDATE-PROBE-001.md`
3. `docs/development/current/main/design/perf-owner-first-optimization-ssot.md`
4. `docs/development/current/main/phases/phase-296x/296x-661-BOOL-SCALAR-LOWERING-001.md`
5. `docs/development/current/main/phases/phase-293x/293x-1040-COMPILER-FOUNDATION-CHECKPOINT-001.md`
6. `docs/development/current/main/workstreams/compiler-foundation-current.md`
7. `docs/development/current/main/design/compiler-expressivity-first-policy.md`
8. `docs/development/current/main/05-Restart-Quick-Resume.md`
9. `docs/development/current/main/10-Now.md`
10. `git status -sb`
11. `bash tools/checks/current_state_pointer_guard.sh`
12. `tools/checks/dev_gate.sh quick` only when a code slice is ready

## Current Lane

- active lane: read `active_lane` in `CURRENT_STATE.toml`
- active phase: read `active_phase` in `CURRENT_STATE.toml`
- latest card: read `latest_card_path` in `CURRENT_STATE.toml`
- taskboard: read `taskboard` in `CURRENT_STATE.toml`
- method anchor: read `method_anchor` in `CURRENT_STATE.toml`
- blocker token: read `current_blocker_token` in `CURRENT_STATE.toml`

## Status

- implementation_gap_count=0
- active work has moved from boot-amortized exact-kernel selection through
  `MIMALLOC-BODY-TIMING-FRONT-SELECT-001` to
  `EXPRESSION-MATERIALIZATION-COPY-ORIGIN-PROBE-002`,
  `PARAM-EXPRESSION-COPY-CHAIN-POLICY-SELECTION-001`, and now to
  `PARAM-DIRECT-CONSUMER-FORWARDING-CANDIDATE-PROBE-001`; exact resident kernels no
  longer expose a meaningful Hako-slower owner, the product-route
  object-lifecycle body timing surface selected `local_ssa_copy_materialization`,
  current MIR origin attribution selected `param_expression_value_copy_chain`,
  param-chain selection selected `param_direct_consumer_value_forwarding`, and
  the next step is candidate safety classification before implementation
- compiler foundation is paused at
  `docs/development/current/main/phases/phase-293x/293x-1040-COMPILER-FOUNDATION-CHECKPOINT-001.md`
- first foundation owner is BoxCallableRegistry / TypeAbiCatalog reconciliation:
  BoxCallableRegistry is callable truth, TypeAbiCatalog and BoxDescriptor are
  read-only projection/tooling surfaces; `BOXCALL-REG-011` reconciles the
  landed BoxCallableRegistry rows with older TypeAbi BoxDomain rows and names
  narrow proof commands
- selfhost/de-Rust lift decisions are owned by
  `docs/development/current/main/design/selfhost-lift-boundary-and-task-order-ssot.md`:
  meaning goes to `.hako`, route shape / ownership events go to MIRBuilder,
  machine boundaries stay substrate
- immediate selfhost lift order is:
  `BOXCALL-PROVIDER-SOURCE-001` landed slice ->
  `BOXCALL-CATALOG-001` landed slice for existing String / Array / Map
  catalogs ->
  `BUFFER-CATALOG-001` landed slice before Buffer provider rows ->
  `BUFFER-PROVIDER-ROWS-001` landed slice ->
  `BOXCALL-ROUTEPLAN-001` landed slice ->
  `TYPE-REGISTRY-PROVIDER-001` landed slice ->
  `PLUGIN-PROVIDER-SNAPSHOT-001` landed slice ->
  `BOXCALL-FOUNDATION-CLOSEOUT-001` landed slice
- selected next lane: collection visible semantics
- next implementation order:
  `COLL-VISIBLE-000` landed lane card ->
  `BUFFER-VISIBLE-INVENTORY-001` landed Buffer visible method / alias / policy
  inventory with modular `.hako` owner ->
  `BUFFER-VISIBLE-CONTRACT-002` landed fixtures and hako_check report fields ->
  `BUFFER-HAKO-CORE-003` landed first `.hako` Buffer visible owner ->
  `BUFFER-NUMERIC-LE-004` landed typed little-endian numeric policy ->
  `STRING-VISIBLE-INVENTORY-001` landed String method / alias / slot
  inventory with modular `.hako` owner ->
  `STRING-HAKO-POLICY-002` landed first `.hako` String policy owner ->
  `MAP-VISIBLE-CONTRACT-001` landed Map method / alias / slot / effect
  contract with modular `.hako` owner ->
  `ARRAY-VISIBLE-CONTRACT-001` landed Array method / alias / slot / effect
  contract with modular `.hako` owner ->
  `COLL-VISIBLE-CLOSEOUT-001` landed collection visible semantics closeout
  in `docs/development/current/main/phases/phase-293x/293x-1032-COLL-VISIBLE-CLOSEOUT-001.md`;
  next foundation lane returns to CorePlan / JoinIR expressivity ->
  `COREPLAN-ONE-ROW-IMPL-001` landed the first post-closeout CorePlan row:
  LoopBuilderApi PHI insertion routes through `phi_lifecycle`, missing current
  block PHI fallback is fail-fast, and the PHI boundary guard rejects
  low-level lifecycle bypasses ->
  `COREPLAN-VARMAP-RESEAL-001` landed the next BoxShape row: selected
  `parts/**` publishers use `var_map_scope` helpers and the variable_map
  no-growth guard pins the direct insert baseline at 54
- second foundation owner is CorePlan / JoinIR expressivity: B1 remaining
  compatibility normalizer lego-ization has its first SSOT/guard boundary, and
  the C1 planner_required fail-fast, D1 normalizer AST-boundary, E1 active-v0
  inventory, E1-002 first retire, E1-003 collect_using_entries, E1-004
  bundle_resolver, E1-005 scan_v0, E1-006 scan_methods_v0, E1-007
  scan_phi_vars_v0 retire, E1 closeout, COREPLAN-LOOP-WIRING-002 PHI input
  materialization, COREPLAN-PLANNER-TAG-001 generic-loop FlowBox evidence, and
  COREPLAN-TIMEOUT-001 StageB bundle-mod timeout metadata,
  `COREPLAN-PHI-BINDING-SSOT-001`, `COREPLAN-VARMAP-BOUNDARY-001`,
  `COREPLAN-PORT07-TIMEOUT-001`, `COREPLAN-FULL-GATE-DRIFT-001`,
  `COREPLAN-ISINTEGER-STRICT-DRIFT-001`, and the 1026..1030 29ae/full-gate
  drift closeouts and `BOXCALL-REG-011` are landed; active routed loop_*_v0
  count is zero;
  `phase29bq_fast_gate_vm.sh --full` now passes BQ, Hako MIRBuilder pin rows,
  Program JSON contract pin, PORT04, PORT07, the 29ae regression pack, and the
  29bp planner-required dev gate
- local-patch prevention is now an active compiler hygiene rule: same failure
  class plus two local patches means stop-the-line, docs-first boundary audit,
  and guard/fixture before more implementation; `COREPLAN-VARMAP-BOUNDARY-001`
  inventories 62 direct `variable_map` writes under CorePlan/plan/SSA and pins
  a no-growth guard
- optimization resumed through `MIMALLOC-AOT-KERNEL-FRONT-SELECT-002`,
  selected the product-route body timing front in
  `MIMALLOC-BODY-TIMING-FRONT-SELECT-001`, and now continues at
  `EXPRESSION-MATERIALIZATION-COPY-ORIGIN-PROBE-002`,
  `PARAM-EXPRESSION-COPY-CHAIN-POLICY-SELECTION-001`, and now continues at
  `PARAM-DIRECT-CONSUMER-FORWARDING-CANDIDATE-PROBE-001`;
  `kilo_micro_userbox_flag_toggle` remains the landed inline-bool scalar keeper,
  `kilo_micro_userbox_counter_step_chain` remains a startup sentinel, and
  process-total boot cost is diagnostic rather than the primary owner selector
- MIM-PORT-FMEM-005 and MIM-PORT-FMEM-006 are historical Done rows, not the next
  active implementation row
- adjacent array-text session route design is documented, and the selected-route session boundary is now landing
- inspect scope dump design is documented: dump is a `hako_check` query, not `.hako` source syntax
- substring-concat length residual is documented as lowering/codegen work:
  `SUBCONCAT-LEN-CLOSED-FORM-001` should emit scalar closed-form IR from the
  existing StableLengthScalar route; do not return to `.hako` or MIRBuilder for
  this slice
- compiler cleanup had a BoxShape-only clean-enough closeout:
  `docs/development/current/main/phases/phase-291x/291x-792-compiler-cleanliness-clean-enough-closeout-card.md`
  and the nav/dead-dust follow-up is
  `docs/development/current/main/phases/phase-291x/291x-793-compiler-cleanliness-nav-dead-dust-closeout-card.md`
- concurrency/thread pre-selfhost work is a side-lane candidate, not the active
  lane: `CONC-RUNTIME-INVENTORY-001`, `CONC-SCHED-ROUTE-001`,
  `CONC-CAP-INVENTORY-001`, `CONC-SYNCBOX-003`, `CONC-CHANNEL-002`,
  `CONC-CHANNEL-003`, `CONC-CONTEXT-002`, and
  `CONC-SOURCE-PARALLEL-001` are landed through docs/report vocabulary;
  `worker_scope workers=N { parallel ... }` is reserved design-only, `workers=N`
  is an upper-bound scheduler hint, and parser/lowering remain gated on
  `THREAD-SAFETY-001`; keep `nowait_os_thread_spawn=0` and do not add
  source-level thread syntax before send/share/thread-root safety is pinned
- Arc retirement is a side-lane taskboard, not the active optimization lane:
  `docs/development/current/main/design/arc-retirement-and-ownership-substrate-ssot.md`
  and `docs/development/current/main/workstreams/arc-retirement-current.md`
  define the task order; first-family host-handle text payload cutover is
  already tracked there, but global Arc / object substrate replacement is still
  closed from the compiler-foundation pointer alone
- compiler-first checkpoint is landed; the previous compiler-first sequence was
  `COREPLAN-NEXT-ROW-SELECTION-001` in
  `docs/development/current/main/phases/phase-293x/293x-1035-COREPLAN-NEXT-ROW-SELECTION-001.md`;
  it selected `COREPLAN-VARMAP-RESEAL-002`; `COREPLAN-VARMAP-RESEAL-002`
  is landed in
  `docs/development/current/main/phases/phase-293x/293x-1036-COREPLAN-VARMAP-RESEAL-002-GENERIC-LOOP-BODY.md`;
  `COREPLAN-PHI-TXN-001` is landed in
  `docs/development/current/main/phases/phase-293x/293x-1037-COREPLAN-PHI-TXN-001.md`;
  `COREPLAN-JOINIR-MERGE-PHI-001` is landed in
  `docs/development/current/main/phases/phase-293x/293x-1038-COREPLAN-JOINIR-MERGE-PHI-001.md`;
  `COREPLAN-NORMALIZER-COMPOSITION-001` is landed in
  `docs/development/current/main/phases/phase-293x/293x-1039-COREPLAN-NORMALIZER-COMPOSITION-001.md`;
  `COMPILER-FOUNDATION-CHECKPOINT-001` is landed in
  `docs/development/current/main/phases/phase-293x/293x-1040-COMPILER-FOUNDATION-CHECKPOINT-001.md`
- treat stale Active labels in phase history as historical unless the current_state says otherwise

## Rules

- keep BoxShape and BoxCount separate
- do not grow restart docs with landed chronology
- point to archive/investigation notes instead of copying long queues
- update `CURRENT_STATE.toml` and the active card first

## Read Next

1. `docs/development/current/main/CURRENT_STATE.toml`
2. `docs/development/current/main/phases/phase-296x/296x-666-PARAM-DIRECT-CONSUMER-FORWARDING-CANDIDATE-PROBE-001.md`
3. `docs/development/current/main/design/perf-owner-first-optimization-ssot.md`
4. `docs/development/current/main/phases/phase-296x/296x-661-BOOL-SCALAR-LOWERING-001.md`
5. `docs/development/current/main/phases/phase-293x/293x-1040-COMPILER-FOUNDATION-CHECKPOINT-001.md`
6. `docs/development/current/main/workstreams/compiler-foundation-current.md`
7. `docs/development/current/main/design/compiler-expressivity-first-policy.md`
8. `docs/development/current/main/design/current-docs-update-policy-ssot.md`
