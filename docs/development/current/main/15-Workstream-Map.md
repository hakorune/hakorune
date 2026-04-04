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
| Now | `phase-95 json_loader escape loop E2E lock` |
| Front | `json_loader escape loop fixture / strict VM proof` |
| Blocker | `none` |
| Next | `phase-96 MiniJsonLoader next_non_ws loop E2E lock` |
| After Next | `phase-97 LLVM EXE parity for MiniJsonLoader fixtures` |

## Current Read

- `launcher.hako emit_mir_mainline` is green
- `stage1_mainline_smoke.sh` is green
- `94` fixed `tools/selfhost/test_pattern5b_escape_minimal.hako` as strict VM E2E (`hello" world`)
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
- current work has handed off to the existing `phase-95` task

## Successor Corridor

1. `phase-96 MiniJsonLoader next_non_ws loop E2E lock`
2. `phase-97 LLVM EXE parity for MiniJsonLoader fixtures`

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
  - `docs/development/current/main/phases/phase-95/README.md`
- recent landed:
  - `docs/development/current/main/phases/phase-94/README.md`
  - `docs/development/current/main/phases/phase-93x/README.md`
