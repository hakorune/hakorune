Status: SSOT mirror
Date: 2026-07-14
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
- priority: read the exact row from `current_blocker_token`. The active macro
  order is CAT0 `L0 -> S0 -> C0a -> C0b -> G0`, then MP0
  `S0 -> R0 -> P0 -> TX0`, then the bounded P0c-B1 sibling-call slice.
  Production Ownership SSA, Loop production, legacy fallback, default source,
  Lambda/capture, ProgramV0 authority, and durable RegionId materialization
  remain inactive
- parked language work: LANGV1 conformance closeout remains parked; no
  language behavior is changed by the reprioritization

## Rule

This file is only a mirror. Implementation details, acceptance, landed history,
and parked tasks belong in the active card, the workstream SSOT, phase cards,
or git history.
