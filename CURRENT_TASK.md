# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-04-04
Scope: repo root から current lane / next lane / restart read order に最短で戻るための薄い anchor。

## Purpose

- root から current lane と current front を最短で読む
- 長い landed history や implementation detail は phase docs を正本にする
- `CURRENT_TASK.md` は pointer に徹し、ledger にはしない

## Quick Restart Pointer

1. `docs/development/current/main/05-Restart-Quick-Resume.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `git status -sb`
4. `tools/checks/dev_gate.sh quick`

## Order At A Glance

1. `phase-79x launcher emit_mir residual blocker follow-up` (landed)
2. `phase-80x root/current pointer thinning` (landed)
3. `phase-81x caller-zero archive rerun` (landed)
4. `phase-82x next source lane selection` (landed)
5. `phase-83x selfhost top-level facade/archive decision` (active)

## Current Front

- Active lane: `phase-83x selfhost top-level facade/archive decision`
- Active micro: `83xA1 top-level facade inventory lock`
- Current blocker: `none`
- Exact focus: classify top-level `tools/selfhost/*` wrappers into explicit front-door keeps vs true archive-ready aliases

## Successor Corridor

1. `phase-83x selfhost top-level facade/archive decision`

## Rust-VM Stop Line

- mainline retirement: achieved
- full source retirement: deferred
- residual explicit keep: frozen
- `vm-hako`: reference/conformance keep

## Read Next

1. `docs/development/current/main/05-Restart-Quick-Resume.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/phases/phase-83x/README.md`
4. `docs/development/current/main/phases/phase-83x/83x-90-selfhost-top-level-facade-archive-decision-ssot.md`
5. `docs/development/current/main/phases/phase-83x/83x-91-task-board.md`

## Notes

- `phase-82x` selected the top-level selfhost facade/archive decision lane.
- `launcher.hako emit_mir_mainline` is green.
- `stage1_mainline_smoke.sh` is green.
- `target/**`, `artifacts/**`, and `dist/**` are artifact roots only.
