Status: Active
Date: 2026-07-14
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
- D′ is accepted: exact coverage/topology/ports/cleanup stays pre-Builder and
  one function-wide Binding SSA becomes canonical local value/PHI authority
- B0-L4-S2′ is closed with production consumers and Binding SSA/Loop
  activation zero; existing A+ If production remains unchanged
- the active blocker is SSA-P0 exhaustive seam inventory; classify current
  BindingRef value, CFG/predecessor, PHI lifecycle, RC, and publication paths
  without changing production ownership or accepted grammar
- old A+A2′ carrier S3/I1/I2 and effect-bearing V2 extraction are superseded;
  do not revive them from historical cards
- keep parser/source-carrier and unrelated parked stashes disconnected
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
