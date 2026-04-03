---
Status: Landed
Decision: provisional
Date: 2026-04-03
Scope: `selfhost.rs` の source materialization と `stage1_cli` raw subcommand bridge の owner split を進め、stage0/stage1 compat residue をさらに thin にする。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-35x/README.md
  - docs/development/current/main/phases/phase-36x/36x-90-selfhost-source-stage1-bridge-split-ssot.md
  - docs/development/current/main/phases/phase-36x/36x-91-task-board.md
---

# Phase 36x: Selfhost Source / Stage1 Bridge Split

## Goal

- `selfhost.rs` から source read / using merge / tmp staging を thin owner へ切り出す。
- `stage1_cli/core.hako` の raw subcommand residue は narrow adapter のまま保ち、source materialization と request parse の owner をさらに分ける。

## Fixed Reading

- `phase-35x stage-a compat route thinning` は landed precursor として読む。
- この phase は `vm.rs` delete ではなく `selfhost.rs` と `stage1_cli` の mixed owner を減らす phase。
- `child.rs` shell residue, `core_executor` direct MIR owner, `stage_a_route.rs` thin owner は前 phase の fixed rule として尊重する。
- raw backend default / token / dispatch rewrite はまだ触らない。

## Non-Goals

- `src/cli/args.rs` の default rewrite
- `src/runner/dispatch.rs` の central selector rewrite
- `src/runner/modes/vm.rs` の archive/delete
- thread/runtime capability の追加

## Exact Next

1. `36x-90-selfhost-source-stage1-bridge-split-ssot.md`
2. `36x-91-task-board.md`
3. `src/runner/selfhost.rs`
4. `src/runner/modes/common_util/selfhost/source_prepare.rs`
5. `lang/src/runner/stage1_cli/core.hako`

## Acceptance Summary

- `selfhost.rs` no longer owns source read / using merge / tmp staging details
- Stage-A route still receives raw source only; direct MIR owner stays below stage0
- `stage1_cli` raw run / emit-mir subcommands remain thin adapters and do not widen runtime capability
- next runtime/thread work does not have to touch Stage1 raw compat ownership

## Current State

- `36xA1` is landed: `source_prepare.rs` now owns source extension gate / source read / using merge / preexpand / tmp staging
- `36xA2` is landed: `selfhost.rs` is now explicitly route ordering / macro gate / terminal accept owner
- `36xB1` is landed: `raw_subcommand_emit_mir.hako` now owns raw `emit mir-json` request parse / materialize / emit / stdout
- `36xB2` is landed: `raw_subcommand_run.hako` now owns raw `run` request parse / script-args env / Program(JSON) materialization
- `36xC1` is landed: proof/closeout fixes the split as evidence instead of reopening compat ownership
- current front is `phase-38x cleanup/archive sweep`
- predecessor lane is `phase-35x stage-a compat route thinning`
