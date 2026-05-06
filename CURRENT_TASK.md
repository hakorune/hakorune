# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-05-07
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
8. `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
   only when returning to phase-29bq

## Current Lane

- active lane: `phase-293x real-app bringup`
- active phase: read `active_phase` in `CURRENT_STATE.toml`
- latest card: read `latest_card_path` in `CURRENT_STATE.toml`
- current blocker token:
  `phase-293x real-app bringup order: BoxTorrent mini -> binary-trees -> mimalloc-lite -> allocator port`
- primary mode: real-app bringup lane
- phase-137x: observe-only unless app work reopens a real blocker

## Restart Handoff

- latest landed card: read `latest_card_path` in `CURRENT_STATE.toml`
- latest known checkpoint: read `latest_card` / `latest_card_path` in
  `CURRENT_STATE.toml`; `291x-691` remains the historical warning-backlog
  inventory baseline
- current no-growth baseline: `classifiers=0 rows=0`; no `.inc`
  method/box string classifiers are allowlisted
- worktree expectation: clean unless the active slice is in progress
- resume point: continue `phase-293x` from the real-app suite; BoxTorrent
  mini, binary-trees, and mimalloc-lite are landed; real allocator port is next
- restart checks: `git status -sb` ->
  `bash tools/checks/current_state_pointer_guard.sh` ->
  `tools/smokes/v2/run.sh --profile integration --suite real-apps --skip-preflight`
  when the next app slice is ready

## Task Order

- current task source: `CURRENT_STATE.toml` plus the phase-293x taskboard
- prior task-order baseline:
  `docs/development/current/main/phases/phase-291x/291x-488-current-task-order-baseline-refresh-card.md`
- detailed landed history: phase-291x card files and
  `docs/development/current/main/CURRENT_STATE.toml`
- next: start the real allocator port after BoxTorrent mini, binary-trees, and
  mimalloc-lite;
  only change compiler acceptance when the app exposes a real blocker
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
- current status: BoxTorrent mini, binary-trees, and mimalloc-lite landed; real
  allocator port is next
- compiler rule: do not hide a real compiler blocker in app code; fix the
  compiler seam structurally when needed

## Detail Pointers

- Real-app bringup phase:
  `docs/development/current/main/phases/phase-293x/README.md`
- Real-app taskboard:
  `docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md`
- Real-app smoke suite:
  `tools/smokes/v2/suites/integration/real-apps.txt`
- Current app checkpoint: read `latest_card_path` in
  `docs/development/current/main/CURRENT_STATE.toml`
- Current app card:
  `docs/development/current/main/phases/phase-293x/293x-003-MIMALLOC-LITE-REAL-APP.md`
- Bootstrap route SSOT:
  `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
- Perf owner-first policy:
  `docs/development/current/main/design/perf-owner-first-optimization-ssot.md`
- Hotline/CoreMethodContract SSOT:
  `docs/development/current/main/design/hotline-core-method-contract-ssot.md`
