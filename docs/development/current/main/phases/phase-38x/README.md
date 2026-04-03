---
Status: Active
Decision: provisional
Date: 2026-04-03
Scope: drained shim / legacy embedded smoke / stale compat wrapper を live surface から外し、archive-later と delete-ready を分離する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-38x/38x-90-cleanup-archive-sweep-ssot.md
  - docs/development/current/main/phases/phase-38x/38x-91-task-board.md
---

# Phase 38x: Cleanup / Archive Sweep

## Goal

- drained shim を delete-ready と archive-later に分ける。
- legacy embedded smoke を top-level `tools/` surface から外す。
- stale compat wrapper の delete order を固定する。

## Fixed Reading

- main work は source rewrite ではなく surface cleanup。
- `bootstrap_selfhost_smoke.sh` / `plugin_v2_smoke.sh` / `hako_check_deadcode_smoke.sh` は current/historical docs drain が残るので archive-later。
- `hako_check_deadblocks_smoke.sh` は delete-ready から削除済み。
- `stage1_smoke.sh` は legacy embedded bridge smoke として archive 済み。

## Exact Next

1. `38x-90-cleanup-archive-sweep-ssot.md`
2. `38x-91-task-board.md`
3. `tools/archive/legacy-selfhost/stage1_embedded_smoke.sh`
4. `tools/bootstrap_selfhost_smoke.sh`
5. `tools/plugin_v2_smoke.sh`
6. `tools/hako_check_deadcode_smoke.sh`

- current active micro task: `38xD1 closeout`
- next queued micro task: `next source lane selection`
