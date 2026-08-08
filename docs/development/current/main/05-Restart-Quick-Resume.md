Status: Active
Date: 2026-07-28
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
- MirBuilder north star: read `mirbuilder_north_star` in `CURRENT_STATE.toml`
- workstream card: read `latest_workstream_card` in `CURRENT_STATE.toml`
- method anchor: read `method_anchor` in `CURRENT_STATE.toml`
- active lane: read `active_lane` in `CURRENT_STATE.toml`
- work mode: read `work_mode` in `CURRENT_STATE.toml`; do not infer it from the blocker text
- active phase: read `active_phase` in `CURRENT_STATE.toml`
- latest card: read `latest_card_path` in `CURRENT_STATE.toml`
- blocker token: read `current_blocker_token` in `CURRENT_STATE.toml`
- current scope and parked resume: read `active_lane_status` and the workstream

## Restart Notes

- handoff frontier: read `current_blocker_token` in `CURRENT_STATE.toml`
- when `work_mode = "design_stop"`, stop the goal-driven execution loop here and review the frontier card before selecting more work
- read `latest_card_path` before editing
- continue only the exact `current_blocker_token` and `latest_card_path` from
  `CURRENT_STATE.toml`; this mirror does not select or rename executable rows
- read `method_anchor` for the in-place production replacement law
- read `mirbuilder_north_star` before selecting a replacement cell; cell and
  LOC counters are migration metrics, not the architecture goal
- `I0` requires a real production caller switch; a disconnected candidate is
  S0/PROBE0, not I0
- the same cell deletes the selected old branch in I0/R0 before unrelated work
- Stage-B, Ownership, Language v1, and `.hako` selfhost lanes are parked
- do not paste landed chronology into restart docs
- keep allocator-provider activation, hooks, host allocator replacement, and `#[global_allocator]` out of scope
- the current lane is the `active_lane` in `CURRENT_STATE.toml`
- all other rows remain parked unless `CURRENT_STATE.toml` explicitly selects
  them
- product/app validation now uses EXE/AOT as the primary route; VM work is a
  small semantic-reference subset only
