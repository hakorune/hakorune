---
Status: landed__2026-09-23__caller_zero
Task: JOINIR-LOOP-M8-ALL19-CLOSEOUT-S6G
Date: 2026-09-23
Parent: JOINIR-LOOP-M8-GENERIC-RESIDUAL-S6E (landed)
PreviousCard: joinir-loop-m8-generic-residual-s6e-d0-2026-09-23.md
NextCard: frontier-pause__family_scheduler_reselection
Implementation permission: false; name the owners to extend and fix the
bounded implementation slice only. No code, no manifest write, no
route/caller change, no new semantic receipt from this card.
---

# JOINIR-LOOP-M8-ALL19-CLOSEOUT-S6G — D0 all-route closeout design

## Six-line brief

```text
Decision: one caller-zero all-route closeout slice — seal `VerifiedLoopAllRouteObservationSetV1` (one typed outcome per canonical route) plus `WholeUnitLoopCoverageProofV1` into the same `CanonicalLoopFamilySelectionV1` selector introduced at S2, and open its `NoCandidate` result. No producer cohort, no second selector, no route-ID/cursor selection.
Source authority + canonical issuer: `loop_route_policy::issue_all_route_observation_set_v1` is the sole observation-set sealer over the card-fixed route inventory; `loop_route_policy::issue_whole_unit_loop_coverage_proof_v1` is the sole coverage-proof issuer; `select_canonical_loop_family_v1` (family_selector.rs, the S2 selector) remains the sole selection owner and is the only place `NoCandidate` may be returned.
Non-authority: raw route order/cursor selection, producer/Recipe issuance, AST/name lookup, the legacy 19-route evaluator as a selection authority, Builder/MIR/CFG/PHI/physical products, retry/fallback/`Option` skip, production callers, corpus re-census.
Fail-fast boundary: exactly 19 rows in canonical order; each row is `RecipeBacked{producer_id}` only where the card inventory attests it, else `PreEffectDeclined{reason}` — zero unclassified; a missing/duplicate/out-of-order row is a typed reject; corpus owner→producer resolution must be total for accepted rows.
Smallest next slice: `loop_route_policy/all_route_observation.rs` (set + coverage proof issuers) + `CanonicalLoopFamilySelectionOutcomeV1::NoCandidate` arm + `producer_id_migration_tests.rs` extended to all 19 routes + focused positive/negative tests.
Non-claims: caller-zero only; no production caller or `route_loop` connection; no M9 parity; no Row F (`LOOP-PRODUCTION-SELECTION-D0`) unblock by itself — this row closes M8 all-route coverage, one of Row F's two gates (the M10 seal series remains); no legacy deletion; no new producer coverage claim.
```

## Row contract

From `generic-loop-source-to-portable-recipe-ssot.md` (coverage table):

> `JOINIR-LOOP-M8-ALL19-CLOSEOUT-S6G` | all M8 observations ->
> `VerifiedLoopAllRouteObservationSetV1` + `WholeUnitLoopCoverageProofV1`
> -> the same `CanonicalLoopFamilySelectionV1` | **all 19 routes are
> typed pre-effect decline or verified Recipe; accepted corpus parity
> and unclassified count zero; this row alone opens NoCandidate** |
> implementation-coupled closeout, not docs-only; no second selector,
> route-ID/cursor selection, production caller, Option/retry/fallback.

And `joinir-loop-selfhost-recipe-pipeline-ssot.md`:

> S6G is an implementation-coupled closeout: it seals
> `VerifiedLoopAllRouteObservationSetV1` plus the whole-unit coverage
> proof into the same `CanonicalLoopFamilySelectionV1` introduced at S2
> and opens its `NoCandidate` result. It does not create a second
> selector and its production caller remains zero.

Ordered ladder:

```text
S6A (landed) -> S6B (landed) -> S6C (landed, V2) -> S6D (landed)
  -> S6E (landed) -> S6G <- this row -> S7A..S7G
  -> M10 seal series -> M10b -> R1/M11/M12
```

This is a milestone closeout row, not a Promote/Stop/Delete
edge-reduction row and not a producer cohort: it classifies existing
evidence into one sealed all-route inventory and opens one selector
result. It adds no new semantic meaning — the sealed products record
coverage, they do not admit source shapes.

## Why now (prerequisites satisfied)

- All five ordered producer cohorts landed caller-zero: S6A
  `VariableAccumRecurrenceV1`, S6B `VariableAccumBreakV1`, S6C
  `produce_s6c_scan_with_init_recipe_v2` (V2 wire), S6D
  `LoopCondBreakContinueV1` (cursor 13), S6E `GenericResidualV1`
  (cursor 18, Exact-only).
- Corpus closure: P1 398/398 observed; D0 398/398 classified
  (40 accepted / 196 typed-reject / 162 pre-loop; unclassified
  accepted = 0 after the `e6178e7335` determinism repair); S0 edge
  inventory 270 rows.
- Existing vocabulary to reuse: `CANONICAL_LOOP_ROUTE_ORDER_V1` /
  `FrozenLoopRouteScheduleV1` (schema.rs:12-32),
  `LoopRoutePolicySourceDeclineReasonV1::PreEffectDeclined`
  (policy_evidence.rs), the five-row assembler
  (`family_admission.rs`), the S2 selector (`family_selector.rs`),
  the builder-free all-route preflight vocabulary
  (`registry/loop_preflight.rs` — "a later row may add the sole
  producer after every route has a truthful source and policy
  proof"), and the parity-receipt pattern
  (`producer_id_migration_tests.rs`: "deliberately outside the
  portable artifact … proves the migration inventory without giving
  the portable schema a route selector").

## Missing (the row's deliverable)

- `VerifiedLoopAllRouteObservationSetV1` — one sealed row per
  canonical route: `RecipeBacked{producer_id}` or
  `PreEffectDeclined{reason}`. Zero matches today.
- `WholeUnitLoopCoverageProofV1` — the whole-unit coverage proof the
  test-only stub `WholeUnitNoLoopEnvelopeProofV1`
  (family_selection.rs:23-27) anticipated: "a future source bridge
  must issue it only after sealing every semantic family as typed
  `Declined`."
- `CanonicalLoopFamilySelectionOutcomeV1::NoCandidate` — the outcome
  enum (family_selector.rs:123-127) currently has only
  `Selected | Rejected | Unresolved`; window input is five-row scoped
  and can never produce `NoCandidate`.
- Route coverage inventory — this card fixes it below.
- Parity receipt — `producer_id_migration_tests.rs` covers only 5 of
  19 routes and now mislabels `GenericLoopV1` as `legacy_only`;
  extend to all 19 rows.

## Route coverage inventory (this card's design decision)

`LoopRouteId` is the single legacy scheduler identity
(`loop_recipe_contract/route_id.rs`, 19 variants; canonical order at
`loop_route_policy/schema.rs:12-32`). Producer cohorts are
family-branded and never claim route ownership; the observation set
is **migration inventory** — it records, per route, whether a landed
portable Recipe cohort demonstrably covers that route's bounded
source profile, or the route is a typed pre-effect decline. This is
the same non-selecting correspondence the parity receipt already
records; it selects nothing.

| Cursor | Route | Outcome | Attestation |
| --- | --- | --- | --- |
| 0 | LoopBreakRecipe | `RecipeBacked{VariableAccumBreakV1}` | S6B fixture `loop_break_plan_subset_min.hako` (predicate Loop + direct Break); slice verifies the cohort profile covers the route's accepted shape, else this row declines |
| 1 | IfPhiJoin | `PreEffectDeclined` | shared If/join obligations, "not a Loop Recipe kind" (pipeline SSOT) |
| 2 | LoopContinueOnly | `PreEffectDeclined` | no landed cohort |
| 3 | LoopTrueEarlyExit | `PreEffectDeclined` | no landed cohort |
| 4 | LoopSimpleWhile | `RecipeBacked{VariableAccumRecurrenceV1}` | S6A fixture `loop_simple_while_inline_explicit_step_min.hako`; same verification gate as cursor 0 |
| 5 | LoopCharMap | `PreEffectDeclined` | S6C kept CharMap a separate family |
| 6 | LoopArrayJoin | `PreEffectDeclined` | S6C kept ArrayJoin a separate family |
| 7 | ScanWithInit | `RecipeBacked{S6C V2 producer}` | `scan_with_init_typed_ok_min.hako`; forward ScanWithInit only |
| 8 | SplitScan | `PreEffectDeclined` | S6C kept SplitScan a separate family |
| 9 | BoolPredicateScan | `PreEffectDeclined` | S6C kept BoolPredicateScan a separate family |
| 10 | AccumConstLoop | `RecipeBacked{DirectAccumV1}` | cursor-pinned demand (policy.rs `into_direct_accum_v1`) |
| 11 | NestedLoopMinimal | `RecipeBacked{NestedPredicateV1}` | parity-receipt attested |
| 12 | LoopTrueBreakContinue | `RecipeBacked{LoopTrueBreakContinueV1}` | cursor-pinned demand |
| 13 | LoopCondBreakContinue | `RecipeBacked{LoopCondBreakContinueV1}` | S6D cursor-pinned demand |
| 14 | LoopCondContinueOnly | `PreEffectDeclined` | outside S6D bounded profile |
| 15 | LoopCondContinueWithReturn | `PreEffectDeclined` | outside S6D bounded profile |
| 16 | LoopCondReturnInBody | `PreEffectDeclined` | outside S6D bounded profile |
| 17 | GenericLoopV0 | `PreEffectDeclined` | S6E types V0-overlap winners as Unresolved; never re-minted as V1 or G0 provenance |
| 18 | GenericLoopV1 | `RecipeBacked{GenericResidualV1}` | S6E cursor-pinned demand, Exact winner only, bounded single-induction profile |

Verification gate for cursors 0/4/7: `RecipeBacked` requires the
slice to attest the cohort's bounded profile covers the route's
fixture/source shape (named fixture + profile correspondence); if the
correspondence fails, the row is a typed `PreEffectDeclined` — the
set is never left unclassified and the gate fails loud, it does not
silently downgrade.

`GenericG0`, `CallableSingleLoopV1`, and the `Main0*` cohorts are
semantic-family / corpus authorities outside the canonical route
inventory; they feed corpus parity, not route rows.

## Accepted corpus parity (this card's design decision)

`parity_gate` stays `not-run` — D0 froze "no parity gate exists for
this corpus yet; do not invent one" and this row does not invent a
runtime gate. Parity here is **inventory-level**: every
`D0-DISPOSITION-CHECKED` accepted corpus row's recorded
`target_owner` token resolves to a landed portable producer cohort
through a fixed owner→producer table written in this row:

| `target_owner` token | Rows | Resolves to |
| --- | --- | --- |
| `main0-continue` | 9 | `Main0ContinueV1` |
| `main0-in-body-step` | 3 | `Main0InBodyStepV1` |
| `main0-derived-predicate` | 4 | `Main0DerivedPredicateV1` |
| `callable-loop` | 1 | `CallableSingleLoopV1` |
| `canonical-main` | 23 | canonical-main callable route — resolves per-row to the named Main0/Callable/GenericResidual cohort covering that row's loop profile, attested in the slice |

An unresolved owner token or an accepted row whose owner has no
landed cohort is a fail-fast gap — the closeout does not ship with an
orphan. Rejected/pre-loop rows keep their D0 dispositions; this row
re-reads, it does not re-classify or re-census.

## Owners to extend (bounded implementation slice)

| File | Change | Owner boundary |
| --- | --- | --- |
| `loop_route_policy/all_route_observation.rs` (new) | `LoopRouteObservationOutcomeV1`, `VerifiedLoopAllRouteObservationSetV1`, `issue_all_route_observation_set_v1` | sole sealer; consumes the card-fixed inventory + landed cohort receipts; canonical order, exactly 19, no duplicates, typed outcomes only; no source/AST/route re-observation |
| `loop_route_policy/all_route_observation.rs` (same file or sibling) | `WholeUnitLoopCoverageProofV1`, `issue_whole_unit_loop_coverage_proof_v1` | sealed only from a complete observation set plus unit/frame identity consistent with the family window lease; no new source authority |
| `loop_route_policy/family_selector.rs` | add `CanonicalLoopFamilySelectionOutcomeV1::NoCandidate(WholeUnitLoopCoverageProofV1)` | same selector, not a second one; `NoCandidate` is issuable only with the coverage proof — the five-row window alone still cannot produce it |
| `loop_route_policy/mod.rs` | module wiring | re-export only |
| `loop_recipe_contract/producer_id_migration_tests.rs` | extend parity receipt to all 19 routes; fix `GenericLoopV1` to `RecipeBacked{GenericResidualV1}`; add the newer producer ids to the roundtrip list | non-selecting external mapping only; no route selector enters the portable schema |
| focused tests beside each touched file | set sealing (order/duplicate/missing/foreign outcomes), coverage-proof issuance, `NoCandidate` arm, parity receipt completeness | caller-zero evidence only |
| `loop_route_policy/README.md`, `loop_recipe_contract/README.md`, `docs/reference/mir/loop-recipe-contract.md`, `generic-loop-stage-matrix.md` | document the observation set, coverage proof, and opened `NoCandidate` | same-slice doc closeout |
| shared `mirbuilder_inplace_replacement_guard.sh` | extend only if new vocabulary lands | S6G is in the `P0/S6G/S7G/M10b/M11/R2G` gate class |

## Fail-fast boundary

- The observation set requires exactly 19 rows in canonical order;
  missing, duplicate, out-of-order, or untyped rows are typed rejects.
- `RecipeBacked` is admitted only for the card inventory's attested
  correspondences; an unattested producer claim is a reject, not a
  silent inclusion.
- `NoCandidate` is returned only by `select_canonical_loop_family_v1`
  and only when the coverage proof accompanies a window with no
  candidate; window-scoped input without the proof still cannot
  produce it.
- Corpus owner→producer resolution must be total over the 40
  accepted rows; an unresolved owner fails the slice.
- No `Option`/skip/retry/fallback anywhere in the slice.

## Non-claims (stop conditions)

- Caller-zero only: no production caller, no `route_loop`/registry/
  handler connection, no production switch, no caller-zero-retirement
  claim from green tests.
- No second selector, no route-ID/cursor selection, no new family
  admission tag, no `LoopRecipeV1` schema widening.
- No producer cohort work: this row issues no Recipe, JoinSig, Facts,
  source projection, or typed map; landed cohorts are cited as
  evidence, not re-derived.
- No M9/S7 parity, no physical/CFG/PHI work, no runtime acceptance
  claim.
- No Row F unblock claim by itself: S6G closes the M8 all-route
  coverage gate; `LOOP-PRODUCTION-SELECTION-D0` remains gated on the
  M10 pre-cutover seal series.
- No corpus re-census and no parity-gate invention: `parity_gate`
  stays `not-run`; parity evidence is the owner→producer resolution
  table plus the extended receipt.
- No legacy deletion or cutover; `GENERIC-M10B-DELETION-MANIFEST-S0`
  stays blocked until M8/M9 and the M10 seal rows close.
- No `GenericLoopV0`/`GenericG0` provenance minting for V0 sources.

## Exit

When this card is accepted, `work_mode` moves to `fast` for the
bounded implementation slice named above (one closeout slice, one
commit family): observation set + coverage proof + `NoCandidate` arm
+ parity receipt extension + focused tests + doc closeout.

## Landed evidence (2026-09-23)

Implementation (caller-zero):

- `loop_route_policy/all_route_observation.rs` —
  `LoopRouteObservationOutcomeV1` (`RecipeBacked` |
  `PreEffectDeclined`), `LoopRouteRecipeBackingV1`
  (`PortableProducer(LoopRecipeProducerIdV1)` | `ScanWithInitV2`),
  `VerifiedLoopAllRouteObservationSetV1`,
  `issue_all_route_observation_set_v1`, `WholeUnitLoopCoverageProofV1`,
  `issue_whole_unit_loop_coverage_proof_v1`. The closed
  `ATTESTED_RECIPE_BACKED_V1` table attests the eight backed routes
  (cursors 0/4/7/10/11/12/13/18); any other `RecipeBacked` claim is a
  typed `UnattestedRecipeBacking` reject. At most one backed row per
  set (`MultipleRecipeBacked`); exact 19-row canonical order enforced
  positionally (`RowCountMismatch` / `RouteOrderMismatch`); the raw
  cursor appears only in reject diagnostics, never as selection.
- `family_selector.rs` — the S2 selector now takes
  `WholeUnitLoopCoverageProofV1` as a second argument. New algebra:
  foreign coverage identity -> `Rejected(CoverageIdentityMismatch)`;
  `2+` candidates -> `Rejected(Overlap)`; `0` candidates with a backed
  row -> `Rejected(CoverageBackedWithoutCandidate)`; `0` candidates
  with a fully pre-effect-declined set -> `NoCandidate(proof)`;
  `1` candidate -> `Selected` (retains the proof on
  `CanonicalLoopFamilySelectionV1::unit_coverage`). The stale
  `Unresolved(OutOfWindow)` arm and `OutOfWindow` reason were removed
  because a window without a coverage proof can no longer reach the
  selector — every call resolves through the sealed proof.
- Forward correspondence note: the selector does not re-check that a
  selected family tag matches the backed route row. No authoritative
  family-to-route map exists (five tags vs 19 routes, `GenericG0`
  family has no attested route) and inventing one would make route
  IDs semantic selection authority.
- `generic_g0_demand.rs` — destructures the five-field
  `into_parts()` and ignores `unit_coverage`; G0 demand semantics
  unchanged.
- `producer_id_migration_tests.rs` — parity receipt extended to all
  19 routes: 7 `portable_producer`, 1 `portable_v2_producer`
  (`ScanWithInit`), 11 `legacy_only` (including `GenericLoopV0`); a
  canonical-order exhaustiveness test and a `GenericG0`-absence test
  added; roundtrip list extended to the four newest producer ids.

Cursor 0/4/7 attestation gate (verified at slice time, recorded not
re-derived): `loop_break_plan_subset_min.hako` matches the S6B
`VariableAccumBreakV1` bounded profile (accumulator + predicate
`break` + induction step); `loop_simple_while_inline_explicit_step_
min.hako` matches S6A `VariableAccumRecurrenceV1` (accumulator
recurrence + induction step); `scan_with_init_typed_ok_min.hako`
matches the S6C V2 forward ScanWithInit cohort (scan return +
`-1` fallback). Correspondence held for all three rows.

Focused evidence:

```text
all_route_observation     11/11
loop_route_policy         91/91  (includes family_selector + generic_g0)
producer_id_migration      4/4
loop_recipe              224/224
producer                  55/55
generic_residual          18/18
pointer guard              ok
```

Pre-existing baseline debt repaired in this slice (both reproduced on
unmodified parent files, fixed as mechanical test repairs only):

- `policy_evidence::tests::evidence_vocabulary_is_closed_and_round_
  trips_each_disposition` — `03a39d9bd3` inserted a decline row at
  index 2 without shifting the assertion indices; indices shifted.
- `source_bound_core_tests::source_bound_core_rejects_derived_
  carrier_and_duplicate_effect_mismatch` — `df45146e48` widened
  `is_loop_statement_site` to accept `Body(_)` so the fixture's
  `Body(0)` site stopped being anchor-empty; the non-loop site is now
  `IfThen(0)`, preserving the test's `SourceBoundDerivedAnchorEmpty`
  intent.

Non-claims held: caller-zero only; no production caller or
`route_loop` connection; no second selector; no route/cursor semantic
selection; no `Option`/retry/fallback; no M9 parity; no Row F unblock
by itself (M10 seal series still gates `LOOP-PRODUCTION-SELECTION-
D0`); no corpus re-census; `parity_gate` stays `not-run`; no legacy
deletion; no `GenericLoopV0`/`GenericG0` provenance minting.
