Status: SSOT mirror
Date: 2026-08-05
Scope: one-screen current dashboard. Do not store landed history here.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md

# Now

## Current

- current-state SSOT: `docs/development/current/main/CURRENT_STATE.toml`
- MirBuilder final pipeline: read `mirbuilder_north_star` in
  `CURRENT_STATE.toml`
- active lane: read `active_lane` in `CURRENT_STATE.toml`
- active phase: read `active_phase` in `CURRENT_STATE.toml`
- workstream card: read `latest_workstream_card` in `CURRENT_STATE.toml`
- latest card: read `latest_card_path` in `CURRENT_STATE.toml`
- blocker token: read `current_blocker_token` in `CURRENT_STATE.toml`
- current decision authority: read `latest_card_path` and
  `current_design_stop` in `CURRENT_STATE.toml`
- current execution authority: read `latest_card_path` in
  `CURRENT_STATE.toml`
- replacement law: read `method_anchor`; an I0 must switch a named production
  caller and retire the selected old edge
- replacement purpose: remove a competing authority and move the production
  graph toward `mirbuilder_north_star`; cell/pack/LOC counts are not the goal
- active row: read `current_execution_row`; use one atomic T0 I0/R0 whenever
  possible
- current frontier: Decision B-prime, M7-S2-A, the full M7-S3 LoopTrue
  source-to-Recipe cohort, Generic D2-B4-S1, D2-B4-S2, the scoped D3 typed
  matrix, S2A, and the resolved projector coverage row are closed as test-only
  evidence. The cfg(test)-only
  `JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-BRIDGE0-D1` source-backed handoff
  bridge is closed. The proposed V0-only D2 subrow was rejected by premise
  audit because actual raw facts produce `[V0,V1]`; no executable D2 subrow is
  selected. No Generic production Recipe, selector arm, source-to-selection handoff, route,
  physical, Retry, or fallback change is authorized; M10b still waits on
  M7/M8/M9 and D2. Read `current_execution_summary` and `current_design_stop`
  for the exact test boundary and non-claims.
- parked: Stage-B special activation, Ownership, Language v1 expansion,
  selfhost migration, cleanliness, and unrelated backend work

## Rule

This file is only a mirror. Implementation details, acceptance, landed history,
and parked tasks belong in the active card, the workstream SSOT, phase cards,
or git history.
