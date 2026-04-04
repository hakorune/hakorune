---
Status: Active
Date: 2026-04-04
Scope: continue pruning proof/compat keeps after phase-59x narrowed the remaining rust-vm route/default/help surfaces.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-59x/README.md
  - docs/development/current/main/phases/phase-59x/59x-90-rust-vm-route-surface-retirement-continuation-ssot.md
  - docs/development/current/main/phases/phase-59x/59x-91-task-board.md
---

# Phase 60x: Proof/Compat Keep Pruning Continuation

## Goal

- keep `rust-vm` off the mainline by tightening the remaining explicit proof/compat keeps
- prune wording, wrappers, and narrow compat seams without claiming broad source deletion yet
- leave `vm-hako` reference/conformance work out of scope

## Focus Surfaces

- `tools/selfhost/run_stageb_compiler_vm.sh`
- `tools/selfhost/bootstrap_selfhost_smoke.sh`
- `tools/selfhost/selfhost_smoke.sh`
- `tools/selfhost/selfhost_stage3_accept_smoke.sh`
- `tools/plugins/plugin_v2_smoke.sh`
- `tools/selfhost/lib/selfhost_run_routes.sh` compat branch
- `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
- `src/runner/modes/vm_fallback.rs`
- `lang/src/runner/stage1_cli/core.hako`

## Success Conditions

- proof-only keeps stay explicit and non-growing
- compat keeps stay explicit and non-default
- `cargo check --bin hakorune` and `git diff --check` stay green

## Big Tasks

1. inventory/freeze the remaining keep surfaces
   - `60xA1` proof/compat keep inventory lock
   - `60xA2` compat keep boundary freeze
2. prune the live keep surfaces
   - `60xB1` stage-a compat seam pruning
   - `60xB2` vm_fallback/core.hako keep pruning continuation
   - `60xC1` proof smoke keep pruning continuation
3. prove and close
   - `60xD1` proof / closeout
