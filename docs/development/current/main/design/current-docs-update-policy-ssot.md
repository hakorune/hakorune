---
Status: SSOT
Date: 2026-04-24
Scope: current docs update policy for restart/current-lane pointers.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - tools/checks/current_state_pointer_guard.sh
---

# Current Docs Update Policy

## Problem

Small implementation cards were forcing updates to too many human-written
mirrors:

- `CURRENT_TASK.md`
- `docs/development/current/main/05-Restart-Quick-Resume.md`
- `docs/development/current/main/10-Now.md`
- phase README
- taskboard / ledger
- `docs/development/current/main/CURRENT_STATE.toml`
- the active card

That made card work depend on manual ledger synchronization instead of one
clear current-state owner.

## Decision

`docs/development/current/main/CURRENT_STATE.toml` is the machine-readable SSOT
for the current lane, blocker, phase pointers, and latest card pointer.

Per-card mandatory docs updates are limited to:

1. `CURRENT_STATE.toml`
2. the active card
3. code/test docs only when the card changes their contract

Do not update `CURRENT_TASK.md`, `05-Restart-Quick-Resume.md`, `10-Now.md`,
phase README, taskboards, or ledgers for every landed card.

Update those mirrors only when one of these changes:

- active lane
- active blocker token
- restart order
- phase status path
- durable design/update policy
- a taskboard or ledger's own stable contract

## Current State Shape

`CURRENT_STATE.toml` should stay compact:

```toml
active_lane = "..."
active_phase = "..."
phase_status = "..."
method_anchor = "..."
taskboard = "..."
current_blocker_token = "..."

latest_card = "291x-121"
latest_card_path = "docs/.../291x-121-..."
latest_card_summary = "..."

landed_tail = [
  "last few cards only",
]
```

Full landed history belongs in phase docs and cards, not in current mirrors.

## Guard Contract

`tools/checks/current_state_pointer_guard.sh` verifies:

- required current-state scalar fields exist
- referenced repo-relative paths exist
- `latest_card_path` matches `latest_card`
- root/current/restart docs still point at `CURRENT_STATE.toml`
- active lane and blocker tokens are present in the thin mirrors
- stale pointer patterns are absent from current docs

The guard must not require every current mirror to repeat latest-card history.

## Non-Goals

- no generated-doc helper in this card
- no physical archive/move of old phase history
- no behavior or compiler changes

## Update Checklist

For a normal implementation card:

1. add/update the card
2. update `latest_card`, `latest_card_path`, `latest_card_summary`, and
   `landed_tail` in `CURRENT_STATE.toml`
3. run `bash tools/checks/current_state_pointer_guard.sh`

Only update mirrors if the card changes the active lane, blocker, restart
order, or a durable design policy.
