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
  source-to-Recipe cohort, and Generic D2-B4-S1 are closed. The active stop is
  D2-B4-S2: a test-only `BindingRefV1` disjointness witness must prove that an
  inner write and post-loop outer `j` read share the same resolved binding;
  shadowing must reject. The parent Generic D2 disposition remains unresolved.
  No Generic production Recipe, route, physical, Retry, or fallback change is
  authorized; M10b still waits on M7/M8/M9 and D2.
  The bounded parsed-source witness is green (3 tests): planner-required V0
  suppression is typed and mode-scoped, while the positive/negative evidence
  remains resolver/source identity rather than runtime parity.
- parked: Stage-B special activation, Ownership, Language v1 expansion,
  selfhost migration, cleanliness, and unrelated backend work

## Rule

This file is only a mirror. Implementation details, acceptance, landed history,
and parked tasks belong in the active card, the workstream SSOT, phase cards,
or git history.
