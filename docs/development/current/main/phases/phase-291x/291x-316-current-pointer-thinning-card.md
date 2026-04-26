---
Status: Landed
Date: 2026-04-26
Scope: current pointer thinning for phase-291x cleanup lane
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/phases/phase-291x/README.md
  - tools/checks/current_state_pointer_guard.sh
  - tools/checks/current_state_stale_pointer_patterns.txt
---

# 291x-316: Current Pointer Thinning

## Goal

Keep current/restart docs as pointers, not ledgers.

This is docs/tooling BoxShape cleanup. It does not change compiler behavior.

## Change

- Moved stale current pointer guard fixtures out of `CURRENT_STATE.toml` into:

```text
tools/checks/current_state_stale_pointer_patterns.txt
```

- Updated `current_state_pointer_guard.sh` to read that guard-owned fixture.
- Shortened `CURRENT_STATE.toml`:
  - compact `active_lane_status`
  - `landed_tail` limited to the recent tail
  - no embedded stale-pattern ledger
- Shortened current mirrors:
  - `CURRENT_TASK.md`
  - `10-Now.md`
  - phase-291x `README.md`
- Removed the long phase-card `Related` ledger from the phase README front
  matter. The numbered `291x-*` cards remain the ledger.

## Preserved Behavior

- `current_state_pointer_guard.sh` still checks the same stale patterns.
- Current mirrors still contain the active lane and blocker token.
- `latest_card_path` remains the card pointer SSOT.

## Non-Goals

- No card archive/move.
- No compiler behavior change.
- No smoke selection change.
- No generated docs helper.

## Validation

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
