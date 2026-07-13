Status: Active
Date: 2026-07-13
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
- workstream card: read `latest_workstream_card` in `CURRENT_STATE.toml`
- method anchor: read `method_anchor` in `CURRENT_STATE.toml`
- active lane: read `active_lane` in `CURRENT_STATE.toml`
- active phase: read `active_phase` in `CURRENT_STATE.toml`
- latest card: read `latest_card_path` in `CURRENT_STATE.toml`
- blocker token: read `current_blocker_token` in `CURRENT_STATE.toml`
- current scope and parked resume: read `active_lane_status` and the workstream

## Restart Notes

- handoff frontier: read `current_blocker_token` in `CURRENT_STATE.toml`
- read `latest_card_path` before editing
- owner-scoped arena through atomic SA3-B is closed (B0-C was skipped); the
  first explicit resolved function route is green and the active frontier is
  B0-L3a straight-line BlockExpr ScopeId/RegionId consumption named by
  `current_blocker_token` and `latest_card_path`
- keep RegionFlow R1, parser/source-carrier P1, and all parked source-projection
  stashes disconnected until the semantic-arena prerequisite series closes
- LANGV1 conformance closeout and Failure/Outcome global migration remain
  parked and must not be inferred complete

- do not paste landed chronology into restart docs
- keep allocator-provider activation, hooks, host allocator replacement, and `#[global_allocator]` out of scope
- the current lane is the `active_lane` in `CURRENT_STATE.toml`
- language-v1 convergence has priority while its workstream is active;
  selfhost/MirBuilder resumes only through the workstream closeout
- product/app validation now uses EXE/AOT as the primary route; VM work is a
  small semantic-reference subset only
- if a future current blocker token names an explicit design-stop frontier,
  pause the goal-driven execution loop here and review the frontier card before
  naming a new executable owner
