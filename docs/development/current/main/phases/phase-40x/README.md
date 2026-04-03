---
Status: Active
Decision: provisional
Date: 2026-04-03
Scope: stage0/bootstrap lane の remaining vm-rust / vm-gated surfaces を archive candidate / keep / rehome / delete-ready に分類し、vm-rust archive への移行条件を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-40x/40x-90-stage0-vm-archive-candidate-selection-ssot.md
  - docs/development/current/main/phases/phase-40x/40x-91-task-board.md
---

# Phase 40x: Stage0 VM Archive Candidate Selection

## Goal

- stage0/bootstrap lane でまだ残る vm-rust / vm-gated surface を archive candidate / keep / rehome / delete-ready に分ける。
- `hakorune` binary は outer facade のまま使い続けつつ、archive へ寄せられる surface を先に切り分ける。
- raw backend default/token はまだ触らない。

## Fixed Reading

- `phase-39x` は landed で、stage0 vm gate thinning は完了している。
- archive candidate selection が主 work で、default/token rewrite はまだ対象外。
- `phase-38x` の archive-later queue は frozen。
- `tools/selfhost/selfhost_build.sh` は still mixed owner で、これが最大の archive candidate input になっている。
- `tools/selfhost/run_stageb_compiler_vm.sh` は explicit VM gate であり、archive candidate selection では keep/archival boundary を明確にする対象。
- `tools/selfhost/run.sh` / `tools/bootstrap_selfhost_smoke.sh` / `tools/plugin_v2_smoke.sh` は outer facades or archive-later shims。
- `tools/selfhost/selfhost_vm_smoke.sh` / `tools/selfhost/selfhost_stage3_accept_smoke.sh` are explicit VM proof gates and must be classified separately.
- `src/runner/modes/common_util/selfhost/child.rs` / `src/runner/core_executor.rs` are thin-owner or direct-route candidates.
- `src/runner/modes/vm.rs` remains engineering keep until archive classification proves otherwise.
- `tools/selfhost/stage1_mainline_smoke.sh` remains the direct proof home.
- `tools/stage1_smoke.sh` is already archived in `phase-38x`.

## Exact Next

1. `40x-90-stage0-vm-archive-candidate-selection-ssot.md`
2. `40x-91-task-board.md`
3. `tools/selfhost/selfhost_build.sh`
4. `tools/selfhost/run_stageb_compiler_vm.sh`
5. `tools/bootstrap_selfhost_smoke.sh`
6. `tools/plugin_v2_smoke.sh`

- current active micro task: `40xA1 archive candidate inventory`
- next queued micro task: `40xA2 keep/archive classification`
