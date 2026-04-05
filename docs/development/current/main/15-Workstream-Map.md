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
| Now | `phase-121x vm backend retirement gate decision` |
| Front | `--backend vm` を public explicit gate のまま残すか internal-only へ狭めるかを blocker ベースで判断する` |
| Blocker | `none` |
| Next | `args.rs` / compat route / proof gate / docs examples が gate decision をどこで止めているかを固定する` |
| After Next | `phase-122x vm compat route exit plan` |

## Current Read

- `launcher.hako emit_mir_mainline` is green
- `stage1_mainline_smoke.sh` is green
- `95` fixed `apps/tests/phase95_json_loader_escape_min.hako` as strict VM E2E (`hello" world`)
- `96` pins strict VM to the explicit VM lane (`NYASH_VM_HAKO_PREFER_STRICT_DEV=0`) for the `next_non_ws` fixture smoke
- `97` fixed LLVM EXE parity for `phase95/96` fixtures under `HAKO_BACKEND_COMPAT_REPLAY=harness`
- `98` fixed plugin loader strict/best-effort runtime proof and kept phase-97 parity green
- `99` trailing-backslash fixture is already green on VM and LLVM EXE; next work is broader trim/escape fixture expansion
- `100` landed with pinned read-only captures and accumulator parity proof
- `102` landed with real-app `read_quoted_from` loop parity
- `103` landed with if-only merge / early return parity
- `104` landed with loop(true)+break-only digits parity on VM and LLVM EXE
- `105` restored the original long digit OR-chain parity on VM and LLVM EXE
- `112x` hardened vm-family lane naming to `rust-vm-keep / vm-hako-reference / vm-compat-fallback`
- `111x` made live selfhost runtime smokes and current docs route-first (`--runtime-route mainline|compat`)
- `110x` fixed long-lived execution vocabulary SSOT and corrected stale wording in `lang/src/vm` / `tools/selfhost`
- `113x` reserved `kernel` for `nyash_kernel` and fixed `lang/src/vm` as a VM/reference cluster
- `114x` made public/help surface read `mainline route` vs `explicit keep/reference override`
- `115x` inventory is now split into `compat route`, `proof wrappers`, and `debug/observability`
- `116x` is shrinking alias pressure first: `stage-a` goes away, `runtime-route compat` and `runtime-mode stage-a-compat` stay
- `117x` is hardening compat ingress next: shell preflight must require `NYASH_VM_USE_FALLBACK=1`
- `118x` is narrowing proof wrapper surface next: public proof surface stays small, bootstrap/acceptance helpers stay internal
- `119x` now narrows the remaining vm-family debug/observability keep: route observability stays live, generic probe pressure should thin further
- `120x` now refreshes the retirement order across `compat / proof / debug-observability` before any broader backend decision
- `121x` now decides whether `--backend vm` can shrink from public explicit gate to internal-only, or whether concrete blockers still keep it public
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
- current work is now on `phase-115x vm route retirement planning`

## Successor Corridor

1. `phase-121x vm backend retirement gate decision`
2. `phase-122x vm compat route exit plan`
3. `phase-123x proof gate shrink follow-up`

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
  - `docs/development/current/main/phases/phase-121x/README.md`
  - `docs/development/current/main/phases/phase-119x/README.md`
  - `docs/development/current/main/phases/phase-120x/README.md`
  - `docs/development/current/main/phases/phase-118x/README.md`
  - `docs/development/current/main/phases/phase-117x/README.md`
  - `docs/development/current/main/phases/phase-116x/README.md`
  - `docs/development/current/main/phases/phase-114x/README.md`
- recent landed:
  - `docs/development/current/main/phases/phase-103/README.md`
  - `docs/development/current/main/phases/phase-102/README.md`
  - `docs/development/current/main/phases/phase-100/README.md`
  - `docs/development/current/main/phases/phase-98/README.md`
  - `docs/development/current/main/phases/phase-99/README.md`
