# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-05-19
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

- active lane: read `active_lane` in `CURRENT_STATE.toml`
- active phase: read `active_phase` in `CURRENT_STATE.toml`
- latest card: read `latest_card_path` in `CURRENT_STATE.toml`
- task breakdown: read `taskboard` in `CURRENT_STATE.toml`
- method anchor / design SSOT: read `method_anchor` in `CURRENT_STATE.toml`
- current blocker token: read `current_blocker_token` in `CURRENT_STATE.toml`
- phase-137x: observe-only unless app work reopens a real blocker

## Restart Handoff

- latest landed card: read `latest_card_path` in `CURRENT_STATE.toml`
- latest known checkpoint: read `latest_card` / `latest_card_path` in
  `CURRENT_STATE.toml`; `291x-691` remains the historical warning-backlog
  inventory baseline
- current no-growth baseline: `classifiers=0 rows=0`; no `.inc`
  method/box string classifiers are allowlisted
- worktree expectation: clean unless the active slice is in progress
- resume point: continue the active phase from `current_blocker_token`,
  `phase_status`, and `latest_card_path` in `CURRENT_STATE.toml`.

## Task Order

- current task source: `CURRENT_STATE.toml` plus the active taskboard
- next active-lane order:
  1. read `current_blocker_token`, `phase_status`, and `latest_card_path` from
     `CURRENT_STATE.toml`
  2. keep allocator-provider activation, host allocator replacement, hooks, and
     `#[global_allocator]` out of scope
  3. migrate hako_alloc non-negative fields only by explicit field-group rows
  4. keep sentinel-bearing indexes signed
  5. keep BoxShape cleanup separate from BoxCount feature rows
- parked lanes:
  - the exact `usize` semantic foundation lane is closed; follow-on mimalloc
    comparison execution is now the active lane
  - allocator-provider ladder remains parked unless host allocator replacement
    is explicitly reopened
  - record defaults / spread / named args / automatic record-to-box copy remain
    parked syntax features
- VM retreat reading: VM is a semantic reference executor, not a product owner
- detailed landed history: phase card files and `CURRENT_STATE.toml`
- keep BoxShape cleanup separate from BoxCount feature rows
- do not add hot inline lowering without proof/evidence gate

## Current Implementation Focus (phase-295x)

- keep mimalloc comparison rows implementation-first: each row must modify at
  least one real `.hako` implementation file under `lang/src/hako_alloc/memory/`
  or `apps/*mimalloc*/*.hako` / `apps/hako-alloc-*/*.hako`
- docs-first ordered work:
  1. lock inline boundary vocabulary in
     `docs/development/current/main/design/inline-plan-ssot.md`
  2. keep `Inline(required)` as verifier-owned required inline contract
     (`Contract(no_alloc)` + `Contract(no_safepoint)` required; fail-fast on
     active required-inline lanes)
  3. continue `.hako` success-path read/write compression in remote-free hot
     paths before widening required-inline transforms
  4. allow required-inline pilot only for selected same-module scalar leaf
     accessors after proof/evidence gates stay green

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

- Active phase: read `active_phase` in `CURRENT_STATE.toml`
- Active taskboard: read `taskboard` in `CURRENT_STATE.toml`
- Active design SSOT: read `method_anchor` in `CURRENT_STATE.toml`
- Bootstrap route SSOT:
  `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
- Perf owner-first policy:
  `docs/development/current/main/design/perf-owner-first-optimization-ssot.md`
- Hotline/CoreMethodContract SSOT:
  `docs/development/current/main/design/hotline-core-method-contract-ssot.md`
