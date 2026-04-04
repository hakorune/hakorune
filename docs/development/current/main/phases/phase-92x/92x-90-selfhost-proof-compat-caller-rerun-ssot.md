---
Status: Active
Date: 2026-04-05
Scope: rerun selfhost proof/compat callers after the top-level wrapper policy freeze.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-91x/README.md
  - lang/src/runner/README.md
---

# 92x-90 Selfhost Proof/Compat Caller Rerun SSOT

## Purpose

- rerun proof/compat callers against the canonical wrapper homes
- confirm the top-level `.hako` wrappers remain thin public/front-door keeps
- keep proof/compat reruns explicit and non-growing

## In Scope

- `tools/selfhost/mainline/stage1_mainline_smoke.sh`
- `tools/selfhost/proof/run_stageb_compiler_vm.sh`
- `tools/selfhost/proof/selfhost_smoke.sh`
- `tools/selfhost/proof/selfhost_vm_smoke.sh`
- `tools/selfhost/proof/bootstrap_selfhost_smoke.sh`
- `tools/selfhost/proof/selfhost_stage3_accept_smoke.sh`
- `tools/selfhost/compat/run_stage1_cli.sh`
- `tools/selfhost/run.sh`
- `lang/src/runner/README.md`

## Policy

- proof/compat reruns should stay explicit and non-growing
- top-level wrappers remain thin public/front-door keeps
- canonical homes stay under `compat/`, `facade/`, and `entry/`

## Out of Scope

- rust-vm retirement
- vm-hako reference/conformance lane
- archive/deletion sweeps
- new wrapper ownership growth
