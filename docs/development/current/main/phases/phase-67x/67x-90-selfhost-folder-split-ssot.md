---
Status: Landed
Date: 2026-04-04
Scope: split tools/selfhost into folder-level lanes after phase-66x ranking.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-66x/66x-90-next-source-lane-selection-ssot.md
---

# 67x-90 Selfhost Folder Split SSOT

## Intent

- turn the folder-first corridor into actual tree structure
- make mainline, proof, compat, and lib ownership obvious from the path alone
- avoid another docs-only cleanup wave by making the split the deliverable

## Starting Read

- `tools/selfhost/` still mixes mainline, proof, compat, and library helpers at the top level
- `run.sh`, `selfhost_build.sh`, and `stage1_mainline_smoke.sh` are daily/mainline-facing
- `run_stageb_compiler_vm.sh`, `selfhost_smoke.sh`, `selfhost_vm_smoke.sh`, and `selfhost_stage3_accept_smoke.sh` are proof-facing
- `run_stage1_cli.sh` is compatibility-facing
- `lib/` already holds shared shell helpers and should stay shared

## Current Inventory

### Mainline-facing candidates

- `tools/selfhost/run.sh`
- `tools/selfhost/selfhost_build.sh`
- `tools/selfhost/build_stage1.sh`
- `tools/selfhost/stage1_mainline_smoke.sh`

### Proof-facing candidates

- `tools/selfhost/run_stageb_compiler_vm.sh`
- `tools/selfhost/bootstrap_selfhost_smoke.sh`
- `tools/selfhost/selfhost_smoke.sh`
- `tools/selfhost/selfhost_stage3_accept_smoke.sh`
- `tools/selfhost/selfhost_vm_smoke.sh`

### Compat-facing candidates

- `tools/selfhost/run_stage1_cli.sh`
- `tools/archive/legacy-selfhost/engineering/legacy_main_readiness.sh`
- `tools/archive/legacy-selfhost/engineering/pre_promote_legacy_main_removal.sh`

### Shared / keep-shared

- `tools/selfhost/lib/**`
- `tools/selfhost/examples/**`
- top-level `README.md`
- the small `*.hako` fixtures that are not route owners

## Target Layout

```text
tools/selfhost/
  README.md
  mainline/
  proof/
  compat/
  lib/
  examples/
```

## Current Progress

- `67xA1` landed: exact top-level inventory is fixed
- `67xA2` landed: split order is ranked as `mainline -> proof/compat -> alias cleanup`
- `67xB1` landed: mainline-facing scripts gained canonical homes under `mainline/`
- `67xB2` landed: proof/compat scripts gained canonical homes under `proof/` and `compat/`
- `67xC1` active: top-level aliases are now thin wrappers and live callers/docs are being narrowed

## Decision Rule

- if a script is day-to-day mainline, it should live under `mainline/`
- if a script is proof-only or vm-gated, it should live under `proof/`
- if a script is an explicit compatibility wrapper, it should live under `compat/`
- shared shell helpers stay in `lib/`

## Big Tasks

1. `67xA1` selfhost folder inventory lock
2. `67xA2` target layout ranking
3. `67xB1` top-level selfhost split
4. `67xB2` proof/compat split
5. `67xC1` lib/alias cleanup
6. `67xD1` proof / closeout
