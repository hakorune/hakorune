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
5. `phase-83x selfhost top-level facade/archive decision` (landed)
6. `phase-84x runner wrapper/source contract thinning` (landed)
7. `phase-85x next source lane selection` (landed)
8. `phase-86x phase index / current mirror hygiene` (landed)
9. `phase-87x embedded snapshot / wrapper repoint rerun` (landed)
10. `phase-88x archive/deletion rerun` (landed)
11. `phase-89x next source lane selection` (landed)
12. `phase-90x current-doc/design stale surface hygiene` (active)

## Current Front

- Active lane: `phase-90x current-doc/design stale surface hygiene`
- Active micro: `90xC1 proof refresh`
- Current blocker: `none`
- Exact focus: thin current/design docs that still describe old wrapper or current surfaces too noisily after the latest runner/selfhost recuts

## Successor Corridor

1. `phase-90x current-doc/design stale surface hygiene`
2. `phase-91x top-level .hako wrapper policy review`
3. `phase-92x selfhost proof/compat caller rerun`

## Rust-VM Stop Line

- mainline retirement: achieved
- full source retirement: deferred
- residual explicit keep: frozen
- `vm-hako`: reference/conformance keep

## Read Next

1. `docs/development/current/main/05-Restart-Quick-Resume.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/phases/phase-90x/README.md`
4. `docs/development/current/main/phases/phase-90x/90x-90-current-doc-design-stale-surface-hygiene-ssot.md`
5. `docs/development/current/main/phases/phase-90x/90x-91-task-board.md`

## Notes

- `phase-88x` confirmed there are still no true archive-ready/delete-ready wrapper surfaces.
- `phase-89x` selected current/design stale surface hygiene as the next structural lane.
- `launcher.hako emit_mir_mainline` is green.
- `stage1_mainline_smoke.sh` is green.
- `target/**`, `artifacts/**`, and `dist/**` are artifact roots only.
