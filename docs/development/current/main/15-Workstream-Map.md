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
| Now | `phase-102 real-app read_quoted loop regression (VM + LLVM EXE)` |
| Front | `MiniJsonLoader.read_quoted_from 最小抽出 fixture を VM/LLVM EXE parity で固定する` |
| Blocker | `none` |
| Next | `phase-102 fixture + parity proof` |
| After Next | `execution SSOT cleanup corridor` |

## Current Read

- `launcher.hako emit_mir_mainline` is green
- `stage1_mainline_smoke.sh` is green
- `95` fixed `apps/tests/phase95_json_loader_escape_min.hako` as strict VM E2E (`hello" world`)
- `96` pins strict VM to the explicit VM lane (`NYASH_VM_HAKO_PREFER_STRICT_DEV=0`) for the `next_non_ws` fixture smoke
- `97` fixed LLVM EXE parity for `phase95/96` fixtures under `HAKO_BACKEND_COMPAT_REPLAY=harness`
- `98` fixed plugin loader strict/best-effort runtime proof and kept phase-97 parity green
- `99` trailing-backslash fixture is already green on VM and LLVM EXE; next work is broader trim/escape fixture expansion
- `100` landed with pinned read-only captures and accumulator parity proof
- `102` is the current lane for real-app `read_quoted_from` loop parity
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
- current work has handed off to `phase-102`

## Successor Corridor

1. `phase-102 real-app read_quoted loop regression (VM + LLVM EXE)`
2. `phase-110x selfhost execution vocabulary SSOT`
3. `phase-111x selfhost runtime route naming cleanup`
4. `phase-112x vm-family lane naming hardening`
5. `phase-113x kernel vs vm-reference cluster wording correction`

## Parked After Optimization

- `vm-hako` small reference interpreter recut

## Structural Stop Lines

- `rust-vm`
  - mainline retirement: achieved
  - residual explicit keep: frozen
- `vm-hako`
  - reference/conformance keep

## Planned Execution SSOT Split

- `stage`
  - artifact generation / historical phase naming only
- `route`
  - end-to-end shell/runtime path (`runtime/mainline`, `runtime/compat`, `direct/proof`)
- `backend override`
  - explicit CLI family (`llvm`, `vm`, `vm-hako`)
- `lane`
  - concrete VM-family implementation (`rust-vm-keep`, `vm-hako-reference`, `vm-compat-fallback`)
- `kernel`
  - reserved for `nyash_kernel`; `lang/src/vm` is VM/reference cluster, not product kernel

## Reference

- current lane docs:
  - `docs/development/current/main/phases/phase-100/README.md`
  - `docs/development/current/main/phases/phase-102/README.md`
- recent landed:
  - `docs/development/current/main/phases/phase-99/README.md`
  - `docs/development/current/main/phases/phase-98/README.md`
  - `docs/development/current/main/phases/phase-97/README.md`
  - `docs/development/current/main/phases/phase-96/README.md`
  - `docs/development/current/main/phases/phase-95/README.md`
