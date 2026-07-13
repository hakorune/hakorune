Status: SSOT mirror
Date: 2026-07-13
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
- priority: read `active_lane` and `current_blocker_token`; owner-forest
  P0/E0/OF0/UP0/UP1/B0-D/B0-P/B0-S/B0-F are closed, B0-C was skipped, and
  B0-L0/B0-L1/B0-L2a are closed; B0-L2b immutable exact source navigation is
  active with production resolved callers and semantic activation zero;
  RegionFlow and parser/source-carrier P1 remain disconnected
- parked language work: LANGV1 conformance closeout remains parked; no
  language behavior is changed by the reprioritization

## Rule

This file is only a mirror. Implementation details, acceptance, landed history,
and parked tasks belong in the active card, the workstream SSOT, phase cards,
or git history.
