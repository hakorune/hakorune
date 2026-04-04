---
Status: Active
Date: 2026-04-04
Scope: narrow top-level selfhost entry wrappers after the canonical subfolders landed.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-67x/README.md
  - docs/development/current/main/phases/phase-71x/README.md
---

# 72x-90 Selfhost Top-Level Facade Thinning SSOT

## Intent

- keep canonical ownership in `mainline/`, `proof/`, `compat/`, and `lib/`
- reduce top-level wrapper pressure where live callers no longer need broad facade spread
- preserve explicit top-level entrypoints that still help daily usage

## Starting Read

- `tools/selfhost/run.sh` remains the broad day-to-day front door
- `tools/selfhost/selfhost_build.sh` and `tools/selfhost/build_stage1.sh` still carry high live pressure
- several top-level helper/smoke wrappers still mirror canonical files in the subfolders

## Decision Rule

- thin first where canonical subfolder callers already dominate
- keep top-level facades that still act as stable user-facing entrypoints
- do not archive proof-only scripts inside this lane

## 72xA1 Inventory Result

### Front-Door Keep

- `tools/selfhost/run.sh`
- `tools/selfhost/selfhost_build.sh`
- `tools/selfhost/build_stage1.sh`

### Thin-First Candidates

- `tools/selfhost/stage1_mainline_smoke.sh`
- `tools/selfhost/run_stage1_cli.sh`
- `tools/selfhost/run_stageb_compiler_vm.sh`
- `tools/selfhost/bootstrap_selfhost_smoke.sh`
- `tools/selfhost/selfhost_smoke.sh`
- `tools/selfhost/selfhost_stage3_accept_smoke.sh`
- `tools/selfhost/selfhost_vm_smoke.sh`

### Lower-Pressure Helpers

- readiness / sync / generator helpers under `tools/selfhost/*.sh`

## 72xA2 Ranking

1. thin canonical-backed smoke/compat/proof wrappers first
2. leave `run.sh` / `selfhost_build.sh` / `build_stage1.sh` as explicit top-level entrypoints
3. defer low-pressure readiness/sync/generator helpers until after the main facade wave

## 72xB1 Result

- first-wave canonical-backed wrappers are already thin exec facades:
  - `stage1_mainline_smoke.sh`
  - `run_stage1_cli.sh`
  - `run_stageb_compiler_vm.sh`
  - `bootstrap_selfhost_smoke.sh`
  - `selfhost_smoke.sh`
  - `selfhost_stage3_accept_smoke.sh`
  - `selfhost_vm_smoke.sh`
- no extra source thinning was needed in the first wave
- `72xC1` should therefore focus on pointer honesty and top-level reading, not forced wrapper churn

## 72xC1 / 72xD1 Closeout Read

- current pointers now match the no-op first-wave result
- adjacent top-level `.hako` wrappers are also already thin compatibility wrappers
- successor work should move to the tracked `emit_mir_mainline` blocker
