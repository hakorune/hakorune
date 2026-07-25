Status: SSOT mirror
Date: 2026-07-25
Scope: one-screen current dashboard. Do not store landed history here.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md

# Now

## Current

- current-state SSOT: `docs/development/current/main/CURRENT_STATE.toml`
- active lane: read `active_lane` in `CURRENT_STATE.toml`
- active phase: read `active_phase` in `CURRENT_STATE.toml`
- workstream card: read `latest_workstream_card` in `CURRENT_STATE.toml`
- latest card: read `latest_card_path` in `CURRENT_STATE.toml`
- blocker token: read `current_blocker_token` in `CURRENT_STATE.toml`
- current semantic authority:
  `docs/reference/language/function-exit-and-entry-result.md`
- priority: follow the exact active task. Do not reopen the superseded App
  any-statement-tail S0 as canonical work.
- next frontier: `ENTRY-RESULT-PROJECTION0-S3-PARITY0`
  (S3 EXECUTION0 now executes exact Main/main/0 with the retained decode plan;
  S3 OWNER0 shares one typed compile kernel; run actual compile/VM parity and
  caller census before any broader activation)
- parked: normal-entry cutover, JSON, executor, old-chain retirement, and CUT0

## Rule

This file is only a mirror. Implementation details, acceptance, landed history,
and parked tasks belong in the active card, the workstream SSOT, phase cards,
or git history.
