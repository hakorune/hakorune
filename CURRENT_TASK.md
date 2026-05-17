# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-05-17
Scope: current lane / next lane / restart order only.

## Purpose

- root から active lane / next lane に最短で戻る
- landed history と rejected history は phase docs / investigations を正本にする
- `CURRENT_TASK.md` 自体は ledger にしない

## Current Docs Policy

- Current-state SSOT: `docs/development/current/main/CURRENT_STATE.toml`
- Update policy SSOT:
  `docs/development/current/main/design/current-docs-update-policy-ssot.md`
- Normal card work must not append landed history here.
- Per-card updates are limited to `CURRENT_STATE.toml` latest-card fields and
  the active card unless lane / blocker / restart order / durable policy changes.

## Quick Restart Pointer

1. `docs/development/current/main/CURRENT_STATE.toml`
2. `docs/development/current/main/05-Restart-Quick-Resume.md`
3. `docs/development/current/main/10-Now.md`
4. Read `active_phase`, `phase_status`, `method_anchor`, `taskboard`, and
   `latest_card_path` from `CURRENT_STATE.toml`
5. `git status -sb`
6. `bash tools/checks/current_state_pointer_guard.sh`
7. `tools/checks/dev_gate.sh quick` when a code slice is ready
   (`allocator-wide` is explicit only for allocator/mimalloc/provider closeout)
8. `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
   only when returning to phase-29bq

## Current Lane

- active lane: `phase-293x mimalloc blueprint lane`
- active phase: read `active_phase` in `CURRENT_STATE.toml`
- latest card: read `latest_card_path` in `CURRENT_STATE.toml`
- task breakdown:
  `docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md`
- mimalloc blueprint SSOT:
  `docs/development/current/main/design/mimalloc-hakorune-blueprint-task-breakdown-ssot.md`
- mimalloc port purpose:
  `docs/development/current/main/design/mimalloc-hako-port-purpose-ssot.md`
- mimalloc blueprint / port taskboard:
  `docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md`
- allocator-first granularity SSOT:
  `docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md`
- pure-first MIR artifact / diagnostics SSOT:
  `docs/development/current/main/design/pure-first-mir-artifact-and-diagnostics-ssot.md`
- mimalloc / Hakorune joint task order:
  `docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md`
- current blocker token:
  `MIMAP-087A post-segment-page-membership-closeout row selection`
- current BoxShape sidecar:
  read `latest_card_path`, `phase_status`, and `landed_tail` in
  `CURRENT_STATE.toml`, plus the phase-293x taskboard. Do not paste landed
  sidecar history into this root pointer.
- primary mode: mimalloc substrate implementation lane; keep upstream source
  untracked and keep each allocator row behind explicit guards before provider
  activation
- phase-137x: observe-only unless app work reopens a real blocker

## Restart Handoff

- latest landed card: read `latest_card_path` in `CURRENT_STATE.toml`
- latest known checkpoint: read `latest_card` / `latest_card_path` in
  `CURRENT_STATE.toml`; `291x-691` remains the historical warning-backlog
  inventory baseline
- current no-growth baseline: `classifiers=0 rows=0`; no `.inc`
  method/box string classifiers are allowlisted
- worktree expectation: clean unless the active slice is in progress
- resume point: continue Phase 293x after `MIMAP-086A`; the next selected
  blocker is `MIMAP-087A`, the post-segment-page-membership-closeout row
  selection.
  VM-LIM-001 remains parked diagnostic.
  Keep LoopRange on the Stage1 route; do not source-desugar range loops.

## Task Order

- current task source: `CURRENT_STATE.toml` plus the phase-293x taskboard
- next 293x order:
  1. `MIMAP-087A`: select exactly one next row after the scalar segment page
     membership closeout
  2. keep real thread scheduling, worker spawning, source-level concurrency features,
     page-source calls, OSVM release, and provider
     activation inactive
  3. keep secure entropy execution parked until a separate random substrate
     route and audit row are accepted
- post-mimalloc selfhost order:
  `SELFHOST-POST-MIMAP-001` is parked for broad Stage1 `.hako` owner
  reduction after mimalloc completeness evidence. Do not make broad `.hako`
  parser/mirbuilder migration a prerequisite for current mimalloc rows.
- recent BoxShape sidecar:
  MIRBUILDER-DIET builder core / FlowPlanner boundary cleanup closed through MIR-SEMANTIC-PLANS-001
- metadata promotion queue:
  `docs/reference/mir/metadata-facts-ssot.md` `Current Promotion Matrix`
  and
  `docs/development/current/main/phases/phase-293x/293x-369-METADATA-CATALOG-003-PROMOTION-MATRIX.md`
  are now historical entries for the landed promotion wave through
  `METADATA-PROMOTE-006`. Future metadata work must use owner-triggered rows
  from `docs/reference/mir/metadata-facts-ssot.md`; do not combine metadata
  cleanup cards with allocator behavior rows.
- optional future allocator-provider ladder:
  `docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md`
  and `docs/development/current/main/design/allocator-provider-post-m101-implementation-ladder-ssot.md`
  remain parked unless host allocator replacement is explicitly reopened.
- detailed landed history: phase card files and `CURRENT_STATE.toml`
- VM retreat reading: new substrate / allocator features should target
  `llvm/exe` / pure-first acceptance first; `vm-hako` is reference/monitor only
  and `rust-vm` is bootstrap/recovery/compat keep, so broad VM parity is not a
  default requirement for new rows
- MIR structural dead-shelf cleanup is closed through `291x-791`; the obsolete
  standalone MIR hints scaffold is retired and that audited MIR vocabulary set
  no longer carries a broad dead-code hold
- normalized-shadow / normalization cleanup burst is closed; larger findings
  must move to a new lane
- keep BoxShape cleanup separate from BoxCount feature rows
- keep Stage-B adapter thinning separate from CoreMethodContract migration
- do not add hot inline lowering without proof/evidence gate
- do not reopen landed CoreBox router rows without an owner-path change

## Current Ordered App Bringup

- latest app card: read `latest_card_path` in
  `docs/development/current/main/CURRENT_STATE.toml`
- order:
  1. BoxTorrent mini
  2. binary-trees
  3. mimalloc-lite
  4. real allocator port
  5. allocator-stress app
  6. BoxTorrent allocator-backed store
  7. JSON stream aggregator
- current status: BoxTorrent mini, binary-trees, mimalloc-lite, the
  `hako_alloc` VM-only page/free-list port, allocator-stress, BoxTorrent
  allocator-backed store, and JSON stream aggregator landed; direct EXE now
  lowers typed-object allocation/field slots, the BoxTorrent `firstChunkId` /
  `refCount` module-generic seam, BoxTorrent mini user-box string field
  returns, global-call handle param metadata, substring handle result
  publication, recursive same-module user-box method bodies, typed-object
  handle global-call returns, allocator handle param-origin inference, and
  explicit same-module PHI type preservation; BoxTorrent mini, binary-trees,
  JSON stream aggregator, mimalloc-lite, and allocator-stress direct EXE
  parity now exit 0
- compiler rule: do not hide a real compiler blocker in app code; fix the
  compiler seam structurally when needed

## Detail Pointers

- Real-app bringup phase:
  `docs/development/current/main/phases/phase-293x/README.md`
- Language-minimal taskboard:
  `docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md`
- Real-app smoke suite:
  `tools/smokes/v2/suites/integration/real-apps.txt`
- Real-app EXE boundary suite:
  `tools/smokes/v2/suites/integration/real-apps-exe-boundary.txt`
- Current app checkpoint: read `latest_card_path` in
  `docs/development/current/main/CURRENT_STATE.toml`
- Current app card: read `latest_card_path` in
  `docs/development/current/main/CURRENT_STATE.toml`
- Bootstrap route SSOT:
  `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
- Perf owner-first policy:
  `docs/development/current/main/design/perf-owner-first-optimization-ssot.md`
- Hotline/CoreMethodContract SSOT:
  `docs/development/current/main/design/hotline-core-method-contract-ssot.md`
