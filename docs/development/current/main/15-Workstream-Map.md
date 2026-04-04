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
| Now | `phase-97 LLVM EXE parity for MiniJsonLoader fixtures` |
| Front | `escape / next_non_ws fixture parity under LLVM EXE` |
| Blocker | `compat replay=harness narrows compile; LLVM EXE runtime still returns wrong output` |
| Next | `phase-97 focused source/runtime parity fix` |
| After Next | `parked / review after LLVM parity` |

## Current Read

- `launcher.hako emit_mir_mainline` is green
- `stage1_mainline_smoke.sh` is green
- `95` fixed `apps/tests/phase95_json_loader_escape_min.hako` as strict VM E2E (`hello" world`)
- `96` pins strict VM to the explicit VM lane (`NYASH_VM_HAKO_PREFER_STRICT_DEV=0`) for the `next_non_ws` fixture smoke
- `97` pins LLVM compile to `HAKO_BACKEND_COMPAT_REPLAY=harness`; remaining blocker is runtime parity under LLVM EXE
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
- current work has handed off to the existing `phase-97` task

## Successor Corridor

1. `phase-97 LLVM EXE parity for MiniJsonLoader fixtures`

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
  - `docs/development/current/main/phases/phase-97/README.md`
- recent landed:
  - `docs/development/current/main/phases/phase-96/README.md`
  - `docs/development/current/main/phases/phase-95/README.md`
  - `docs/development/current/main/phases/phase-94/README.md`
  - `docs/development/current/main/phases/phase-93x/README.md`
