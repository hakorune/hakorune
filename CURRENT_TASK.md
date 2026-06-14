# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-06-14
Scope: current lane / next lane / restart order only.

## Purpose

- root から active lane に最短で戻る
- landed history は phase docs / investigations を正本にする
- `CURRENT_TASK.md` 自体は ledger にしない

## Quick Restart Pointer

1. `docs/development/current/main/CURRENT_STATE.toml`
2. `docs/development/current/main/workstreams/compiler-foundation-current.md`
3. `docs/development/current/main/phases/phase-293x/293x-1004-COMPILER-FOUNDATION-SELECTION-001.md`
4. `docs/development/current/main/design/box-callable-registry-ssot.md`
5. `docs/development/current/main/design/type-abi-catalog-planning-spine-ssot.md`
6. `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`
7. `docs/development/current/main/design/compiler-expressivity-first-policy.md`
8. `docs/development/current/main/design/local-patch-prevention-ssot.md`
9. `docs/development/current/main/05-Restart-Quick-Resume.md`
10. `docs/development/current/main/10-Now.md`
11. `git status -sb`
12. `bash tools/checks/current_state_pointer_guard.sh`
13. `tools/checks/dev_gate.sh quick` only when a code slice is ready

## Current Lane

- active lane: read `active_lane` in `CURRENT_STATE.toml`
- active phase: read `active_phase` in `CURRENT_STATE.toml`
- latest card: read `latest_card_path` in `CURRENT_STATE.toml`
- taskboard: read `taskboard` in `CURRENT_STATE.toml`
- method anchor: read `method_anchor` in `CURRENT_STATE.toml`
- blocker token: read `current_blocker_token` in `CURRENT_STATE.toml`

## Status

- implementation_gap_count=0
- current work is the compiler foundation lane named by `CURRENT_STATE.toml`;
  exact-front optimization is paused until this lane reaches a closeout or an
  explicit pause point
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
  `BOXCALL-FOUNDATION-CLOSEOUT-001` landed slice; next lane requires explicit
  selection between collection visible semantics and CorePlan / JoinIR
  expressivity before moving on
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
- optimization resumes later at `MIMALLOC-AOT-KERNEL-FRONT-SELECT-002`;
  `kilo_micro_userbox_flag_toggle` remains the landed inline-bool scalar keeper,
  and `kilo_micro_userbox_counter_step_chain` remains a startup sentinel
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
  define the task order; do not start Arc replacement implementation from this
  pointer alone
- treat stale Active labels in phase history as historical unless the current_state says otherwise

## Rules

- keep BoxShape and BoxCount separate
- do not grow restart docs with landed chronology
- point to archive/investigation notes instead of copying long queues
- update `CURRENT_STATE.toml` and the active card first

## Read Next

1. `docs/development/current/main/CURRENT_STATE.toml`
2. `docs/development/current/main/workstreams/compiler-foundation-current.md`
3. `docs/development/current/main/phases/phase-293x/293x-1004-COMPILER-FOUNDATION-SELECTION-001.md`
4. `docs/development/current/main/design/box-callable-registry-ssot.md`
5. `docs/development/current/main/design/type-abi-catalog-planning-spine-ssot.md`
6. `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`
7. `docs/development/current/main/phases/phase-293x/293x-1006-COREPLAN-FOUND-002-REMAINING-FAMILY-INVENTORY.md`
8. `docs/development/current/main/phases/phase-293x/293x-1021-COREPLAN-PHI-BINDING-SSOT-001.md`
9. `docs/development/current/main/phases/phase-293x/293x-1022-COREPLAN-VARMAP-BOUNDARY-001.md`
10. `docs/development/current/main/phases/phase-293x/293x-1023-COREPLAN-PORT07-TIMEOUT-001.md`
11. `docs/development/current/main/phases/phase-293x/293x-1024-COREPLAN-FULL-GATE-DRIFT-001.md`
12. `docs/development/current/main/phases/phase-293x/293x-1031-BOXCALL-REG-011-SSOT-LADDER-RECONCILIATION.md`
13. `docs/development/current/main/phases/phase-293x/293x-1030-JOINIR-STRICT-HELPER-ROUTE-PIN-001.md`
14. `docs/development/current/main/design/coreplan-compat-normalizer-legoization-ssot.md`
15. `docs/development/current/main/design/current-docs-update-policy-ssot.md`
