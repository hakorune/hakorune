---
Status: Active
Date: 2026-04-25
Scope: 再起動直後に 2〜5 分で current lane に戻るための最短手順。
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
---

# Restart Quick Resume

## Quick Start

```bash
cd /home/tomoaki/git/hakorune-selfhost
git status -sb
bash tools/checks/current_state_pointer_guard.sh
```

Heavy gates are not first-step restart work. Run them only when the next code
slice is ready:

```bash
tools/checks/dev_gate.sh quick
cargo check -q
```

## Current Lane

- current-state SSOT: `docs/development/current/main/CURRENT_STATE.toml`
- active lane: `phase-291x CoreBox surface contract cleanup`
- active phase: read `active_phase` from `CURRENT_STATE.toml`
- latest card: read `latest_card_path` from `CURRENT_STATE.toml`
- current blocker token: `phase-291x remaining inc mirror cleanup selection pending`
- update policy:
  `docs/development/current/main/design/current-docs-update-policy-ssot.md`

## Handoff Snapshot

- latest card: `291x-246`
- last landed work: push route mirror prune review, then len route mirror prune
  review
- worktree: current local diff now includes the 291x-240..246 cleanup/review
  batch plus this restart handoff refresh
- resume point: the next remaining `.inc` mirror cleanup from the phase-291x
  inventory after the len/push review batch

## Immediate Next

- select the next remaining `.inc` mirror cleanup from the phase-291x inventory
- keep legacy `push` mirror rows until metadata-absent mutating boundary coverage exists
- keep generic `set` emit-kind and MapBox/RuntimeData storage-route fallback
  rows until their own fallback contracts are replaced
- keep docs mirrors thin; update `CURRENT_STATE.toml` and the active card first
- keep Stage-B adapter thinning separate from CoreMethodContract migration
- keep phase-137x observe-only unless app work reopens a real blocker

## Restart Notes

- worktree should be clean after the last commit
- do not run heavy perf ladders during restart unless explicitly requested
