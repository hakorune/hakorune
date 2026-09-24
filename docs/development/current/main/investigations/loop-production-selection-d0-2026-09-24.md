---
Status: open__design_stop__boundary_census_recorded
Task: LOOP-PRODUCTION-SELECTION-D0
Date: 2026-09-24
Parent: workstream row F (`LOOP-PRODUCTION-SELECTION-D0 -> Loop/M10b -> M11 -> M12`)
PreviousRow: SELFHOST-LOOP-PORTABLE-ALL19-PARITY-S7G (landed caller-zero)
NextCard: GENERIC-M10B-DELETION-MANIFEST-S0 (bounded, conditional on the
  recorded Decision)
Contract source: `mirbuilder-final-pipeline-ssot.md` return task —
  "consume the M10 prerequisite list and the closed-status evidence in
  `generic-loop-source-to-portable-recipe-ssot.md` and
  `loop-common-physical-demand-and-session-ssot.md`, select the first
  unmet dependency by full task ID, then return to production selection."
---

# LOOP-PRODUCTION-SELECTION-D0 — production selection design

## Six-line brief

```text
Decision: the selected production boundary is the already-connected canonical
  winner set (three `CanonicalLoopFamilyPlanV1` arms, three Main0
  program-root routes, one Callable edge); every M10 seal row inside that
  boundary is closed, and the two literally-open seal rows are scoped
  outside it by their own caller-zero family boundaries.
Source authority + canonical issuer: `MirCompiler::compile_resolved` ->
  `compile_resolved_first_family` -> `CanonicalLoweringPreflightV1::verify`
  (NestedPredicate > DirectAccum > G0 probe) -> source-bound cutover ->
  publish; `route_loop` is the single legacy caller being switched.
Non-authority: `.hako` wire emitters and parity harnesses (caller-zero
  coverage artifacts), the 19-route ordered scheduler's route IDs as
  production selectors, M8 producer cohorts as production arms, and any
  Recipe/JoinSig/Layout re-inference at the switch point.
Fail-fast boundary: an M10 seal row required by a selected profile that is
  not closed, an accepted corpus row with no implemented portable owner or
  accepted typed reject, or a winner dispatch admitting a profile with no
  production consumer is a reject, not a gap to paper over.
Smallest next slice: `GENERIC-M10B-DELETION-MANIFEST-S0` — freeze the exact
  selected old symbols/callers immediately before cutover, in literal seal
  order after this card's boundary decision.
Non-claims: no production switch in this card, no M10b activation, no
  legacy deletion, no new Verified*/Prepared* receipt, no reopening of
  closed seal rows, no corpus re-census, no `.hako` producer claim.
```

## Row F gate status

Row F has exactly two named gates (S6G card):

1. **M8 all-route coverage — CLOSED.** S6G landed
   `VerifiedLoopAllRouteObservationSetV1` + `WholeUnitLoopCoverageProofV1`
   and opened `NoCandidate`; S7G landed the `.hako` wire parity closeout
   (8 wire-backed attested routes anchored on live producer products, 11
   typed-declined). Caller-zero evidence; no production claim.
2. **M10 pre-cutover seal series — evaluated below.** The series is a
   shallow ordered list in `joinir-loop-selfhost-recipe-pipeline-ssot.md`
   ("Before `LOOP-PRODUCTION-SELECTION-D0` and M10b, close this shallow
   ordered series"). This card consumes the list in literal order and
   resolves each open row against the selected boundary, which is the
   scoping decision this row exists to make.

## Census A — M10 seal series, boundary-scoped status

| # | Seal row (full task ID) | Literal status | Boundary resolution |
| --- | --- | --- | --- |
| 1 | `LOOP-SEMANTIC-PROGRAM-COSEAL-R0` | closed `NoSafeSlice` 2026-09-11; rerouted through the G0 chain | satisfied for the selected boundary: per-family co-seals landed for every selected profile (GenericG0 `VerifiedGenericG0SourceParentV1`, Main0 `VerifiedMain0*RecipeProductV1`, DirectAccum/NestedPredicate plan arms, Callable product). The universal all-family `VerifiedLoopSemanticProgramV1` stays a non-goal — its absent families are not in the boundary |
| 2 | `LOOP-PHYSICAL-TRANSFER-AUTHORITY-R0` | closed (archive `dynamic-fault-exit-transaction-d0-history`) | in-boundary, closed |
| 3 | `LOOP-COMMON-TRANSFER-BOUND-SEGMENT-INPUT-R0` + `LOOP-PHYSICAL-TRANSFER-AUTHORITY-R0` cleanup `LOOP-PHYSICALIZER-BOUNDARY-CLEANUP-D0` | closed (`28c4bdd5c4`, `46fbf8d0d7`) | in-boundary, closed |
| 4 | `LOOP-S6C-COMMON-V2-PRESESSION-CONTRACT-D0` | accepted design-only | contract-level obligation met; its I0 is row 5 below |
| 4a | `CALLABLE-TEXT-FORMAL-PHYSICAL-SIGNATURE-D0`/`I0` | closed | in-boundary, closed |
| 5 | `LOOP-S6C-COMMON-V2-PRESESSION-I0` | **open — parked caller-zero** | **outside the selected boundary.** Its own park condition: the scheduler cannot select it "until one already-inventoried production consumer and its exclusive old-edge delete set are named." The only V2-envelope consumer family is cursor 7 `ScanWithInit` (`LoopRecipeArtifactV2`), which is caller-zero and has no accepted corpus row. The selected winner's lanes produce V1 products and never traverse the V2 envelope transport. Reopen trigger: a selected profile requiring the S6C V2 envelope names its consumer and delete set |
| 6 | `LOOP-PHYSICAL-ALWAYS-COVERAGE-I0` / `LOOP-PHYSICAL-IF-COVERAGE-I0` / `LOOP-PHYSICAL-EXIT-COVERAGE-I0` | IF + EXIT closed 2026-08-11; **ALWAYS has no closure evidence** | **ALWAYS is outside the selected boundary.** Its only consumer is `LoopConditionV1::Always` = cursor 12 `LoopTrueBreakContinue`, caller-zero, with no accepted corpus row. IF/EXIT are in-boundary and closed. Reopen trigger: an admitted `LoopConditionV1::Always` profile |
| 7 | `LOOP-COMMON-V2-PHYSICAL-SESSION-I0` | closed (implementation receipt 2026-08-17) | in-boundary, closed — the canonical CFG/Binding-SSA/Phi/Completion/DraftSeal session is reused as-is |
| 8 | `LOOP-PRECUTOVER-AUTHORITY-G0` | landed for the Generic G0 lane (D0 -> preflight issuer -> production terminal -> canonical issuer I0 -> hardening -> normal-package consumer -> source-to-EXE publication -> acceptance -> helper-backend reach, verified `SourceCalledEXE3`); sibling H2 closed | satisfied for the selected boundary: each selected profile reached its own pre-cutover chain (Main0 production arms 2026-09-23, DirectAccum/NestedPredicate canonical preflight issuers, Callable edge). The literal "every admitted all-19 fixture" wording is read boundary-scoped: the admitted set is the selected boundary, not the 19-route scheduler's vocabulary |

Literal first unmet dependency in seal order is row 5
(`LOOP-S6C-COMMON-V2-PRESESSION-I0`), then row 6-ALWAYS. Both evaluate to
outside the selected boundary by their own family scoping — not skipped,
scoped out with recorded reopen triggers. All seal rows inside the
selected boundary are closed.

## Census B — accepted corpus owner resolution

From `design/fixtures/generic-loop-legacy-disposition-v1.tsv`
(`D0-DISPOSITION-CHECKED`, `portable-owner` rows; S6G owner->producer
resolution):

| `target_owner` | Rows | Observed route today | Resolution |
| --- | --- | --- | --- |
| `main0-continue` | 9 | canonical-main / release-adopt | `Main0ContinueV1` — production-connected via `src/mir/builder/program_root_lowering/main0_continue_route.rs` |
| `main0-in-body-step` | 3 | canonical-main / release-adopt | `Main0InBodyStepV1` — production-connected via `main0_in_body_step_route.rs` |
| `main0-derived-predicate` | 4 | canonical-main | `Main0DerivedPredicateV1` — production-connected via `main0_derived_predicate_route.rs` |
| `callable-loop` | 1 | callable-loop | `CallableSingleLoopV1` — existing production edge |
| `canonical-main` | 23 | canonical-main | 21 rows carry no loop (`StepTree root` / blockexpr/if fixtures) — canonical-main callable route owns them without any loop producer. 2 rows (`selfhost_parse_program2_loop_if_return_local_min` apps + phase29bq twin) observe `[flowbox/adopt box_kind=Loop features=return via=shadow]`; the legacy loop path already emits an accepted typed reject (`callable-loop-handoff`/`callable-loop` sibling rows), and the canonical-main route owns them — no loop-cohort gap |

M10b Stop clause check: every accepted row resolves to an implemented
portable owner or an accepted typed reject. No corpus row resolves to
`GenericResidual`, `LoopTrueBreakContinue`, `ScanWithInit`, or any other
caller-zero-only family.

## Census C — production arms vs caller-zero cohorts

Production-connected today:

- `CanonicalLoopFamilyPlanV1` arms (`capability/first_family_plan.rs`):
  `DirectAccum`, `NestedPredicate`, `GenericG0` — reached through
  `compile_resolved` -> `compile_resolved_first_family` ->
  `CanonicalLoweringPreflightV1::verify` (probe order NestedPredicate >
  DirectAccum > G0 > ordinary Trivial/A+).
- `program_root_lowering` routes: `main0_continue_route.rs`,
  `main0_in_body_step_route.rs`, `main0_derived_predicate_route.rs`
  (consume `VerifiedMain0*RecipeProductV1`).
- Callable loop edge (`CallableSingleLoopV1`).

Caller-zero (coverage artifacts and parked families — none selected):

- The 8 attested recipe-backed producers (S7G wire parity evidence):
  `VariableAccumRecurrenceV1`, `LoopBreakRecipe` family,
  `ScanWithInitV2`, `DirectAccumV1`, `NestedPredicateV1`,
  `LoopTrueBreakContinueV1`, `LoopCondBreakContinueV1`,
  `GenericResidualV1`. Their wire cohorts prove artifact parity; they
  are not production arms unless a plan arm or program-root route
  consumes them (DirectAccum/NestedPredicate do via plan arms).
- `GenericResidual` projection/typed-map/coseal chain: no
  `src/mir/builder/` consumer; no corpus row needs it.
- `LoopTrueBreakContinue` (Always) and `ScanWithInit` (S6C V2 envelope):
  the families that would consume seal rows 5/6 — both outside.

## Decision — selected boundary and first unmet dependency

**Selected winner boundary**: the profiles that already carry production —
`DirectAccum`, `NestedPredicate`, `GenericG0` (plan arms),
`Main0ContinueV1`, `Main0InBodyStepV1`, `Main0DerivedPredicateV1`
(program-root routes), `CallableSingleLoopV1` (callable edge), plus the
non-loop `canonical-main` corpus rows that never enter the loop path.
This boundary covers all 40 `portable-owner` corpus rows and all
accepted typed rejects stay typed rejects.

**First unmet dependency by full task ID**: none inside the selected
boundary. In literal seal order the first unmet row is
`LOOP-S6C-COMMON-V2-PRESESSION-I0` (parked; requires a named production
consumer the selected boundary does not have), followed by
`LOOP-PHYSICAL-ALWAYS-COVERAGE-I0` (no closure evidence; sole consumer
`LoopConditionV1::Always` is outside). Both keep their own reopen
triggers recorded in Census A. Neither blocks the switch.

**Production selection**: the winner is the existing canonical chain —
`route_loop` (`route_entry/router.rs:255`, invoked at `routing.rs:552`)
switches to frozen source -> StructuralFacts -> one policy winner
(`select_canonical_loop_family_v1` over the landed products) -> verified
recipe -> the one canonical physicalizer -> one external commit. Loop
sources outside the selected profiles hit the existing typed-decline
outcomes and `Freeze` — terminal failure, not fallback.

**Bounded next slice**: `GENERIC-M10B-DELETION-MANIFEST-S0` — freeze the
exact selected old symbols and caller counts immediately before cutover
(the ordered retry scheduler, Generic post-effect retry debt, private
continuation/error-to-None edges, Generic V0/V1 registry
handler/predicate edges, nested Generic `.ok()`/retry edges, selected
old JoinIR caller/physical edges, new-subtree AST reconstruction
facades). `generic-loop-legacy-disposition-v1.tsv` is inventory
evidence, not deletion authority; the manifest re-freezes at cutover
time.

## Fail-fast boundary

- Any seal row reclassified into the selected boundary by a later
  finding reopens this card before M10b work.
- The manifest must be checked against live symbol/caller counts; a
  stale or unmatched row stops cutover.
- A loop source that matches a selected profile but reaches an
  unimplemented consumer is a reject, not a NoCandidate fallback.
- The corpus is consumed as recorded; new fixtures do not silently
  widen the boundary.

## Non-claims

- No production switch, no `route_loop` edit, no registry/handler
  change, no legacy deletion in this card.
- No M10b activation: the manifest is the next slice, cutover is
  `M10b-I0-R0`'s own atomic commit.
- No new `Verified*`/`Prepared*` receipt: the selection is a recorded
  boundary decision over already-sealed products.
- No seal-row reopening and no re-census of the corpus, the 19 routes,
  or the wire cohorts.
- No claim that the caller-zero producer cohorts are production arms;
  their production connection, if any, belongs to later rows (M11/M12
  chain or their own family rows).
- No `.hako` producer/verifier claim; the S7 wire evidence stays
  caller-zero coverage.
- B3's Call-family `NoSafeSlice` (substring route/codepoint outcome,
  ArrayPush provider) is not a Row-F gate and stays family-local.

## Exit

- `work_mode = design_stop` is preserved: this card is the design
  decision; the next bounded slice (`GENERIC-M10B-DELETION-MANIFEST-S0`)
  opens under its own card/mode transition.
- Pointer: `next_execution_card` updates to the manifest row at the
  next scheduler step; this card's Decision is its entry evidence.
