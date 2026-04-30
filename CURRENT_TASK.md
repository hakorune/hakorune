# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-04-30
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

- active lane: `phase-29ci Program(JSON v0) public compat retirement`
- active phase: read `active_phase` in `CURRENT_STATE.toml`
- latest card: read `latest_card_path` in `CURRENT_STATE.toml`
- current blocker token: `phase-29ci raw compat caller inventory pending`
- primary mode: Program(JSON v0) public-surface cleanup lane
- phase-137x: observe-only unless app work reopens a real blocker

## Restart Handoff

- latest landed card: read `latest_card_path` in `CURRENT_STATE.toml`
- latest known checkpoint: read `latest_card` / `latest_card_path` in
  `CURRENT_STATE.toml`; `291x-691` remains the historical warning-backlog
  inventory baseline
- current no-growth baseline: `classifiers=0 rows=0`; no `.inc`
  method/box string classifiers are allowlisted
- worktree expectation: clean unless the active slice is in progress
- resume point: continue `phase-29ci` raw compat caller inventory after the
  P6 public alias retirement
- restart checks: `git status -sb` ->
  `bash tools/checks/current_state_pointer_guard.sh` ->
  `tools/checks/dev_gate.sh quick` when the next slice is ready

## Task Order

- current task source: `CURRENT_STATE.toml` plus the latest phase-29ci card
- prior task-order baseline:
  `docs/development/current/main/phases/phase-291x/291x-488-current-task-order-baseline-refresh-card.md`
- detailed landed history: phase-291x card files and
  `docs/development/current/main/CURRENT_STATE.toml`
- next: inventory raw compat flag callers (`--emit-program-json-v0` and
  `--program-json-to-mir`) before any broader Program(JSON) deletion
- MIR structural dead-shelf cleanup is closed through `291x-791`; the obsolete
  standalone MIR hints scaffold is retired and that audited MIR vocabulary set
  no longer carries a broad dead-code hold
- normalized-shadow / normalization cleanup burst is closed; larger findings
  must move to a new lane
- keep BoxShape cleanup separate from BoxCount feature rows
- keep Stage-B adapter thinning separate from CoreMethodContract migration
- do not add hot inline lowering without proof/evidence gate
- do not reopen landed CoreBox router rows without an owner-path change

## Current Ordered Cleanup

- latest cleanup card: read `latest_card_path` in
  `docs/development/current/main/CURRENT_STATE.toml`
- latest checkpoint: read `latest_card_path` in `CURRENT_STATE.toml`; detailed
  landed history lives in phase card files
- next cleanup: Program(JSON v0) public compat retirement is reopened through
  `phase-29ci`; `--hako-emit-program-json` is retired, and raw compat flag
  callers must be inventoried before deletion
- normalized-shadow / normalization cleanup burst is closed; larger findings
  must move to a new lane
- keep these cleanup cards BoxShape-only; do not change bundle semantics, do
  not reuse legacy `entry/bundle_resolver.hako`, and do not reopen
  CoreMethodContract fallback rows

## Detail Pointers

- Program(JSON v0) boundary retirement phase:
  `docs/development/current/main/phases/archive/phase-29ci/README.md`
- Current cleanup checkpoint: read `latest_card_path` in
  `docs/development/current/main/CURRENT_STATE.toml`
- Route vocabulary card:
  `docs/development/current/main/phases/phase-29ci/P6-STAGE1-MIR-ROUTE-VOCABULARY.md`
- Bootstrap route SSOT:
  `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
- Perf owner-first policy:
  `docs/development/current/main/design/perf-owner-first-optimization-ssot.md`
- Hotline/CoreMethodContract SSOT:
  `docs/development/current/main/design/hotline-core-method-contract-ssot.md`
