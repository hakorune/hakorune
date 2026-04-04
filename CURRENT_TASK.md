# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-04-05
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
12. `phase-90x current-doc/design stale surface hygiene` (landed)
13. `phase-91x top-level .hako wrapper policy review` (landed)
14. `phase-92x selfhost proof/compat caller rerun` (landed)
15. `phase-93x archive-later engineering helper sweep` (landed)
16. `phase-94 escape route P5b “完全E2E” のための ch 再代入対応` (active)

## Current Front

- Active lane: `phase-94 escape route P5b “完全E2E” のための ch 再代入対応`
- Active micro: `TBD`
- Current blocker: `none`
- Exact focus: move legacy engineering helpers into the archive bucket and keep current/doc mirrors thin

## Successor Corridor

1. `phase-94 escape route P5b “完全E2E” のための ch 再代入対応`

## Rust-VM Stop Line

- mainline retirement: achieved
- full source retirement: deferred
- residual explicit keep: frozen
- `vm-hako`: reference/conformance keep

## Read Next

1. `docs/development/current/main/05-Restart-Quick-Resume.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/phases/phase-93x/README.md`
4. `docs/development/current/main/phases/phase-94/README.md`

## Notes

- `phase-88x` confirmed there are still no true archive-ready/delete-ready wrapper surfaces.
- `phase-89x` selected current/design stale surface hygiene as the next structural lane.
- `phase-90x` thinned current/design stale surface wording and is now landed.
- `phase-91x` froze the top-level `.hako` wrapper policy and is now landed.
- `phase-92x` reruns selfhost proof/compat callers against the canonical wrapper homes.
- `phase-93x` moved archive-later engineering helpers out of `tools/selfhost/` and into `tools/archive/legacy-selfhost/engineering/`.
- `launcher.hako emit_mir_mainline` is green.
- `stage1_mainline_smoke.sh` is green.
- `target/**`, `artifacts/**`, and `dist/**` are artifact roots only.
