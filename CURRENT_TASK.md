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
16. `phase-94 escape route P5b “完全E2E” のための ch 再代入対応` (landed)
17. `phase-95 json_loader escape loop E2E lock` (landed)
18. `phase-96 MiniJsonLoader next_non_ws loop E2E lock` (landed)
19. `phase-97 LLVM EXE parity for MiniJsonLoader fixtures` (landed)
20. `phase-98 Plugin loader fail-fast + LLVM parityの持続化` (landed)
21. `phase-99 Trim/escape 実コード寄り強化（VM+LLVM EXE）` (active)

## Current Front

- Active lane: `phase-99 Trim/escape 実コード寄り強化（VM+LLVM EXE）`
- Active micro: `phase96/97 fixture を実コード寄りケースへ広げ、VM+LLVM EXE parity を維持する`
- Current blocker: `none`
- Exact focus: `next_non_ws の追加ケースと escape 末尾バックスラッシュ fixture を VM+LLVM EXE の両方で固定する`

## Successor Corridor

1. `phase-99 Trim/escape 実コード寄り強化（VM+LLVM EXE）`

## Parked After Optimization

- `phase-kx vm-hako small reference interpreter recut`
  - keep `vm-hako` as reference/conformance only
  - do not promote to product/mainline
  - revisit after the optimization corridor, not before

## Rust-VM Stop Line

- mainline retirement: achieved
- full source retirement: deferred
- residual explicit keep: frozen
- `vm-hako`: reference/conformance keep

## Read Next

1. `docs/development/current/main/05-Restart-Quick-Resume.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/phases/phase-98/README.md`

## Notes

- `phase-98` locked plugin loader strict/best-effort runtime contract and kept `phase-97` LLVM EXE parity green.
- `phase-97` fixed LLVM EXE parity for `phase95/96` fixtures under `HAKO_BACKEND_COMPAT_REPLAY=harness`.
- `launcher.hako emit_mir_mainline` is green.
- `stage1_mainline_smoke.sh` is green.
- thin top-level wrappers remain public/front-door keep, not archive-ready by default.
- `vm-hako` stays reference/conformance keep; future interpreter recut is parked until after optimization work.
