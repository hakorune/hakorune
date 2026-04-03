---
Status: Active
Decision: provisional
Date: 2026-04-03
Scope: stage0/bootstrap lane の `--backend vm` 残面を inventory し、direct route と explicit vm gate を分ける。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-39x/39x-90-stage0-vm-gate-thinning-ssot.md
  - docs/development/current/main/phases/phase-39x/39x-91-task-board.md
---

# Phase 39x: Stage0 VM Gate Thinning

## Goal

- stage0/bootstrap lane でまだ `--backend vm` に依存する surface を inventory し、direct route へ寄せられるものと explicit keep すべき gate を分ける。
- `hakorune` binary は outer facade として使い続けつつ、execution owner を `vm` から thin owner / `core_executor` / direct route へ寄せる。
- raw backend default/token はまだ触らない。

## Fixed Reading

- main work is source-route thinning, not default/token rewrite.
- `tools/selfhost/selfhost_build.sh` is the biggest mixed owner remaining in the bootstrap lane.
- `tools/selfhost/run_stageb_compiler_vm.sh` is the explicit VM gate and must be treated separately from direct-route candidates.
- `tools/selfhost/run.sh` / `tools/selfhost/bootstrap_selfhost_smoke.sh` are outer facades or explicit engineering gates.
- `src/runner/modes/common_util/selfhost/child.rs` / `src/runner/core_executor.rs` / `tools/selfhost/stage1_mainline_smoke.sh` are direct-route or thin-owner candidates.
- `phase-38x` cleanup/archive sweep is landed and the archive-later queue is frozen.
- `39xA1` landed: caller inventory is fixed for `selfhost_build.sh` / `run_stageb_compiler_vm.sh` / `run.sh`.

## Exact Next

1. `39x-90-stage0-vm-gate-thinning-ssot.md`
2. `39x-91-task-board.md`
3. `tools/selfhost/selfhost_build.sh`
4. `tools/selfhost/run_stageb_compiler_vm.sh`

- current active micro task: `39xA2 route classification`
- next queued micro task: `39xB1 bootstrap mainline direct route`
