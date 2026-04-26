# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-04-27
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

- active lane: `phase-291x CoreBox surface contract cleanup`
- active phase: read `active_phase` in `CURRENT_STATE.toml`
- latest card: read `latest_card_path` in `CURRENT_STATE.toml`
- current blocker token: `phase-291x normalized-shadow shared expression implementation extraction pending`
- primary mode: compiler cleanup lane
- phase-137x: observe-only unless app work reopens a real blocker

## Restart Handoff

- latest landed card: read `latest_card_path` in `CURRENT_STATE.toml`
- latest known checkpoint: `291x-396` added a normalized-shadow support facade
  for shared expression lowering and migrated route lowerers away from legacy
  helper imports
- current no-growth baseline: `classifiers=0 rows=0`; no `.inc`
  method/box string classifiers are allowlisted
- worktree expectation: clean unless the active slice is in progress
- resume point: physically move shared expression helper implementation from
  legacy storage into the support facade
- restart checks: `git status -sb` ->
  `bash tools/checks/current_state_pointer_guard.sh` ->
  `tools/checks/dev_gate.sh quick` when the next slice is ready

## Task Order

- current task source:
  `docs/development/current/main/phases/phase-291x/291x-396-normalized-shadow-shared-expr-facade-card.md`
- detailed landed history: phase-291x card files and
  `docs/development/current/main/CURRENT_STATE.toml`
- next: normalized-shadow shared expression implementation extraction
- keep BoxShape cleanup separate from BoxCount feature rows
- keep Stage-B adapter thinning separate from CoreMethodContract migration
- do not add hot inline lowering without proof/evidence gate
- do not reopen landed CoreBox router rows without an owner-path change

## Current Ordered Cleanup

- latest cleanup card: read `latest_card_path` in
  `docs/development/current/main/CURRENT_STATE.toml`
- latest checkpoint: `291x-396`; detailed landed history lives in phase card
  files and the compact `landed_tail` in `CURRENT_STATE.toml`
- next cleanup: normalized-shadow shared expression implementation extraction
- keep these cleanup cards BoxShape-only; do not change bundle semantics, do
  not reuse legacy `entry/bundle_resolver.hako`, and do not reopen
  CoreMethodContract fallback rows

## Detail Pointers

- CoreBox surface phase:
  `docs/development/current/main/phases/phase-291x/README.md`
- CoreBox design brief:
  `docs/development/current/main/phases/phase-291x/291x-90-corebox-surface-catalog-design-brief.md`
- StringBox taskboard:
  `docs/development/current/main/phases/phase-291x/291x-91-stringbox-surface-task-board.md`
- CoreBox inventory:
  `docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md`
- Smoke index:
  `docs/development/current/main/phases/phase-291x/291x-smoke-index.md`
- Perf owner-first policy:
  `docs/development/current/main/design/perf-owner-first-optimization-ssot.md`
- Hotline/CoreMethodContract SSOT:
  `docs/development/current/main/design/hotline-core-method-contract-ssot.md`
