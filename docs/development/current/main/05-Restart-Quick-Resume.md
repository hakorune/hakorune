Status: Active
Date: 2026-06-09
Scope: restart in 2-5 minutes with a thin pointer surface.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/investigations/docs-pointer-inventory-2026-06-09.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md

# Restart Quick Resume

## Quick Start

```bash
cd /home/tomoaki/git/hakorune-selfhost
git status -sb
bash tools/checks/current_state_pointer_guard.sh
```

Run heavier gates only when the next slice is ready:

```bash
tools/checks/dev_gate.sh quick
cargo check -q
```

## Current Lane

- current-state SSOT: `docs/development/current/main/CURRENT_STATE.toml`
- compiler foundation taskboard: `docs/development/current/main/workstreams/compiler-foundation-current.md`
- method anchor: read `method_anchor` in `CURRENT_STATE.toml`
- active lane: read `active_lane` in `CURRENT_STATE.toml`
- active phase: read `active_phase` in `CURRENT_STATE.toml`
- latest card: read `latest_card_path` in `CURRENT_STATE.toml`
- blocker token: read `current_blocker_token` in `CURRENT_STATE.toml`
- implementation gaps: none; read `active_lane_status` in `CURRENT_STATE.toml`

## Restart Notes

- do not paste landed chronology into restart docs
- keep allocator-provider activation, hooks, host allocator replacement, and `#[global_allocator]` out of scope
- the current lane is the `active_lane` in `CURRENT_STATE.toml`
- exact-front optimization is paused while the compiler foundation lane is
  active; resume only through the return lane named by the active taskboard
- product/app validation now uses EXE/AOT as the primary route; VM work is a
  small semantic-reference subset only
- if the current blocker token names the explicit design-stop frontier, stop
  the goal-driven execution loop here and review the frontier card before
  naming a new executable owner
