---
Status: Active
Date: 2026-05-18
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
- current blocker token: `MIMAP-143A segment allocation modeled local-free reuse ledger release-applied recycle closeout guard`
- allocator-first granularity SSOT:
  `docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md`
- pure-first MIR artifact / diagnostics SSOT:
  `docs/development/current/main/design/pure-first-mir-artifact-and-diagnostics-ssot.md`
- mimalloc row validation cadence SSOT:
  `docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md`
- mimalloc blueprint SSOT:
  `docs/development/current/main/design/mimalloc-hakorune-blueprint-task-breakdown-ssot.md`
- mimalloc port purpose:
  `docs/development/current/main/design/mimalloc-hako-port-purpose-ssot.md`
- update policy:
  `docs/development/current/main/design/current-docs-update-policy-ssot.md`

## Handoff Snapshot

- latest landed card: read `latest_card_path` in `CURRENT_STATE.toml`
- current blocker token: `MIMAP-143A segment allocation modeled local-free reuse ledger release-applied recycle closeout guard`
- latest known checkpoint: read `latest_card` / `latest_card_path` in
  `CURRENT_STATE.toml`; `291x-691` remains the historical warning-backlog
  inventory baseline
- no-growth checkpoint: `classifiers=0 rows=0`; no `.inc` method/box string
  classifiers are allowlisted
- worktree expectation: clean after the last commit unless an active slice is
  underway

## Immediate Next

- continue `phase-293x` with `MIMAP-143A`; current blocker is the closeout guard
  row after release-applied local-free reuse ledger token recycle
- latest allocator row: `MIMAP-142A` landed and selected MIMAP-143A
- keep LoopRange on the Stage1 route; do not source-desugar range loops
- keep allocator-provider activation, hooks, host allocator replacement, and `#[global_allocator]` inactive unless explicitly reopened

## Restart Notes

- do not paste landed-card history into restart/current mirrors
- do not run heavy perf ladders during restart unless explicitly requested
