---
Status: Active
Date: 2026-05-17
Scope: 再起動直後に 2-5 分で current lane に戻るための最短手順。
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
- active lane: `phase-293x mimalloc blueprint lane`
- active phase: read `active_phase` from `CURRENT_STATE.toml`
- latest card: read `latest_card_path` from `CURRENT_STATE.toml`
- current blocker token: `RANDOM-CAP-002 random capability unsupported-route preflight`
- allocator-first granularity SSOT:
  `docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md`
- pure-first MIR artifact / diagnostics SSOT:
  `docs/development/current/main/design/pure-first-mir-artifact-and-diagnostics-ssot.md`
- mimalloc blueprint SSOT:
  `docs/development/current/main/design/mimalloc-hakorune-blueprint-task-breakdown-ssot.md`
- mimalloc port purpose:
  `docs/development/current/main/design/mimalloc-hako-port-purpose-ssot.md`
- update policy:
  `docs/development/current/main/design/current-docs-update-policy-ssot.md`

## Handoff Snapshot

- latest landed card: read `latest_card_path` in `CURRENT_STATE.toml`
- current blocker token: `RANDOM-CAP-002 random capability unsupported-route preflight`
- latest known checkpoint: read `latest_card` / `latest_card_path` in
  `CURRENT_STATE.toml`; `291x-691` remains the historical warning-backlog
  inventory baseline
- no-growth checkpoint: `classifiers=0 rows=0`; no `.inc` method/box string
  classifiers are allowlisted
- worktree expectation: clean after the last commit unless an active slice is
  underway

## Immediate Next

- continue `phase-293x` after `RANDOM-CAP-001`; current blocker is
  `RANDOM-CAP-002`, the unsupported random route preflight/diagnostics row
- next row: keep random/entropy execution unsupported while making failures
  happen before backend emission
- keep LoopRange on the Stage1 route; do not source-desugar range loops
- keep allocator-provider activation, hooks, host allocator replacement, and `#[global_allocator]` inactive unless explicitly reopened

## Restart Notes

- do not paste landed-card history into restart/current mirrors
- do not run heavy perf ladders during restart unless explicitly requested
