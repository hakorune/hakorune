---
Status: Active
Decision: provisional
Date: 2026-04-03
Scope: stage0/bootstrap lane の remaining direct/core route ownership を harden し、proof-only VM gates と compat keep を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-40x/README.md
  - docs/development/current/main/phases/phase-40x/40x-90-stage0-vm-archive-candidate-selection-ssot.md
  - docs/development/current/main/phases/phase-40x/40x-91-task-board.md
---

# Phase 41x: Stage0 Direct/Core Route Hardening

## Goal

- move the remaining stage0/bootstrap mainline ownership toward `hakorune` binary direct/core routes
- keep proof-only VM gates frozen and non-growing
- keep `vm.rs` and raw compat surfaces as proof/compat keep, not feature surfaces
- treat `selfhost_build.sh` and `run.sh` as direct/core facades, not mixed feature owners

## Plain Reading

- 40x removed the drained top-level shims and froze the obvious VM gates.
- 41x hardens the remaining route so new capability work does not drift back into `rust-vm`.
- This is not a `vm.rs` delete wave.
- It is a route-hardening wave: direct/core first, proof-only VM keep second, archive later only after caller drain proves the route is stable.

## Success Conditions

- `selfhost_build.sh` and `run.sh` are thin route facades
- proof-only VM gates stay frozen
- direct/core mainline is the default path for new work
- `vm.rs` stops receiving new capability work
- raw backend default/token still remain deferred

## Failure Patterns

- vm-gated bootstrap routes become day-to-day mainline again
- stage1 compat or raw routes absorb new capability work
- proof-only VM gates widen into convenience routes
- direct/core routes stay behind facade-only wiring

## Fixed Reading

- phase-40x is landed; drained top-level vm-facing shims are gone
- `tools/selfhost/selfhost_build.sh` is already split through helper files; 41x is about hardening the route, not reintroducing a broad split
- `src/runner/build.rs` is already split into product/engineering helpers; do not reopen it unless a new caller demands it
- `tools/selfhost/run_stageb_compiler_vm.sh`, `tools/selfhost/selfhost_vm_smoke.sh`, and `tools/selfhost/selfhost_stage3_accept_smoke.sh` remain proof-only keeps
- `41xA2` landed: proof-only VM gate set is frozen and non-growing
- `41xB1` landed: `selfhost_build.sh` direct/core route hardening is fixed as a route facade
- `41xB2` landed: `run.sh` facade trim is fixed as a route facade
- `src/runner/modes/vm.rs` remains engineering keep until route hardening proves it can shrink
- `lang/src/runner/stage1_cli/core.hako` remains compat keep
- `src/runner/core_executor.rs` remains the direct owner
- `tools/selfhost/stage1_mainline_smoke.sh` remains the direct proof home

## Big Tasks

1. inventory the remaining direct/core route facades and caller families
2. keep proof-only VM gates frozen and non-growing
3. harden `selfhost_build.sh` and `run.sh` so the mainline path is direct/core-first
4. shrink `vm.rs` toward proof/oracle keep only after caller drain
5. close out with proof and handoff

## Exact Next

1. `41x-90-stage0-direct-core-route-hardening-ssot.md`
2. `41x-91-task-board.md`
3. `tools/selfhost/selfhost_build.sh`
4. `tools/selfhost/run.sh`
5. `src/runner/modes/vm.rs`
6. `src/runner/core_executor.rs`

- current active micro task: `41xC1 vm.rs proof/oracle shrink`
- next micro task: `41xD1 proof / closeout`
