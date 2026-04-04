---
Status: Landed
Date: 2026-04-04
Scope: choose the next source lane after `phase-53x` landed so archive/historical cleanup does not become the de facto live default again.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-53x/README.md
  - docs/development/current/main/phases/phase-53x/53x-90-residual-vm-source-audit-ssot.md
  - docs/development/current/main/phases/phase-53x/53x-91-task-board.md
---

# Phase 54x: Next Source Lane Selection

## Goal

- inventory the candidate next source lanes after `phase-53x`
- rank the candidates by leverage, not by doc symmetry
- choose the next source lane without reopening rust-vm as a live owner

## Plain Reading

- `phase-53x` is landed and handed off.
- `rust-vm` is no longer the day-to-day owner, and `vm-hako` stays reference/conformance only.
- the lane decided the next source lane cleanly before any new work restarted.
- `kilo` remains far-future; this phase is about the nearer next source focus, not a delayed optimization wave.

## Inventory Findings

- keep-now route surfaces:
  - `src/runner/dispatch.rs`
  - `src/runner/route_orchestrator.rs`
  - `tools/selfhost/run.sh`
- archive-later wording surfaces:
  - `src/cli/args.rs` still presents `vm` / `vm-hako` as selectable backend strings in help/default text
  - `tools/selfhost/lib/selfhost_run_routes.sh` still has a `stage-a` branch that shells `--backend vm`
- proof / compat / reference keeps:
  - `src/runner/modes/vm.rs`
  - `src/runner/modes/vm_fallback.rs`
  - `src/runner/modes/vm_hako.rs`
  - `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
  - `tools/selfhost/run_stageb_compiler_vm.sh`
  - `lang/src/runner/stage1_cli/core.hako`
  - proof smoke surfaces and `vm_hako_caps/**`
- delete-ready:
  - none yet

## Retirement Corridor

- this selection lane now owns the retirement corridor up to rust-vm exit from the live source surface.
- the intended corridor is:
  - `phase-55x rust-vm route-surface retirement prep`
  - `phase-56x proof/compat keep pruning`
  - `phase-57x rust-vm delete-ready audit / removal wave`
- `vm-hako` is outside this corridor because it remains a live reference/conformance lane.
- `kilo` optimization also stays outside this corridor.

## Decision

- selected successor lane: `phase-55x rust-vm route-surface retirement prep`
- selected because route/default/help surfaces still expose rust-vm as a selectable live path
- `phase-56x proof/compat keep pruning` stays next after route-surface retirement prep
- `phase-57x rust-vm delete-ready audit / removal wave` stays after explicit keep pruning

## Handoff

- `phase-54x` is landed and handed off.
- current active lane moved to `phase-55x rust-vm route-surface retirement prep`.
- this lane leaves behind the explicit retirement corridor:
  - `55x rust-vm route-surface retirement prep`
  - `56x proof/compat keep pruning`
  - `57x rust-vm delete-ready audit / removal wave`

## Candidate Lanes

1. `phase-55x rust-vm route-surface retirement prep`
   - drain backend-route/default/help surfaces that still expose rust-vm as a selectable live code path
   - focus on `src/cli/args.rs`, `src/runner/dispatch.rs`, `src/runner/route_orchestrator.rs`, `tools/selfhost/lib/selfhost_run_routes.sh`, and `tools/selfhost/run.sh`
2. `phase-56x proof/compat keep pruning`
   - reduce keep-now proof/compat surfaces down to the smallest explicit set
   - focus on `run_stageb_compiler_vm.sh`, `stage_a_compat_bridge.rs`, `vm_fallback.rs`, `core.hako`, and proof smoke routing
3. `phase-57x rust-vm delete-ready audit / removal wave`
   - run final caller audit and delete or archive only after proof/compat replacements are explicit
   - focus on `vm.rs` and any remaining rust-vm-only route glue

## Success Conditions

- candidate next lanes are inventoried and compared
- a concrete successor is selected with rationale
- current docs point at the new active lane instead of the landed audit lane
- `cargo check --bin hakorune` and `git diff --check` stay green

## Failure Patterns

- selecting a lane for doc symmetry instead of leverage
- reopening `--backend vm` / rust-vm as a day-to-day default
- leaving current mirrors pointed at a landed lane after handoff
- treating proof-only or compat keeps as if they were the next default producer
- mixing `vm-hako` reference work into the rust-vm retirement corridor

## Big Tasks

1. inventory candidate next source lanes
   - `54xA1` successor lane inventory lock
   - `54xA2` candidate lane ranking
2. select the successor lane
   - `54xB1` successor lane decision
   - `54xB2` retirement corridor lock
3. prove and close the selection lane
   - `54xD1` proof / closeout
