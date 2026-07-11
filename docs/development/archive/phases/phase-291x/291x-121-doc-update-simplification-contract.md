---
Status: Landed
Date: 2026-04-24
Scope: simplify current docs update flow before continuing phase-291x work.
Related:
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - tools/checks/current_state_pointer_guard.sh
---

# 291x-121 Docs Update Simplification Contract

## Decision

Adopt `docs/development/current/main/design/current-docs-update-policy-ssot.md`
as the current docs update policy.

This card is BoxShape-only. It changes docs ownership and guard behavior, not
compiler behavior.

## Implementation

- compact `CURRENT_STATE.toml` so it stores current pointers plus the latest
  card pointer, not a long landed-history ledger
- thin `CURRENT_TASK.md`, `05-Restart-Quick-Resume.md`, and `10-Now.md` back to
  restart/current summaries
- change `current_state_pointer_guard.sh` to validate current-state paths and
  stale pointers instead of requiring latest-card history in every mirror
- update docs layout / check-script index wording to match the new guard role

## Required Update Set After This Card

Normal per-card work updates only:

1. the active card
2. `CURRENT_STATE.toml` latest-card fields
3. relevant code/test docs if the card changes their contract

Current mirrors are updated only for lane, blocker, restart-order, phase-status,
or durable policy changes.

## Non-Goals

- no generated docs helper
- no archive/move of historical phase docs
- no feature rows or router behavior changes

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
