---
Status: Landed
Decision: provisional
Date: 2026-04-03
Scope: shared helper family の path-truth を進め、low-blast helper だけ family home に寄せ、`emit_mir` thin wrapper は route-preset thin shim として truthify する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-32x/README.md
  - docs/development/current/main/phases/phase-33x/33x-90-shared-helper-family-recut-ssot.md
  - docs/development/current/main/phases/phase-33x/33x-91-task-board.md
---

# Phase 33x: Shared Helper Family Recut

## Goal

- shared helper family を `keep / family-home / shim-only` に切り分け、top-level helper surface をこれ以上太らせない。
- first cut は `hako_check` family と `emit_mir` thin wrapper truth。
- `hako_check.sh` と `hakorune_emit_mir.sh` は live caller が広いので、先に thin wrapper / smoke helper から family home に寄せる。

## Fixed Reading

- `phase-32x product / engineering split` は landed precursor として読む。
- この phase は helper implementation rewrite ではなく path-truth / caller-drain / shim-only 化が主題。
- raw backend default / token / dispatch rewrite はまだ触らない。
- `vm-rust` keep / `vm-hako` reference / `wasm` experimental の読みはそのまま維持する。

## Non-Goals

- `src/cli/args.rs` / `src/runner/dispatch.rs` の token/default truthification
- `src/runner/modes/vm.rs` の archive/delete
- `tools/hako_check.sh` 本体の機能変更
- `tools/hakorune_emit_mir.sh` 本体の route semantics 変更

## Exact Next

1. `33x-90-shared-helper-family-recut-ssot.md`
2. `33x-91-task-board.md`
3. `tools/hako_check/deadblocks_smoke.sh`
4. `tools/hakorune_emit_mir_mainline.sh`
5. `tools/hakorune_emit_mir_compat.sh`
6. `tools/smokes/v2/lib/emit_mir_route.sh`

## Canonical Child Docs

- split rules / helper disposition:
  - `33x-90-shared-helper-family-recut-ssot.md`
- concrete queue / evidence commands:
  - `33x-91-task-board.md`

## Acceptance Summary

- `tools/hako_check_deadblocks_smoke.sh` is rehomed under `tools/hako_check/**` and the top-level path becomes shim-only
- `emit_mir` thin wrappers stay as top-level route-preset compatibility wrappers; operational routing truth stays in `tools/smokes/v2/lib/emit_mir_route.sh`
- `tools/hako_check.sh` and `tools/hakorune_emit_mir.sh` remain explicit top-level keep until caller drain is exact
- current/public docs point at truthful family homes where blast is low

## Current State

- helper-family micro tasks are landed through `33xD1`
- current front is `phase-33x closeout review`
- successor lane is `phase-34x stage0 shell residue split`
