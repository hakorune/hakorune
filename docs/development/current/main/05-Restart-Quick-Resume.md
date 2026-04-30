---
Status: Active
Date: 2026-04-30
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
- active lane: `phase-29ci Program(JSON v0) public compat retirement`
- active phase: read `active_phase` from `CURRENT_STATE.toml`
- latest card: read `latest_card_path` from `CURRENT_STATE.toml`
- current blocker token: `phase-29ci raw compat caller migration pending`
- update policy:
  `docs/development/current/main/design/current-docs-update-policy-ssot.md`

## Handoff Snapshot

- latest landed card: read `latest_card_path` in `CURRENT_STATE.toml`
- current blocker token: `phase-29ci raw compat caller migration pending`
- latest known checkpoint: read `latest_card` / `latest_card_path` in
  `CURRENT_STATE.toml`; `291x-691` remains the historical warning-backlog
  inventory baseline
- no-growth checkpoint: `classifiers=0 rows=0`; no `.inc` method/box string
  classifiers are allowlisted
- worktree expectation: clean after the last commit unless an active slice is
  underway

## Immediate Next

- continue `phase-29ci` raw compat caller migration after the P8 helper-local
  legacy delegate retirement slice
- `--hako-emit-mir-json` stays as the Stage-1 MIR launcher for
  `stage1-env-mir-source`; raw Program(JSON) compat callers are inventoried and
  the next work is migration by caller family
- do not reopen broad `plan/facts` or `lower::planner_compat` ownership work
  without focused BoxShape lanes and SSOT cards
- normalized-shadow / normalization cleanup burst is closed; larger findings
  move to a new lane
- use `docs/development/current/main/phases/phase-291x/291x-488-current-task-order-baseline-refresh-card.md`
  for the current task-order baseline
- use `docs/development/current/main/phases/phase-291x/291x-smoke-index.md`
  for smoke selection
- keep docs mirrors thin; update `CURRENT_STATE.toml` and the active card first
- keep Stage-B adapter thinning separate from CoreMethodContract migration
- keep phase-137x observe-only unless app work reopens a real blocker

## Restart Notes

- do not paste landed-card history into restart/current mirrors
- do not run heavy perf ladders during restart unless explicitly requested
