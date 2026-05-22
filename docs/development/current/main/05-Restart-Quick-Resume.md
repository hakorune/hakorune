---
Status: Active
Date: 2026-05-22
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
- active lane: read `active_lane` from `CURRENT_STATE.toml`
- active phase: read `active_phase` from `CURRENT_STATE.toml`
- latest card: read `latest_card_path` from `CURRENT_STATE.toml`
- current blocker token: read `current_blocker_token` from `CURRENT_STATE.toml`
- method anchor / design SSOT: read `method_anchor` from `CURRENT_STATE.toml`
- update policy:
  `docs/development/current/main/design/current-docs-update-policy-ssot.md`

## Handoff Snapshot

- latest landed card: read `latest_card_path` in `CURRENT_STATE.toml`
- current blocker token: read `current_blocker_token` from
  `CURRENT_STATE.toml`
- worktree expectation: clean after the last commit unless an active slice is
  underway

## Immediate Next

- continue the active phase from `current_blocker_token`, `phase_status`, and
  `latest_card_path` in `CURRENT_STATE.toml`
- keep allocator-provider activation, hooks, host allocator replacement, and `#[global_allocator]` out of scope

## Restart Notes

- do not paste landed-card history into restart/current mirrors
- do not run heavy perf ladders during restart unless explicitly requested
