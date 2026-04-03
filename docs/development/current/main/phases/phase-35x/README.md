---
Status: Active
Decision: provisional
Date: 2026-04-03
Scope: `selfhost.rs` と `stage_a_compat_bridge.rs` の Stage-A payload/compat fallback ownership を thin にし、stage0 engineering lane の route sequencing を slimmer にする。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-34x/README.md
  - docs/development/current/main/phases/phase-35x/35x-90-stage-a-compat-route-thinning-ssot.md
  - docs/development/current/main/phases/phase-35x/35x-91-task-board.md
---

# Phase 35x: Stage-A Compat Route Thinning

## Goal

- `selfhost.rs` を route sequencing / orchestration 寄りにし、captured payload resolution と Program(JSON v0) compat fallback ownership を `stage_a_compat_bridge.rs` 側へ寄せる。
- first target は:
  - `src/runner/selfhost.rs`
  - `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
  - `src/runner/modes/common_util/selfhost/stage_a_policy.rs`

## Fixed Reading

- `phase-34x stage0 shell residue split` は landed precursor として読む。
- この phase は `vm.rs` delete ではなく Stage-A compat routing の mixed ownership を thin にする phase。
- `child.rs` public facade と `stage1_cli/core.hako` raw compat no-widen は前 phase の fixed rule として尊重する。
- raw backend default / token / dispatch rewrite はまだ触らない。

## Non-Goals

- `src/cli/args.rs` の default rewrite
- `src/runner/dispatch.rs` の central selector rewrite
- `src/runner/modes/vm.rs` の archive/delete
- thread/runtime capability の追加

## Exact Next

1. `35x-90-stage-a-compat-route-thinning-ssot.md`
2. `35x-91-task-board.md`
3. `src/runner/selfhost.rs`
4. `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
5. `src/runner/modes/common_util/selfhost/stage_a_policy.rs`

## Acceptance Summary

- captured Stage-A payload resolution is owned by `stage_a_compat_bridge.rs`
- `selfhost.rs` keeps Stage-A spawn/orchestration and stops owning payload-family branching
- direct MIR lane stays `LANE_DIRECT`; Program(JSON v0) fallback remains explicit compat only
- next thread/runtime work does not have to widen Stage-A compat routing

## Current State

- `35xA1` is landed: captured payload resolution moved under `stage_a_compat_bridge.rs`
- current front is `35xA2 selfhost orchestration-only lock`
- predecessor lane is `phase-34x stage0 shell residue split`
