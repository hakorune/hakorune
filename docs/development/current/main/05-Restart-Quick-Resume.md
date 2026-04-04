---
Status: Active
Date: 2026-04-05
Scope: 再起動直後に 2〜5 分で current lane に戻るための最短手順。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/10-Now.md
---

# Restart Quick Resume

## Quick Start

```bash
cd /home/tomoaki/git/hakorune-selfhost
git status -sb
tools/checks/dev_gate.sh quick
```

## Current

- lane: `phase-93x archive-later engineering helper sweep`
- current front: `93xB1 archive move and doc repoint`
- blocker: `none`
- recent landed:
  - `phase-92x selfhost proof/compat caller rerun`
  - `phase-91x top-level .hako wrapper policy review`
  - `phase-90x current-doc/design stale surface hygiene`
  - `phase-89x next source lane selection`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/phases/phase-93x/README.md`
4. `docs/development/current/main/phases/phase-93x/93x-90-archive-later-engineering-helper-sweep-ssot.md`
5. `docs/development/current/main/phases/phase-93x/93x-91-task-board.md`

## Current Proof Bundle

```bash
cargo check --manifest-path Cargo.toml --bin hakorune
bash tools/selfhost/mainline/stage1_mainline_smoke.sh
bash tools/hakorune_emit_mir_mainline.sh lang/src/runner/launcher.hako /tmp/launcher_probe.mir.json
git diff --check
```

## Optional Checks

```bash
bash tools/smokes/v2/profiles/integration/apps/phase29x_llvm_only_daily_gate.sh
bash tools/selfhost/run_lane_a_daily.sh
./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4
```
