---
Status: Landed
Decision: provisional
Date: 2026-04-03
Scope: `selfhost_build.sh` と `build.rs` の mixed ownership を speed-first で切り分け、bootstrap/product owner を明示する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-36x/README.md
  - docs/development/current/main/phases/phase-37x/37x-90-bootstrap-owner-split-ssot.md
  - docs/development/current/main/phases/phase-37x/37x-91-task-board.md
---

# Phase 37x: Bootstrap Owner Split

## Goal

- `tools/selfhost/selfhost_build.sh` の mixed owner を
  - `Stage-B engineering producer`
  - `direct MIR / core-direct runner`
  - `ny-llvmc / exe artifact`
  に分離する。
- `src/runner/build.rs` の mixed owner を
  - `product build`
  - `engineering build`
  に明示分離する。

## Fixed Reading

- この phase は `vm.rs` delete ではなく、bootstrap/product owner split を先に取る phase。
- `bootstrap_selfhost_smoke.sh` / `run_stageb_compiler_vm.sh` / `selfhost_vm_smoke.sh` は explicit engineering keep として先に凍結する。
- `child.rs` caller drain はこの phase の main work ではなく、owner split 後の follow-up として扱う。
- raw backend default / token / dispatch rewrite はまだ触らない。

## Speed Rule

- speed-first で進める。
- 途中の smoke が一時的に red でも、owner split が前進し `cargo check --bin hakorune` と `git diff --check` が通るなら進める。
- canonical proof / closeout smoke は `37xD` でまとめて戻す。
- `37xD1` の後ろには cleanup/archive sweep を置き、drained shim と legacy embedded smoke を live surface から外す。

## Non-Goals

- `src/cli/args.rs` の default rewrite
- `src/runner/dispatch.rs` の central selector rewrite
- `src/runner/modes/vm.rs` の archive/delete
- thread/runtime capability の追加
- `child.rs` の deep rewrite

## Exact Next

1. `37x-90-bootstrap-owner-split-ssot.md`
2. `37x-91-task-board.md`
3. `tools/selfhost/selfhost_build.sh`
4. `src/runner/build.rs`

- current active micro task: `phase-39x stage0 vm gate thinning`
- next queued micro task: `archive/delete of drained shims`

## Acceptance Summary

- `selfhost_build.sh` が `producer / direct-run / exe-artifact / dispatcher` で読める
- `build.rs` が `product build / engineering build` で読める
- explicit engineering keep が docs と path で固定される
- `37xD1` proof is restored on focused probes instead of reopening the whole-script Stage-B source route
- `selfhost_minimal.sh` red stays inherited from the upstream Stage-B source route and does not block cleanup/archive
- 次 phase は cleanup/archive sweep に集中できる
- `37xD1` の次は cleanup/archive sweep に入り、drained shim / stale compat wrapper / legacy embedded smoke を候補別に整理する
