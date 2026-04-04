---
Status: SSOT
Date: 2026-04-04
Scope: audit remaining rust-vm surfaces after keep-pruning and determine what is truly delete-ready.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-56x/README.md
---

# 57x-90 Rust-VM Delete-Ready Audit / Removal Wave SSOT

## Intent

- move from keep-pruning to deletion readiness
- require explicit caller/replacement proof before removal
- keep `vm-hako` outside this lane as reference/conformance keep

## Canonical Reading

- `phase-56x` left rust-vm as explicit proof/compat keep only.
- `phase-57x` decides which of those keeps are still necessary.
- if a surface still has a defensible proof/compat role, it stays `keep-now`.

## Target Surfaces

- `src/runner/modes/vm.rs`
- `src/runner/modes/vm_fallback.rs`
- `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
- `lang/src/runner/stage1_cli/core.hako`
- `tools/selfhost/run_stageb_compiler_vm.sh`
- residual wrappers/docs/examples around these surfaces

## Required Classification

- `keep-now`
  - still needed for explicit proof/compat behavior with no replacement yet
- `archive-later`
  - not needed on the live path, but still useful as archive/historical evidence
- `delete-ready`
  - caller-zero or replacement-proven and safe to remove in this lane

## Boundaries

- do not widen proof/compat keeps while auditing them
- do not mix `vm-hako` reference/conformance work into rust-vm removal
- keep `cargo check --bin hakorune` and `git diff --check` green

## Success Conditions

- the classification is explicit and source-backed
- any removal in `57xC1` is small and justified by caller/replacement proof
- the lane hands off to successor selection without reopening rust-vm as a live owner path

## First-Pass Inventory Lock

- `keep-now`
  - `src/runner/modes/vm.rs`
  - `src/runner/modes/vm_fallback.rs`
  - `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
  - `lang/src/runner/stage1_cli/core.hako`
  - `tools/selfhost/run_stageb_compiler_vm.sh`
- `archive-later`
  - residual archive/manual-smoke wrappers and historical docs around the explicit keep surfaces
- `delete-ready`
  - none in the first pass; all target surfaces still have live proof/compat callers

## Caller-Zero Audit Result

- `src/runner/modes/vm.rs`
  - still selected by `src/runner/route_orchestrator.rs` for explicit `VmRouteAction::Vm`
  - still exercised by proof/compat scripts and smokes that invoke `--backend vm`
- `src/runner/modes/vm_fallback.rs`
  - still selected by `src/runner/route_orchestrator.rs` for `VmRouteAction::CompatFallback`
  - still guarded by `tools/checks/vm_route_bypass_guard.sh`
- `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
  - still called from `src/runner/modes/common_util/selfhost/stage_a_route.rs`
  - still owns the explicit Program(JSON) compat bridge
- `lang/src/runner/stage1_cli/core.hako`
  - `run_program_json` still remains the raw compat hold line
  - stage1 tests still exercise the raw compat entry
- `tools/selfhost/run_stageb_compiler_vm.sh`
  - still referenced by `tools/selfhost/lib/selfhost_run_routes.sh`
  - still referenced by focused selfhost proof smokes
- first-pass conclusion:
  - no target surface is caller-zero yet
  - `57xB2` must narrow removal candidates before any delete wave

## Removal-Prep Direction

- do not target the five explicit keep surfaces in `57xC1`
- prefer archive/manual-smoke residue and historical wrappers if they are proven disconnected from the live path
- if `57xB2` cannot produce a justified delete-ready set, `57xC1` stays a no-op removal wave and the lane closes honestly
