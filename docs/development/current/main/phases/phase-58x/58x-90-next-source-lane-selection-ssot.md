---
Status: SSOT
Date: 2026-04-04
Scope: rank and select the next source lane after phase-57x closed with keep-now rust-vm surfaces intact.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-57x/README.md
---

# 58x-90 Next Source Lane Selection SSOT

## Intent

- choose the next source lane after the phase-57x delete-ready audit
- do not pretend that broad rust-vm deletion is ready when the audit says otherwise
- keep the next lane source-backed and leverage-first

## Inputs from Phase 57x

- `keep-now`
  - `src/runner/modes/vm.rs`
  - `src/runner/modes/vm_fallback.rs`
  - `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
  - `lang/src/runner/stage1_cli/core.hako`
  - `tools/selfhost/run_stageb_compiler_vm.sh`
- `archive-later`
  - residual archive/manual-smoke wrappers and historical docs
- `delete-ready`
  - none in the first pass

## Candidate Lanes

1. `59x rust-vm route-surface retirement continuation`
   - keep shrinking explicit route/default/help exposure without touching keep-now source ownership
2. `60x proof/compat keep pruning continuation`
   - continue narrowing the explicit keep surfaces and their stale contracts
3. `61x rust-vm delete-ready audit rerun`
   - only after more caller drain or replacement work makes real delete-ready candidates possible

## Selection Rule

- prefer the lane that reduces future rust-vm pressure without forcing a fake removal wave
- do not pick a removal lane unless caller-zero or replacement proof materially changes
- keep `vm-hako` reference/conformance work out of scope

## Successor Inventory Lock

- remaining route-surface pressure still exists on explicit affordance sites
  - `src/cli/args.rs` still advertises explicit `vm` / `vm-hako` backend override
  - `src/runner/dispatch.rs` still exposes explicit compat/proof and reference route banners
  - `src/runner/route_orchestrator.rs` still owns explicit `vm` / `vm-hako` plan selection
  - `tools/selfhost/lib/selfhost_run_routes.sh` still executes `stage-a` compat via `--backend vm`
- remaining keep-pruning pressure is narrower and local
  - `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
  - `src/runner/modes/vm_fallback.rs`
  - `lang/src/runner/stage1_cli/core.hako`
  - `tools/selfhost/run_stageb_compiler_vm.sh`
- removal rerun has no new input yet
  - `phase-57x` still ended with no delete-ready core source target

## Candidate Ranking

1. `59x rust-vm route-surface retirement continuation`
   - highest leverage: it attacks the remaining live affordance surface that can re-grow rust-vm pressure
2. `60x proof/compat keep pruning continuation`
   - second: useful, but mostly local wording/contract narrowing after the route/default pressure is handled
3. `61x rust-vm delete-ready audit rerun`
   - lowest: premature until caller/replacement facts materially change

## Decision

- select `59x rust-vm route-surface retirement continuation`
- reason:
  - route/default/help surfaces still expose the largest remaining rust-vm pressure
  - `phase-57x` proved broad deletion is not ready yet
  - another keep-pruning-only lane would lower wording debt but not reduce the live affordance surface as much
