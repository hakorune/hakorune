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
  audit because actual raw facts produce `[V0,V1]`. The bounded
  `JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-PLANNER-SUPPRESSION0-D2-S1`
  row is closed as cfg(test)-only evidence: actual Strict+planner-required mode
  co-seals the existing S2A source and yields typed unresolved raw `[V1]` after
  V0 suppression. `JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-INDEX-AMBIGUOUS0-D2-S2`
  is now closed as cfg(test)-only evidence: parsed nested IndexWrite plus
  facts Ambiguous, actual Release/Strict raw `[V0,V1]`, and typed unresolved
  pre-effect disposition. `JOINIR-GENERIC-RESOLVED-CARRIER-ELIGIBILITY-PROTOCOL0-D3-S0`
  is now closed as cfg(test)-only source-backed natural-Both eligibility with
  typed mismatch negatives. Execution has returned to the parent
  `JOINIR-GENERIC-RESOLVED-CARRIER-SELECTION-BOUNDARY-D3-DESIGN0-D0` design
  stop. The bounded Compound/Unavailable row is now closed as
  `JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-COMPOUND-UNAVAILABLE0-D2-S3`:
  parsed nested CompoundAssignment, exact facts Unavailable, measured
  `[V0,V1]`, and typed pre-effect unresolved evidence only. Execution returns
  to the parent design stop. No
  Generic production Recipe, selector arm, source-to-selection handoff, route,
  physical, Retry, or fallback change is authorized; M10b still waits on
  M7/M8/M9 and D2. Read `current_execution_summary` and `current_design_stop`
  for the exact boundary and non-claims.
- D2-S4 is closed as cfg(test)-only evidence for parsed top-level
  `CompoundAssignment`: resolver/source/frame/BindingRef identity is present,
  but facts are absent and Release/Strict both measure raw schedule `[]`.
  The typed disposition is `NoStandaloneRow`; no CompleteNoRecursive,
  Unavailable, V0-only, selector, eligibility, production handoff, Recipe,
  PHI, Builder, MIR, Retry, or fallback claim follows. A separate parsed
  Both/NoRecursive row requires a new design decision.
- D2-S5-S1 is closed as cfg(test)-only evidence for one parsed flat
  Assignment shape. Exact `CompleteNoRecursiveCarrier` plus Release/Strict
  raw `[V0,V1]` maps only to typed `UnresolvedStop(NonRecursiveOutOfTarget)`;
  no selector, eligibility, Legacy, Recipe, PHI, Builder, MIR, Retry,
  fallback, or production handoff moved. Execution returns to the parent D3
  design stop for the remaining matrix and winner/disjointness work.
- D3-S1 is closed as the prior policy boundary. D3-S2-S0, D3-S2-S1, and the
  bounded D3-S2-S2 cfg(test)-only passive provenance product are closed.
  S2 consumes one co-sealed handoff in a private non-Clone
  `resolved_semantics` factory and rejects typed owner/forest/frame/role
  mismatches before effects. The selected D3-S2-S3 repeat audit consumes two
  complete S2 products as one non-Clone pair and observes structural equality,
  distinct resolver brands, and raw frame-coordinate collision. It adds no
  Generic snapshot/key/seed, selector, Builder/MIR/Recipe/PHI,
  Return/Home/debt, DirectAccum frame, or production authority; after the
  audit the frontier returns to the D3-S2 design stop.
- parked: Stage-B special activation, Ownership, Language v1 expansion,
  selfhost migration, cleanliness, and unrelated backend work

## Rule

This file is only a mirror. Implementation details, acceptance, landed history,
and parked tasks belong in the active card, the workstream SSOT, phase cards,
or git history.
