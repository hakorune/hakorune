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

1. `phase-65x stage1/selfhost mainline hardening` (landed)
2. `phase-66x next source lane selection` (landed)
3. `phase-67x selfhost folder split` (landed)
4. `phase-68x .hako runner authority/compat/facade recut` (landed)
5. `phase-69x rust runner product/keep/reference recut` (landed)
6. `phase-70x caller-zero archive sweep` (landed)
7. `phase-71x next source lane selection` (landed)
8. `phase-72x selfhost top-level facade thinning` (landed)
9. `phase-73x emit_mir_mainline blocker follow-up` (landed)
10. `phase-74x next source lane selection` (landed)
11. `phase-75x selfhost top-level alias canonicalization` (landed)
12. `phase-76x next source lane selection` (landed)
13. `phase-77x runner top-level pressure thinning` (landed)
14. `phase-78x next source lane selection` (landed)
15. `phase-79x launcher emit_mir residual blocker follow-up` (landed)
16. `phase-80x root/current pointer thinning` (landed)
17. `phase-81x caller-zero archive rerun` (active)

## Current Front

- Active lane: `phase-81x caller-zero archive rerun`
- Active micro: `81xA1 caller inventory rerun`
- Current blocker: `none`
- Exact focus: rerun live caller facts after the folder/pointer cleanup and confirm whether any wrapper or alias became archive-ready

## Successor Corridor

1. `phase-81x caller-zero archive rerun`

## Rust-VM Stop Line

- mainline retirement: achieved
- full source retirement: deferred
- residual explicit keep: frozen
- `vm-hako`: reference/conformance keep

## Read Next

1. `docs/development/current/main/05-Restart-Quick-Resume.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/phases/phase-81x/README.md`
4. `docs/development/current/main/phases/phase-81x/81x-90-caller-zero-archive-rerun-ssot.md`
5. `docs/development/current/main/phases/phase-81x/81x-91-task-board.md`

## Notes

- `phase-80x` is landed.
- `launcher.hako emit_mir_mainline` is green.
- `stage1_mainline_smoke.sh` is green.
- `target/**`, `artifacts/**`, and `dist/**` are artifact roots only.
