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
- language-v1 workstream:
  `docs/development/current/main/workstreams/language-v1-convergence-current.md`
- priority: D′ is taskized through the final canonical-source form. The active
  code-facing blocker is B0-L4-S2′ generic located coverage with production
  activation zero. Existing If remains runtime-green on its old owner until
  the later whole-function SSA cutover; Loop production, carrier rows, legacy
  fallback, default source, Lambda/capture, ProgramV0 authority, and durable
  RegionId materialization remain inactive
- parked language work: LANGV1 conformance closeout remains parked; no
  language behavior is changed by the reprioritization

## Rule

This file is only a mirror. Implementation details, acceptance, landed history,
and parked tasks belong in the active card, the workstream SSOT, phase cards,
or git history.
