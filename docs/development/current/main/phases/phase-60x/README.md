---
Status: Landed
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

## Inventory Lock

- proof-only keep:
  - `tools/selfhost/run_stageb_compiler_vm.sh`
    - explicit Stage-B proof gate; guarded by `NYASH_SELFHOST_STAGEB_PROOF_ONLY=1`
    - still owned by `tools/selfhost/lib/selfhost_run_routes.sh` direct proof path and Stage-B proof smokes
  - `tools/selfhost/bootstrap_selfhost_smoke.sh`
    - bootstrap parity smoke keep; still the c0/c1/c1' proof entry
  - `tools/selfhost/selfhost_smoke.sh`
    - explicit selfhost proof smoke; still exercises the compat/proof emit path
  - `tools/selfhost/selfhost_stage3_accept_smoke.sh`
    - stage3 acceptance proof keep; still bridges `--direct` producer and `--ny-parser-pipe --backend vm` consumer
  - `tools/plugin_v2_smoke.sh`
    - plugin-host proof keep; still the current explicit plugin compatibility smoke
- compat keep:
  - `tools/selfhost/lib/selfhost_run_routes.sh`
    - `stage-a-compat` remains the explicit shell compat entry; still calls `--backend vm`
  - `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
    - still owns Program(JSON v0) compat resolution and explicit Program(JSON)->MIR fallback
  - `src/runner/modes/vm_fallback.rs`
    - still owns the explicit `NYASH_VM_USE_FALLBACK=1` interpreter lane
  - `lang/src/runner/stage1_cli/core.hako`
    - still owns the raw Program(JSON) compat hold line and `vm|pyvm` accept / `llvm` reject policy
- explicit out-of-scope keep:
  - `src/runner/modes/vm.rs`
  - `src/runner/modes/vm_hako.rs`
  - `src/runner/dispatch.rs`
  - `src/runner/route_orchestrator.rs`
  - these stay outside `60x`; the lane only prunes proof/compat keeps and does not reopen broad owner removal

## Success Conditions

- proof-only keeps stay explicit and non-growing
- compat keeps stay explicit and non-default
- `cargo check --bin hakorune` and `git diff --check` stay green
- focused proofs stay green:
  - `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_route_smoke_vm.sh`
  - `bash tools/selfhost/selfhost_smoke.sh`
  - `bash tools/selfhost/bootstrap_selfhost_smoke.sh`

## Big Tasks

1. inventory/freeze the remaining keep surfaces
   - `60xA1` proof/compat keep inventory lock
   - `60xA2` compat keep boundary freeze
2. prune the live keep surfaces
   - `60xB1` stage-a compat seam pruning
     - landed: direct MIR acceptance now stays in `stage_a_route.rs`
     - compat bridge now owns Program(JSON) fallback only
   - `60xB2` vm_fallback/core.hako keep pruning continuation
     - landed: caller-zero helper `execute_vm_fallback_from_ast(...)` removed from `vm_fallback.rs`
     - `core.hako` remains a thin no-widen compat keep
   - `60xC1` proof smoke keep pruning continuation
3. prove and close
   - `60xD1` proof / closeout

## Retirement Corridor

- `60x` does not claim full retirement by itself.
- fixed follow-up corridor:
  1. `61x residual rust-vm caller-zero audit rerun`
  2. `62x rust-vm delete-ready removal wave`
  3. `63x rust-vm final retirement decision`
- expected reading:
  - full rust-vm retirement is only realistic after `60x` narrows the explicit keep bucket and `61x/62x` prove delete-ready facts

## Result

- `60xA1` landed: proof/compat keep inventory lock
- `60xA2` landed: compat keep boundary freeze
- `60xB1` landed: stage-a compat seam pruning
- `60xB2` landed: vm_fallback/core.hako keep pruning continuation
- `60xC1` landed: proof smoke keep pruning continuation
- `60xD1` landed: proof / closeout
- handoff:
  - next lane is `phase-61x residual rust-vm caller-zero audit rerun`
