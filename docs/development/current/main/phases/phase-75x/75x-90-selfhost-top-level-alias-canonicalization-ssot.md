---
Status: Landed
Date: 2026-04-04
Scope: canonicalize remaining top-level selfhost aliases after phase-67x folder split.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-67x/README.md
  - docs/development/current/main/phases/phase-74x/README.md
---

# 75x-90 Selfhost Top-Level Alias Canonicalization SSOT

## Intent

- make `tools/selfhost` read canonically from the tree itself
- keep top-level front doors thin and explicit
- avoid reopening broad docs-only cleanup

## Initial Read

- canonical buckets already exist:
  - `tools/selfhost/mainline/`
  - `tools/selfhost/proof/`
  - `tools/selfhost/compat/`
  - `tools/selfhost/lib/`
- remaining pressure is top-level alias duplication such as:
  - `tools/selfhost/build_stage1.sh`
  - `tools/selfhost/stage1_mainline_smoke.sh`
  - `tools/selfhost/run_stage1_cli.sh`
  - `tools/selfhost/run_stageb_compiler_vm.sh`
  - `tools/selfhost/selfhost_smoke.sh`
  - `tools/selfhost/selfhost_stage3_accept_smoke.sh`
  - `tools/selfhost/selfhost_vm_smoke.sh`
  - `tools/selfhost/bootstrap_selfhost_smoke.sh`

## Decision Rule

- keep only front-door wrappers that still improve entry/readability
- move callers to canonical folder paths first
- archive/delete only after caller-zero is proven

## Current Read

- `75xA1` landed:
  - live non-doc callers were concentrated on `tools/selfhost/run_stageb_compiler_vm.sh`
  - top-level wrappers such as `build_stage1.sh`, `stage1_mainline_smoke.sh`, and `run_stage1_cli.sh` were already thin front-door keep shims
- `75xA2` landed:
  - keep-now front-door wrappers stay at the top level
  - canonical execution homes remain `mainline/`, `proof/`, and `compat/`
- `75xB1` landed:
  - live proof callers now point at `tools/selfhost/proof/run_stageb_compiler_vm.sh`
  - top-level `tools/selfhost/run_stageb_compiler_vm.sh` is reduced to front-door compatibility only
- `75xB2` landed:
  - top-level selfhost wrappers stay as explicit front-door keep
  - canonical execution homes remain `tools/selfhost/mainline/`, `tools/selfhost/proof/`, and `tools/selfhost/compat/`
- `75xD1` landed:
  - proof kept `cargo check --bin hakorune`, `git diff --check`, alias `--help`, and a focused Stage-B proof smoke green
