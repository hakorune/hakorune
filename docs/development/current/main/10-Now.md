Status: SSOT mirror
Date: 2026-08-06
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
- current loop frontier: common admission assembler S1 and the caller-zero
  selector S2 are landed. The shallow `GENERIC-SELECTION-OPEN-D0` gate is
  closed through its I0/R0 resolver-branded candidate envelope. The next
  boundary is `GENERIC-SELECTION-POLICY-HANDOFF-D0`. Production selection, Recipe handoff,
  physical cutover, and legacy removal remain closed.
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
- D3-S2-D1, P0 source-site totality, P1 source projection, P2 neutral facts,
  and P3 independent family-overlap census are closed docs/static/test-only.
  P0 proves current Generic facts are AST/name/derived products; P1 packages
  existing resolver/projector/bridge evidence in one machine-readable
  source-site/owner/BindingRef/strict-ancestor witness with typed pre-effect
  mismatch rows; P2 consumes that sealed witness and adds only a mode-neutral
  disposition; P3 retains raw Generic mode/carrier/schedule rows separately
  from resolved family/rejection rows. Only the Recipe producer may later
  issue `LoopBindingKeyV1`; Binding SSA owns physical ValueId/PHI. D4-WITNESS0
  is closed: one private MIR-root cfg(test) module lends paired raw/resolved
  views from one non-Clone canonical source receipt, with seven focused tests
  and no production caller. D4-S1 DirectAccum and D4-S2-S0 are also closed:
  the latter freezes six legacy-labelled same-source rows (two fixtures ×
  three modes). D4-S3-D0 is closed as the docs-only authority decision: a new
  resolver-branded observation set feeds a future family selector in
  `mir::loop_route_policy`; legacy schedule/cursor order is not policy.
  DirectAccum/NestedPredicate resolved lanes remain live, while Generic stays
  caller-zero. D4-S3-S0 is closed as a private cfg(test) witness: six
  resolver-branded sets (two fixtures × three modes), each with one receipt,
  mode snapshot, loop-window coverage seal, and unresolved family rows.
  D4-S3-S1 is also closed as a private cfg(test) matrix: nine source-backed
  fixture/mode sets with explicit V0Only/V1Only/Both/Neither cells,
  NoStandaloneRow and planner-freeze separation, and typed foreign/non-Loop
  rejects. D4-S3-S2 is closed as a private neutral selector consumer: all nine
  S1 rows remain typed Unresolved, with no Selected/NoCandidate, Recipe/key, or
  production caller. D4-S4-D0 is also closed as a worker-reviewed design:
  current SelectedFamilyV1 lacks provenance and current Generic facts are
  AST/Builder-derived, so a new resolver-issued AST-free semantic demand and
  one-shot source lease are required. D4-S4-S0 remains closed as a bounded
  NoSafeSlice audit after the cfg(test)-only two-role lease witness:
  no real Selected(Generic), Generic demand, or one-shot selected policy
  capability exists; the resolver AST-free candidate envelope and one-shot
  BindingRef lease witness are now closed. D4-S4-S0-D0 is now closed as a
  worker-reviewed design: resolver SourceLease, AST-free shape/candidate
  envelope, policy observation, selector, Generic demand, Recipe producer, and
  Binding SSA have separate issuers. GENERIC-SEMANTIC-SHAPE-SCHEMA-D1 is now
  closed as a typed Carrier/Condition/Step/BodyEffect/Coverage-Exit schema.
  The bounded CarrierProof witness is now closed: same-BindingRef
  NestedWrite -> PostLoopRead proof, lease-brand retention, and no source
  lifetime. D4-S4-S2-D1 closes the worker-reviewed design boundary: V1 stays
  immutable and V2 begins with inner-loop Condition+Step roles; BodyEffect/
  Coverage remain separate D0 cells. D4-S4-S2-D0 closes direct issuance as
  NoSafeSlice and selects the neutral inventory prerequisite. Inventory S0 and
  V2 role issuance are now closed: resolver traversal co-seals branded point
  membership, and the move-only catalog validates Condition+Step topology.
  D4-S4-S3-D0 is now closed by worker authority split: resolver/source-view
  publishes AST-free syntax facts and policy owns operator/type/overflow/
  monotonicity. D4-S4-S3-S0 is closed with six cfg(test)-only syntax-fact
  witnesses and no public reference row. D4-S4-S3-D1 and S1-D0 are closed;
  S1-S0 is closed with its cfg(test)-only source-unit receipt/map witness; the
  S1-S1 is now closed by the cfg(test)-only move-only co-sealed receipt and six
  focused tests. S1-S2-D0 fixed the two-stage policy boundary, S1-S2-S0 closed
  the substrate projection, and S1-S2-S1-S1 closed the one-consume policy
  witness. The deep D4 evidence exit is closed. The worker-reviewed current
  stop is GENERIC-SOURCE-TO-PORTABLE-RECIPE-D0: G0 has explicit `: i64`,
  contextual plain literals, three recurrence carriers for `i/j`, and a
  separate post-loop completion envelope. Common nested shadow, logical
  Header/After binding, source-bound Recipe core co-seal, and route-independent
  producer provenance are closed before the Generic producer; only S4 issues
  real G0 keys/relations. Generic candidate S1 is now closed as a caller-zero
  Less/positive-Add Candidate/Unresolved/Rejected observation with neutral
  operator facts. DirectAccum S1 is now closed as a caller-zero AST-free
  Candidate/Declined/Unresolved/Rejected observer with a test-only source
  adapter, seven focused tests, and a green shared guard. LoopCond S1 is now a
  landed caller-zero observer with a green shared family guard. Generic
  G0 row normalization is now landed caller-zero with 12 adapter tests, 7
  policy tests, and a green shared guard. A design audit found that D/U/R
  variants drop row metadata, so the next exact row is
  FAMILY-ROW-CONTEXT-RETENTION-R0, resolver-owned
  LOOP-FAMILY-WINDOW-LEASE-ISSUER-S0, and the common five-family admission
  assembler S1 are landed. The worker-reviewed Ready-only selector design and
  its caller-zero implementation are now closed: `family_selector.rs` covers
  five typed candidates, `Overlap`, and `OutOfWindow` with retained lease/row
  evidence. The finite shallow order and atomic legacy-retirement boundary
  live in the dedicated Generic SSOT. The shallow
  `GENERIC-SELECTION-OPEN-D0-I0-R0` candidate-envelope witness is now closed;
  the next boundary is policy/selector handoff. Production and public
  reference activation remain zero.
- parked: Stage-B special activation, Ownership, Language v1 expansion,
  selfhost migration, cleanliness, and unrelated backend work

## Rule

This file is only a mirror. Implementation details, acceptance, landed history,
and parked tasks belong in the active card, the workstream SSOT, phase cards,
or git history.
