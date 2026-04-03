---
Status: Landed
Decision: provisional
Date: 2026-04-03
Scope: `stage0` engineering lane に残る shell residue を thin にし、thread 設計が `vm`/selfhost/raw compat branch を再肥大化させない owner boundary を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-33x/README.md
  - docs/development/current/main/phases/phase-34x/34x-90-stage0-shell-residue-split-ssot.md
  - docs/development/current/main/phases/phase-34x/34x-91-task-board.md
---

# Phase 34x: Stage0 Shell Residue Split

## Goal

- `vm-rust` keep を維持したまま、`stage0` engineering lane に残る shell residue を thin にする。
- first target は:
  - `src/runner/modes/common_util/selfhost/child.rs`
  - `lang/src/runner/stage1_cli/core.hako`
  - `src/runner/core_executor.rs`
- thread 設計はこの phase の後ろに置き、shell/process/raw compat branch をこれ以上 widen しない。

## Fixed Reading

- `phase-33x shared helper family recut` は landed precursor として読む。
- この phase は `vm.rs` delete ではなく、`stage0` engineering residue を narrow owner に寄せる phase。
- `stage0 hakorune binary` は outer engineering facade として keep。
- ただし internal source は shell-out を増やさず、`core_executor` 側に寄せる。
- `args.rs` / `dispatch.rs` / raw backend token/default はまだ触らない。

## Non-Goals

- `src/runner/modes/vm.rs` の archive/delete
- `src/cli/args.rs` の default rewrite
- `src/runner/dispatch.rs` の central selector rewrite
- thread/runtime capability 設計の追加

## Exact Next

1. `34x-90-stage0-shell-residue-split-ssot.md`
2. `34x-91-task-board.md`
3. `src/runner/modes/common_util/selfhost/child.rs`
4. `lang/src/runner/stage1_cli/core.hako`
5. `src/runner/core_executor.rs`

## Acceptance Summary

- `child.rs` shell residue is split into thinner spawn/capture ownership
- `stage1_cli/core.hako` raw compat branch stays narrow and does not widen for new runtime work
- `core_executor` becomes the explicit in-proc owner for already-materialized `MIR(JSON)` execution
- direct `MIR(JSON)` proof is pinned by unit tests on `execute_mir_json_text(...)`
- thread design can be added later without re-expanding `vm`/selfhost/raw compat surfaces

## Current State

- phase-34x tasks are landed through `34xD1`
- successor lane is `phase-35x stage-a compat route thinning`
