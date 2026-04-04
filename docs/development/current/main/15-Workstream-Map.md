---
Status: Active
Date: 2026-04-05
Scope: current mainline / next lane / parked corridor の one-screen map。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
---

# Workstream Map

## Current Lane

| Item | State |
| --- | --- |
| Now | `phase-94 escape route P5b “完全E2E” のための ch 再代入対応` |
| Front | `P5b ch derived-value lowering / escape_cond contract` |
| Blocker | `none` |
| Next | `phase-95x current pointer / SSOT stale-focus correction` |
| After Next | `phase-96x selfhost root wrapper and fixture contraction` |

## Current Read

- `launcher.hako emit_mir_mainline` is green
- `stage1_mainline_smoke.sh` is green
- `80x` is landed; pointer docs are thin again
- `81x` closed with a no-op archive sweep
- `83x` closed as an explicit keep proof for top-level selfhost wrappers
- `84x` landed after repointing Stage1 build/default entry contracts to canonical `entry/*`
- `85x` selected `86x` as the next structural source lane
- `86x` landed with a thinner phase index/current mirror surface
- `87x` landed after refreshing snapshot-pinned runner paths to canonical `facade/*` and `entry/*`
- `88x` landed as a no-op archive/deletion rerun
- `89x` selected `90x current-doc/design stale surface hygiene`
- `90x` thinned stale wrapper/current wording in current/design docs after the latest recuts
- `91x` froze the top-level `.hako` wrapper policy after the latest runner/selfhost recuts
- `92x` closed the proof/compat caller rerun lane against the canonical wrapper homes
- `93x` moved archive-later engineering helpers into `tools/archive/legacy-selfhost/engineering/`
- current work has handed off to the existing `phase-94` task

## Successor Corridor

1. `phase-95x current pointer / SSOT stale-focus correction`
2. `phase-96x selfhost root wrapper and fixture contraction`
3. `phase-97x rust-vm explicit keep hardening`

## Parked After Optimization

- `vm-hako` small reference interpreter recut

## Structural Stop Lines

- `rust-vm`
  - mainline retirement: achieved
  - residual explicit keep: frozen
- `vm-hako`
  - reference/conformance keep

## Reference

- current lane docs:
  - `docs/development/current/main/phases/phase-94/README.md`
- recent landed:
  - `docs/development/current/main/phases/phase-93x/README.md`
  - `docs/development/current/main/phases/phase-92x/README.md`
  - `docs/development/current/main/phases/phase-91x/README.md`
