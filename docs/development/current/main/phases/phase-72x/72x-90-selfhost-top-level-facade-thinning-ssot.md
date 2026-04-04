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
