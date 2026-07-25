Status: Active
Date: 2026-07-25
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
- function-exit meaning is owned by
  `docs/reference/language/function-exit-and-entry-result.md`
- ordinary function/Main uses ExplicitReturnOnly; Script result, physical
  entry transport, and process termination remain distinct later owners
- the former App any-statement-tail S0 is superseded before implementation and
  may be read only as historical compatibility evidence
- continue only the exact `current_blocker_token` and `latest_card_path` from
  `CURRENT_STATE.toml`; this mirror does not select or rename executable rows
- keep parser/source-carrier and unrelated parked stashes disconnected
- Script classification through the complete S3 Raw VM-reference activation
  row are closed. R0A/R0B migrated and retired the caller-zero old-Raw
  authority; G0, PROFILE0, CANARY0, and CANARY-PARITY0/G0 are closed by
  scoped zero/cargo/guard plus real-binary evidence. The accepted cutover
  decision promotes only this explicit route to a supported opt-in reference
  lane. RAW-VM-REFERENCE-SUPPORT0-S0 is next; normal/default cutover,
  general VM/LLVM, JSON, executor, and CUT0 remain parked.

- do not paste landed chronology into restart docs
- keep allocator-provider activation, hooks, host allocator replacement, and `#[global_allocator]` out of scope
- the current lane is the `active_lane` in `CURRENT_STATE.toml`
- other language-v1 and ownership rows remain parked unless
  `CURRENT_STATE.toml` explicitly selects them
- product/app validation now uses EXE/AOT as the primary route; VM work is a
  small semantic-reference subset only
- if a future current blocker token names an explicit design-stop frontier,
  pause the goal-driven execution loop here and review the frontier card before
  naming a new executable owner
