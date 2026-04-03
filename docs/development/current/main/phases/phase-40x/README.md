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

## Plain Reading

- live な `--backend vm` route を残すと、新機能のたびに `rust-vm` 対応が増えやすい。
- この phase は `vm.rs` を今すぐ消す wave ではない。
- 先にやるのは、stage0/bootstrap の mainline owner を `hakorune` binary の direct/core route 側へ寄せること。
- その後で、残った vm route を `proof-only keep` / `compat keep` / `archive-later` に縮める。

## Success Conditions

- `run_stageb_compiler_vm.sh` みたいな explicit VM gate を少数の proof 用に固定する。
- `selfhost_build.sh` と `build.rs` の mixed owner を decisive split input として扱う。
- 新機能が `--backend vm`、stage1 compat、raw route に流れ込まないようにする。
- 最後に `vm.rs` を broad owner から oracle/proof keep に縮める。

## Failure Patterns

- selfhost/bootstrap の mainline が引き続き `--backend vm` を通る。
- stage1 compat や raw route に新機能を足す。
- proof 用 VM gate を日常路に戻す。

## Fixed Reading

- `phase-39x` は landed で、stage0 vm gate thinning は完了している。
- `40xA1` landed: caller inventory is fixed for the residual archive candidates.
- archive candidate selection が主 work で、default/token rewrite はまだ対象外。
- `phase-38x` の archive-later queue は frozen。
- `tools/selfhost/selfhost_build.sh` は still mixed owner で、これが最大の archive candidate input になっている。
- `tools/selfhost/run_stageb_compiler_vm.sh` は explicit VM gate であり、archive candidate selection では keep/archival boundary を明確にする対象。
- `tools/selfhost/run.sh` is the outer facade; `tools/bootstrap_selfhost_smoke.sh` / `tools/plugin_v2_smoke.sh` top-level shims are deleted and their canonical homes are `tools/selfhost/bootstrap_selfhost_smoke.sh` / `tools/plugins/plugin_v2_smoke.sh`.
- `tools/selfhost/selfhost_vm_smoke.sh` / `tools/selfhost/selfhost_stage3_accept_smoke.sh` are explicit VM proof gates and must be classified separately.
- `src/runner/modes/common_util/selfhost/child.rs` / `src/runner/core_executor.rs` are thin-owner or direct-route candidates.
- `src/runner/modes/vm.rs` remains engineering keep until archive classification proves otherwise.
- `tools/selfhost/stage1_mainline_smoke.sh` remains the direct proof home.
- `tools/stage1_smoke.sh` is already archived in `phase-38x`.

## Big Tasks

1. fix a small proof-only VM gate set and mark it `do-not-grow`
2. treat `selfhost_build.sh` and `build.rs` as the decisive mixed-owner split inputs
3. stop new capability work from landing on `--backend vm`, stage1 compat, or raw routes
4. drain top-level vm-facing shims
5. archive drained shims and only then shrink `vm.rs`

## Exact Next

1. `40x-90-stage0-vm-archive-candidate-selection-ssot.md`
2. `40x-91-task-board.md`
3. `tools/selfhost/selfhost_build.sh`
4. `src/runner/build.rs`
5. `tools/selfhost/run_stageb_compiler_vm.sh`
6. `tools/selfhost/bootstrap_selfhost_smoke.sh`
7. `tools/plugins/plugin_v2_smoke.sh`

- current active micro task: `40xD1 proof / closeout`
- next queued micro task: `next source lane selection`
