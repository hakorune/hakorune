---
Status: Landed
Date: 2026-04-04
Scope: prune proof/compat keep surfaces after route-surface retirement has landed, without entering delete-ready removal yet.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-55x/README.md
  - docs/development/current/main/phases/phase-55x/55x-90-rust-vm-route-surface-retirement-prep-ssot.md
---

# Phase 56x: Proof/Compat Keep Pruning

## Goal

- reduce the remaining proof-only / compat keep surfaces without removing the explicit keeps yet
- narrow `stage-a`, `vm_fallback`, `core.hako`, and proof smoke ownership to the smallest still-defensible surface
- prepare `phase-57x` delete-ready audit by separating true keep-now payloads from pruneable residue

## Plain Reading

- `phase-55x` retired route/default/help exposure that made rust-vm look selectable.
- `phase-56x` does not delete rust-vm yet.
- this lane only prunes keep surfaces that are still broader than necessary.
- `vm-hako` remains reference/conformance and stays out of scope.

## Focus Surfaces

- `tools/selfhost/lib/selfhost_run_routes.sh` (`stage-a` compat branch)
- `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
- `src/runner/modes/vm_fallback.rs`
- `lang/src/runner/stage1_cli/core.hako`
- `tools/selfhost/run_stageb_compiler_vm.sh`
- proof smoke wrappers that still imply broader keep ownership than necessary

## Success Conditions

- proof-only keeps stay explicit and non-growing
- compat keeps stay explicit and non-growing
- pruneable wording/helpers are removed from keep surfaces
- `cargo check --bin hakorune` and `git diff --check` stay green

## Failure Patterns

- deleting explicit keeps before the delete-ready audit exists
- mixing `vm-hako` reference/conformance work into rust-vm pruning
- widening `stage-a`, fallback, or proof smoke ownership while trying to prune them

## Big Tasks

1. lock keep inventory and freeze boundaries
   - `56xA1` proof-only keep inventory lock
   - `56xA2` compat keep boundary freeze
2. prune keep surfaces
   - `56xB1` stage-a compat route pruning prep
   - `56xB2` vm fallback/core.hako keep pruning
   - `56xC1` proof smoke keep pruning
3. prove and close
   - `56xD1` proof / closeout

## Outcome

- proof-only and compat keeps are now explicitly bounded and non-growing
- proof smoke wrappers no longer overstate rust-vm as a day-to-day owner lane
- the lane hands off to `phase-57x rust-vm delete-ready audit / removal wave`
