# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-06-09
Scope: current lane / next lane / restart order only.

## Purpose

- root から active lane に最短で戻る
- landed history は phase docs / investigations を正本にする
- `CURRENT_TASK.md` 自体は ledger にしない

## Quick Restart Pointer

1. `docs/development/current/main/CURRENT_STATE.toml`
2. `docs/development/current/main/investigations/docs-pointer-inventory-2026-06-09.md`
3. `docs/development/current/main/investigations/hako-vs-c-mimalloc-direct-exact-comparison-2026-06-09.md`
4. `docs/development/current/main/05-Restart-Quick-Resume.md`
5. `docs/development/current/main/10-Now.md`
6. `git status -sb`
7. `bash tools/checks/current_state_pointer_guard.sh`
8. `tools/checks/dev_gate.sh quick` only when a code slice is ready

## Current Lane

- active lane: read `active_lane` in `CURRENT_STATE.toml`
- active phase: read `active_phase` in `CURRENT_STATE.toml`
- latest card: read `latest_card_path` in `CURRENT_STATE.toml`
- taskboard: read `taskboard` in `CURRENT_STATE.toml`
- method anchor: read `method_anchor` in `CURRENT_STATE.toml`
- blocker token: read `current_blocker_token` in `CURRENT_STATE.toml`

## Status

- implementation_gap_count=0
- current work is exact-front optimization selection
- treat stale Active labels in phase history as historical unless the current_state says otherwise

## Rules

- keep BoxShape and BoxCount separate
- do not grow restart docs with landed chronology
- point to archive/investigation notes instead of copying long queues
- update `CURRENT_STATE.toml` and the active card first

## Read Next

1. `docs/development/current/main/CURRENT_STATE.toml`
2. `docs/development/current/main/investigations/docs-pointer-inventory-2026-06-09.md`
3. `docs/development/current/main/design/current-docs-update-policy-ssot.md`
