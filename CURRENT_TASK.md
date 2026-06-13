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
- current work is the compiler foundation lane named by `CURRENT_STATE.toml`;
  exact-front optimization is paused until this lane reaches a closeout or an
  explicit pause point
- first foundation owner is BoxCallableRegistry / TypeAbiCatalog reconciliation:
  BoxCallableRegistry is callable truth, TypeAbiCatalog and BoxDescriptor are
  read-only projection/tooling surfaces
- second foundation owner is CorePlan / JoinIR expressivity: B1 remaining
  compatibility normalizer lego-ization has its first SSOT/guard boundary, and
  the remaining family inventory now orders the next slices as C1
  planner_required fail-fast guard, D1 normalizer AST-boundary inventory, then
  E1 active-v0 inventory/retire work
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
8. `docs/development/current/main/design/coreplan-compat-normalizer-legoization-ssot.md`
9. `docs/development/current/main/design/current-docs-update-policy-ssot.md`
