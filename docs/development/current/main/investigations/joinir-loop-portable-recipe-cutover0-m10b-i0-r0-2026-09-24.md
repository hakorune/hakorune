---
Status: landed__6e88441c0b__m11_residual_shadow_enumeration_open
Task: M10b-I0-R0 (JOINIR-LOOP-PORTABLE-RECIPE-CUTOVER0-I0-R0)
Date: 2026-09-24
Parent: GENERIC-M10B-DELETION-MANIFEST-S0 (landed, manifest frozen)
PreviousCard: generic-m10b-deletion-manifest-s0-2026-09-24.md
Contract source: `joinir-loop-selfhost-recipe-pipeline-ssot.md` M10b row —
  "Switch `route_loop` to frozen source -> StructuralFacts -> one policy
  winner -> verified recipe -> one candidate physicalizer." Deletion
  authority: `design/fixtures/generic-m10b-deletion-manifest-v1.tsv`
  (44 rows, checked at S0).
---

# M10b-I0-R0 — atomic portable-recipe cutover design

## Current prerequisite — accepted VAR correction (2026-09-24)

P1/P2-E are landed; R0 landed at `6e88441c0b` after closing
[M10b-I0-R0-VAR](mir-call-variable-accum-recurrence-production-i0-2026-09-24.md):
the accepted recurrence is preserved via its existing callable source owner
(`b579959a3d`), and stale guard pins were re-aligned to the post-flip
canonical callers at `a97f564250`.
The old CallableSingleLoopV1 attribution and blanket VAR exclusion below
are superseded. No sixth node family is selected; VAR acceptance gated R0
and is green. A 2026-09-25 source-shape audit found direct overlap and forced
producer/admission negatives unconstructible without synthetic authority; keep
their fail-fast branches and do not manufacture those fixtures.

## Six-line brief

```text
Decision: the M10b cutover lands as bounded caller-zero construction
  slices (P1 winner/recipe spine, P2 physicalizer edge) followed by the
  single atomic flip commit (R0) — `route_loop` becomes frozen located
  source -> StructuralFacts -> five-row family window + unit coverage ->
  `select_canonical_loop_family_v1` (exactly one) -> family Recipe
  demand -> family producer -> `VerifiedLoopRecipe` -> one canonical
  physicalizer edge -> publish, with every failure a terminal typed
  Freeze and whole-candidate discard.
Source authority + canonical issuer: the loop node's located source
  (`RawInvocationSourceContextV1` through the route-neutral
  `joinir/structural_port.rs` seam), `CanonicalLoopFacts` issuance,
  `assemble_loop_family_admission_window_v1` +
  `issue_whole_unit_loop_coverage_proof_v1`, `select_canonical_loop_family_v1`,
  the per-family Recipe demands and producers, and the canonical
  CFG/Binding-SSA physicalizer chain (`loop_recipe_physicalizer`).
Non-authority: the registry ordered schedule (`select_recipe_first_routes`,
  `observe_all_route_preflight_v1`, `try_execute_if_allowed`, handlers,
  `ENTRIES`/`dispatch_entry`), `VerifiedSelectedLoopRecipeDemandV1`
  (legacy 19-route unified demand shape), `LoopRouteKind` six-kind
  admission, `planner_reject_detail`, `Ok(None)` route tail, CorePlan
  composers, and any retry/`Option`-skip/re-decision edge.
  `evaluate_frozen_loop_route_schedule_v1`/`VerifiedLoopPolicyWinnerV1`
  are retained demand-level provenance inside the per-family demand
  issuers — not the switch winner and not deleted.
Fail-fast boundary: zero or two `Selected` family candidates, a missing
  lease/facts/coverage/window co-seal, a Recipe/JoinSig/effect-relation
  failure, or a physicalizer reject is one terminal typed Freeze —
  never `Ok(None)`, never string-flattened, never retried.
Smallest next slice: `M10b-I0-P1` — the caller-zero node-level
  winner+recipe spine (source -> facts -> window -> selector -> demand
  -> producer -> `VerifiedLoopRecipe`) plus the unified typed terminal
  vocabulary, no Builder mutation, no production caller.
Non-claims: no production switch in this card; no deletion before the
  atomic commit; no GenericResidual activation; no Main0/callable
  ingress change; no `.hako` wire change; no repetition of landed
  G0/S6/S7 proofs; no new accepted-shape claim beyond the frozen
  corpus boundary.
```

## Topology census (verified against live source, 2026-09-24)

### `route_loop` — one caller, three feeders, per-node granularity

- Sole production caller: `control_flow/joinir/routing.rs:552` inside
  `cf_loop_joinir_impl` (`routing.rs:419-564`), reached from
  `lower_loop_or_freeze_v1` (`routing.rs:269-305`), which already turns
  `Ok(None)` into a freeze string — typed-freeze wrapper exists, the
  *typed* part does not (string freeze today).
- Feeders into `lower_loop_or_freeze_v1`:
  - `raw_loop_child_port.rs:34-49` — `RawLegacyChildLoweringPortV1`
    (record values, raw legacy drives).
  - `raw_loop_child_entry.rs:607-613` —
    `lower_non_callable_loop_legacy_v1`, entered when
    `issue_callable_loop_binding_schedule_v1` returns `None` —
    which happens iff `callable_ledger` is absent
    (`recursive_child_lowering_loop_true.rs:4-30`).
  - Nested loops re-enter `statement_surface` via PlanLowerer's raw
    child port and arrive at the same `route_loop`.
- `LoopRouteContext` is AST-bearing; the AST-free seam exists as
  `joinir/structural_port.rs` (route-neutral structural view,
  caller-zero today).

### Mode split — which compiles reach `route_loop`

- `program_root_lowering.rs` — `NormalCallableSemanticPackageMode` is
  `Installed(package_port)` | `Compatibility(...)`. Main0 selection
  (`select_main0_root_product_v1`, `:371`) runs only in the Installed
  branch; Compatibility opens the wrapper unconditionally.
- Installed mode: the port gets `callable_loop_root_scope` + ledger;
  every loop gets `Some(disposition)` — `Ready` -> callable source-facts
  path, `Outside` -> terminal. `route_loop` is unreachable for
  ledger-equipped functions.
- Compatibility mode (`semantic_package.take() == None`,
  `normal_default_root_catalog_lifecycle.rs:486-522`): port is
  `RawInvocationChildPortV1::new_with_cleanup_exit_policy` — no ledger,
  `active_source` still present. Every loop -> `callable_handoff =
  None` -> `lower_non_callable_loop_legacy_v1` -> `route_loop`.
- **Probe evidence**: `static box Main { main() { local i=0; local
  acc=0; loop(i<4){acc=acc+i;i=i+1} print(acc); return acc } }`
  compiled by `./target/debug/hakorune --backend vm` runs
  **Compatibility mode** — trace shows
  `loop_legacy_selected route=generic_loop_v1` (and the dev parity
  duplicate `loop_cond_break_continue`). `route_loop` is the live
  production owner for ordinary corpus loops today.

### Canonical function-level ingress is separate, not upstream

- `compile_resolved`/`compile_resolved_first_family` (canonical
  preflight -> `CanonicalLoopFamilyPlanV1` -> source-bound compile) is
  an explicit ingress chosen by the caller; it does not run "before"
  `route_loop` in a normal compile. The two worlds coexist:
  canonical-ingress functions never reach `route_loop`; every other
  loop does.

## Machinery census

### Exists (mostly caller-zero)

- Frozen source: `RawInvocationSourceContextV1` located source is
  present at node level in both modes; `structural_port.rs` is the
  route-neutral view seam; `CallableLoopStructuralLeaseRejectV1` is a
  typed reject precedent.
- StructuralFacts: `try_build_outcome` produces `PlanBuildOutcome` with
  `CanonicalLoopFacts` — the facts step exists but is consumed by the
  scheduler today.
- Winner chain (all caller-zero): five per-family observation modules
  (`loop_route_policy/*_observation.rs`),
  `assemble_loop_family_admission_window_v1`,
  `issue_whole_unit_loop_coverage_proof_v1`,
  `select_canonical_loop_family_v1` -> `CanonicalLoopFamilySelectionV1`
  (Selected | typed NoCandidate/overlap). Lease issuer:
  `VerifiedResolvedFunctionV1::issue_loop_family_window_lease_v1`
  (`resolved_semantics/loop_family_window.rs:41`).
- Recipe producers: the S6 cohort (`accum_direct`, `recurrence`,
  `scan_with_init_v2`, `loop_cond_break_continue`, `loop_true_break_continue`,
  `nested_predicate`, `generic_residual`) plus the production G0 chain
  — per-family `VerifiedLoopRecipe` issuance exists; the Generic
  dedicated demand (`VerifiedGenericRecipeDemandV1`) is the precedent
  for family demands.
- Typed dispositions: `normal_callable_loop_source_facts/generic/
  issuer.rs` already returns `FactsAbsent | SourceUnavailable |
  FactsRejected | RouteNotFrontSelected | LoopCondReady | LoopTrueReady
  | Ready` — the decline vocabulary shape exists.
- Physicalizer: `resolved_lowering/loop_recipe_physicalizer` facade is
  the canonical CFG/Binding-SSA chain (own emitters, no PlanLowerer);
  production-connected at function-draft level (CallableSingleLoop,
  GenericG0). The five callable source arms
  (`raw_loop_child_entry.rs:312-502`) prove node-level physicalization
  via dedicated source lowerers + `PlanVerifier` + `PlanLowerer` —
  but they are ledger-bound and feed through delete-set
  `generic_loop_context.rs` (manifest A11).

### Missing (the construction gap)

- Node-level lease/window binding for non-callable contexts:
  `issue_loop_family_window_lease_v1` needs `VerifiedResolvedFunctionV1`;
  wiring Compatibility-mode located sources to the lease is unbuilt.
- Node-level winner integration: nothing feeds per-loop facts+lease
  into the window assembler at node level.
- Node-level Recipe demands for the in-boundary families and the
  demand -> producer wiring at node granularity.
- Node-level canonical physicalizer entry: the facade lowerers are
  function-draft-scoped; a loop-node entry (or a proven-equivalent
  canonical edge) does not exist.
- The unified typed terminal: `raw_loop_child_entry.rs:470-488`
  flattens issuer dispositions to strings; `router.rs:363-388` returns
  `Ok(None)`.
- Test-surface: ~15 registry test files + `builder.rs:448-559`
  test-only bridges call scheduler symbols — they die in the atomic
  commit by necessity, recorded here so the diff plan includes them.

### Explicitly excluded authorities

- `VerifiedSelectedLoopRecipeDemandV1` — the legacy 19-route demand;
  the SSOT excludes it ("the legacy selected demand ... explicitly
  excluded").
- `evaluate_frozen_loop_route_schedule_v1` /
  `VerifiedLoopPolicyWinnerV1` — legacy-cursor winner; must not become
  the post-cutover authority or the registry survives renamed.
- `LoopRouteKind` six-kind admission — dead weight after the switch;
  only Unknown-style facts-level rejects may be preserved.

## Switch body (fixed by this card)

```text
route_loop(builder, ctx):
  1. frozen source      := structural-port view of the loop site
                           (typed CallableLoopStructuralLeaseRejectV1
                           class on mismatch)
  2. StructuralFacts    := canonical facts issuance (AST-free;
                           typed FactsAbsent/SourceUnavailable/
                           FactsRejected)
  3. family window      := per-family observation rows ->
                           assemble_loop_family_admission_window_v1
                           + issue_whole_unit_loop_coverage_proof_v1
  4. one policy winner  := select_canonical_loop_family_v1
                           (exactly one Selected; NoCandidate/overlap
                           -> typed decline -> terminal Freeze)
  5. verified recipe    := winner.family() Recipe demand -> family
                           producer -> VerifiedLoopRecipe + JoinSig +
                           BindingRef/key effect relation; verify
  6. one physicalizer   := canonical CFG/Binding-SSA physicalizer edge
                           for the loop node (P2 fixes the exact owner;
                           candidates: loop_recipe_physicalizer node
                           entry vs canonical successor of the five-arm
                           pattern — decided in P1/P2, recorded here:
                           exactly one survives the flip)
  7. publish/commit     := caller-owned session semantics unchanged;
                           no partial publication
```

`lower_loop_or_freeze_v1` keeps its name/role as the typed-freeze
wrapper; its `Ok(None)` conversion becomes a real typed terminal.
`RawLegacyChildLoweringPortV1` and `lower_non_callable_loop_legacy_v1`
feed the same new body — there is no second route.

## Bounded decomposition under this card

The atomic flip cannot split (two winner authorities between commits
violates one-authority). Construction before the flip is caller-zero
and bounded:

| Slice | Scope | Production edge |
|---|---|---|
| `M10b-I0-P1` | Node-level winner+recipe spine: located source -> facts -> window+coverage -> `select_canonical_loop_family_v1` -> family demand -> producer -> `VerifiedLoopRecipe`; unified typed terminal vocabulary. Covers the in-boundary families; records per-family demand/issuer names. | none (caller-zero) |
| `M10b-I0-P2` | Node-level canonical physicalizer edge for the in-boundary families (one owner), re-feeding what `generic_loop_context.rs` + the five arms did, without the ledger dependency. | none (caller-zero) |
| `M10b-I0-R0` | Atomic flip: `route_loop` body per the fixed order; feeder rewiring; `Ok(None)` tail -> typed Freeze; manifest's 44 rows + test-only bridges + registry test files deleted in the same commit; old-symbol census zero. | THE switch |

P1/P2 land as normal bounded slices (own commits, focused gates);
R0 is the single atomic commit. If P1 reveals a family whose
node-level demand/producer cannot issue within the boundary, the card
returns to design — it does not grow a profile adapter.

## Contract, Done, Stop (from the SSOT row, binding)

- Contract: meaning once, physical allocation once, external commit
  once. Failure is terminal `Freeze` + whole-candidate discard.
- Done (preview, all must be green at flip): Rust/selfhost recipe
  parity, winner equivalence, five-adapter fault injection (the five
  `raw_loop_child_entry` source arms — `raw_loop_child_entry.rs:312-502`
  — are the injection surface), fresh reuse, accepted corpus/backend
  parity, representative phase29bq smokes, quick/release, shared
  guards, old-symbol census; exactly one production verified-recipe
  consumer, exactly one canonical CFG/Binding-SSA physicalizer, zero
  ordered-scheduler callers, zero selected composer/PHI edges, zero
  Retry/fallback; `LoopPhiMaterializerV1` caller-zero or retired.
- Stop: retry/fallback, source redecision, unverified lower, partial
  publish, diagnostics drift, backend mismatch, or an unclassified
  legacy Generic fixture blocks cutover. Every currently accepted
  fixture must already have an implemented portable owner or an
  accepted explicit typed reject.

## Evidence notes

- Probe: `/tmp` probe program (above) through debug `hakorune
  --backend vm` emitted `loop_legacy_selected route=generic_loop_v1`,
  proving Compatibility-mode main-body loops execute through
  `route_loop` + the ordered scheduler today.
- The unarmed nested callable-loop fixture now supplies the exact original
  resolver input and pins its source-owner terminal in both default and strict
  planner modes: `raw_loop_child_port::tests::unarmed_nested_loop_stops_at_source_facts_without_compatibility_reentry`
  passes 1/1. It expects `facts-absent` / `facts-rejected` respectively and
  rejects compatibility-route re-entry.
- Caller-zero checks (`rg`): `select_canonical_loop_family_v1` and the
  window/coverage issuers have no production caller; `route_loop` has
  exactly one; `collect_candidates` is caller-zero outside registry.
- The registry test-file surface (~15 files) plus
  `builder.rs:448-559` test bridges are compile-blocking on scheduler
  removal and are therefore part of the atomic diff, recorded here so
  the flip commit plans them explicitly.

## P1 recon notes (recorded 2026-09-24, before spine construction)

### Verified inventory

- Five per-family `Verified*SourceAttemptV1` issuers exist ONLY as
  `#![cfg(test)]` adapters in `src/mir/compiler/{direct_accum,nested_
  predicate,loop_true_break_continue,loop_cond_break_continue,
  generic_g0}_observation.rs` — each wraps a production projector
  (`issue_*_facts_from_source_v1`, `pub(crate)`, not test-gated) and
  maps its reject enums to `Declined`/`Unresolved`/`Rejected` attempt
  outcomes. P1 promotes (or production-twins) these five.
- `select_canonical_loop_family_v1(window, unit_coverage)` at
  `loop_route_policy/family_selector.rs:147` — outcome =
  `Selected | NoCandidate(all declined) | Rejected(Overlap |
  CoverageIdentityMismatch | CoverageBackedWithoutCandidate)`.
- `issue_all_route_observation_set_v1` (`all_route_observation.rs:138`)
  seals exactly 19 canonical-order rows; **at most one**
  `RecipeBacked` row (`MultipleRecipeBacked` reject). The caller must
  produce all 19 route outcomes — no real-source driver exists yet;
  S6G tests hand-build rows.
- `ATTESTED_RECIPE_BACKED_V1` (same file) maps 8 routes to producers:
  `LoopBreakRecipe`->VariableAccumBreakV1, `LoopSimpleWhile`->
  VariableAccumRecurrenceV1, `ScanWithInit`->ScanWithInitV2,
  `AccumConstLoop`->DirectAccumV1, `NestedLoopMinimal`->
  NestedPredicateV1, `LoopTrueBreakContinue`->LoopTrueBreakContinueV1,
  `LoopCondBreakContinue`->LoopCondBreakContinueV1, `GenericLoopV1`->
  GenericResidualV1.

### Open questions P1 must resolve (bounded)

- **Family<->route correspondence**: RESOLVED empirically (see "P1
  empirical probe results" below) — **none**: the five family arms
  admit only their own canonical profiles; `LoopBreakRecipe`,
  `LoopSimpleWhile`, `ScanWithInit`, and `GenericLoopV1`->residual are
  family-less. This reopens the winner-coverage design question before
  spine construction (see Findings).
- **Route-row production**: for each loop site, the 19 route rows must
  be produced truthfully — `RecipeBacked` only for the row the
  selected family owns (per the correspondence table),
  `PreEffectDeclined(reason)` for all others. The route-set and the
  family window independently detect overlap; they must never
  disagree.
- **Per-family node demand**: the winner `Selected(family)` -> family
  Recipe demand -> producer. The Generic demand precedent is
  `VerifiedGenericRecipeDemandV1` (function-level); node-level demand
  types for DirectAccum/NestedPredicate/LoopTrue/LoopCond are the
  construction surface. `VerifiedSelectedLoopRecipeDemandV1`
  (19-route) stays excluded.
- **Builder-side bridge**: `VerifiedResolvedFunctionV1` /
  `ResolvedFunctionLoweringInputV1` reach the node level today only
  through the callable ledger (`normal_callable_semantic_lowering_
  state`, `normal_callable_dynamic_source.rs:346`). In Compatibility
  mode there is no ledger; `resolved_binding_state`
  (`function_lowering_state.rs:190`) is the candidate install point.
  Whether P1's spine runs at compiler level (against
  `ResolvedFunctionLoweringInputV1`, then bridged) or directly in the
  builder is fixed when the first spine test is written — record the
  choice in this card.
- **`LoopRoutePolicySourceDeclineReasonV1` vocabulary**: confirm the
  per-route decline reasons cover "family owns another route" and
  "no portable owner" without inventing new reason kinds.

### P1 Done boundary (bounded)

- Spine function(s) caller-zero: located loop site + resolved
  function -> lease -> five attempts -> five rows -> window ->
  coverage -> selector -> `Selected(family)`/typed decline -> family
  demand -> producer -> `VerifiedLoopRecipe`.
- The family<->route correspondence table recorded + enforced.
- Focused positive (per attested fixture family) and negative
  (foreign lease, missing row, dual candidate, unsealed mode) tests.
- No `route_loop` edit, no production caller, no manifest deletion —
  all inside R0.

## P1 empirical probe results (caller-zero census, 2026-09-24)

Probe: `src/mir/compiler/loop_family_window_probe_tests.rs` — resolves
each representative fixture via `VerifiedResolvedSourceUnitV1`, locates
the top-level loop, reissues `VerifiedResolvedLoopSourceV1` per arm
(non-Clone), and runs all five `*_for_test` attempt issuers in
`Release` mode + `Complete` coverage (`NumericTarget::host()` for G0).
Diagnostic only; no production caller.

### Measured family<->route mapping

| Attested route / fixture | DirectAccum | NestedPred | LoopTrue | LoopCond | GenericG0 |
|---|---|---|---|---|---|
| `AccumConstLoop` (.hako `accum()`) | **Candidate** | Declined | Declined | Declined | Declined |
| `NestedLoopMinimal` (`nested_function()` AST) | Declined | **Candidate** | Declined | Declined | Declined |
| `LoopTrueBreakContinue` (.hako) | Declined | Declined | **Candidate** | Declined | Declined |
| `LoopCondBreakContinue` (.hako + `positive_function()` AST) | Declined | Declined | Declined | **Candidate** | Declined |
| `generic_g0` TYPED nested fn (`loop_route_policy/generic_g0_observation_tests.rs` fixture) | Unresolved(SourceNavigation) | Declined | Declined | Declined | **Candidate** |
| `LoopSimpleWhile`->`VariableAccumRecurrence` (`acc+=i`) | Declined | Declined | Declined | Declined | Declined |
| `LoopBreakRecipe`->`VariableAccumBreak` | Declined | Declined | Declined | Declined | Declined |
| `ScanWithInit`->`ScanWithInitV2` | Declined | Declined | Declined | Declined | Declined |
| `GenericLoopV1`->`GenericResidual` (`.hako` + `positive_function()` AST) | Declined | Declined | Declined | Declined | Declined |

Probe artifact note: the `.hako` transcription of `nested_function()`
(`local j` with no initializer) declined on all arms while the
canonical AST fixture produced `NestedPredicate Candidate` — the
parser materializes a different shape than the AST builder. Canonical
AST fixtures are the authoritative probe input.

### Findings

- All five families admit their own canonical profiles — the window
  arms work end-to-end from `ResolvedFunctionLoweringInputV1` +
  located stmt + reissued `VerifiedResolvedLoopSourceV1`.
- **4 of 8 attested portable routes have no admitting family** —
  `VariableAccumRecurrence`, `VariableAccumBreak`, `ScanWithInit`,
  `GenericResidual`. Producers and wire parity exist (S7B), but no
  family observation accepts their shapes.
- **Resolution: this is the recorded D0 selected-boundary semantics,
  not a design hole.** `loop-production-selection-d0-2026-09-24.md`
  (landed) names the winner `select_canonical_loop_family_v1`, lists
  all 8 attested producers as "caller-zero coverage artifacts and
  parked families — none selected", and states "Loop sources outside
  the selected profiles hit the existing typed-decline outcomes and
  `Freeze` — terminal failure, not fallback" (lines 115-153). All 40
  `portable-owner` corpus rows resolve to selected profiles; no
  accepted row needs a family-less route. The probe CONFIRMS the
  boundary rather than contradicting it.
- Selector tripwire confirmed by contract: with 0 family candidates,
  `select_canonical_loop_family_v1` returns `NoCandidate` only when
  the whole-unit route set is `all_pre_effect_declined`; if any route
  row is `RecipeBacked` (as these four routes would mark), the outcome
  is `Rejected(CoverageBackedWithoutCandidate)` — typed terminal, not
  silent. For a family-less loop shape the honest route row is backed,
  so the terminal is `CoverageBackedWithoutCandidate` — which IS the
  designed Freeze for out-of-boundary profiles.
- **Non-authority correction**: `evaluate_frozen_loop_route_schedule_v1`
  + `VerifiedLoopPolicyWinnerV1` are NOT deleted legacy — they are
  retained caller-zero machinery consumed INSIDE the per-family policy
  demand issuers (`issue_generic_residual_policy_demand_v1`,
  `issue_loop_cond_break_continue_policy_demand_v1`,
  `issue_loop_true_break_continue_policy_demand_v1`,
  `issue_direct_accum_route_admission_v1`), where the winner-cursor
  check is demand-level provenance, not the switch winner. The deleted
  authorities remain the registry scheduler edges pinned in manifest
  C01 (`select_recipe_first_routes`, `observe_all_route_preflight_v1`,
  `try_execute_if_allowed`, handlers, `Ok(None)` tail). The switch
  winner stays `select_canonical_loop_family_v1` per D0.
- `Unresolved(SourceNavigation)` observed on the DirectAccum arm for
  the G0 nested fixture — a third outcome class the spine must carry
  (not Candidate, not Declined).

### Worker-audit reconciliation (read-only census, 2026-09-24)

- **"Correspondence table" scope fix (S6G anti-map rule)**: S6G
  records "no authoritative family-to-route map exists (five tags vs
  19 routes) and inventing one would make route IDs semantic selection
  authority". The P1 Done phrase "correspondence table recorded +
  enforced" therefore means ONLY the empirical admission table above
  plus per-family demand wiring (five arms, each Selected(family) ->
  its own demand issuer). Route IDs are never used to dispatch; the
  route row's `RecipeBacked` marking stays migration coverage, not
  selection.
- **Frozen-schedule demand provenance is retained, cursor-checked**:
  every landed per-family demand issuer
  (`issue_generic_residual_policy_demand_v1`,
  `issue_loop_cond_break_continue_policy_demand_v1`,
  `issue_loop_true_break_continue_policy_demand_v1`,
  `issue_direct_accum_route_admission_v1`) internally calls
  `evaluate_frozen_loop_route_schedule_v1` and requires the winner at
  its own canonical cursor (e.g. GenericLoopV1 = position 18,
  WrongWinnerCursor typed reject on overlap). This is data-only
  provenance consistent with the M12 contract ("retained route rows
  are data-only source policy"); it is not a second winner — the
  family selector already picked the family.
- **Corpus boundary correction (2026-09-24)**: the accepted
  `loop_simple_while_inline_explicit_step_min.hako` recurrence was
  incorrectly attributed to `CallableSingleLoopV1`. The existing VAR
  source projector matches its static shape; its callable production
  bridge and execution evidence remain pending under `M10b-I0-R0-VAR`.
  Preserve output 6/exit 0; do not reclassify it as typed-freeze.
- **Function-level arms are only 3**: `CanonicalLoopFamilyPlanV1` has
  DirectAccum/NestedPredicate/GenericG0 arms only — LoopTrue/LoopCond
  window families have no function-level production arm. At node
  level their caller-zero producers gain a consumer only through the
  P1 spine.
- **No documented growth path**: no doc plans a sixth
  `LoopFamilyTagV1`, `GenericG0PolicyProfileV1::G1+`, or a new family
  arm; S6E/S6G cards explicitly deny window widening. The five
  families are the complete selectable domain at node level.

## P1 implementation record (2026-09-24)

### Landed construction

- Five attempt-issuer adapters promoted: `#![cfg(test)]` removed and
  `*_for_test` renamed to `issue_*_source_attempt_v1` /
  `issue_generic_g0_source_attempt_with_window_v1` in
  `src/mir/compiler/{direct_accum,nested_predicate,loop_true_break_
  continue,loop_cond_break_continue,generic_g0}_observation.rs`; all
  34 call sites updated; module gates lifted in
  `module_registry.in.rs`.
- Production observation contexts: `*ObservationContextV1::issue`
  added to the five `loop_route_policy/*_observation.rs` modules;
  `for_test` remains `#[cfg(test)]` and delegates.
- `Verified*FamilyCandidateV1::into_parts` added to the four
  non-G0 candidates (G0 already had one) so the selected family can
  hand its sealed observation/projection to its demand issuer.
- Spine: `src/mir/compiler/loop_node_winner_spine.rs` —
  `issue_loop_node_winner_recipe_v1(input, loop_stmt, numeric)`
  performs lease -> five attempts -> five rows -> window assembly ->
  whole-unit coverage -> `select_canonical_loop_family_v1` ->
  family demand -> family producer -> `LoopNodeWinnerRecipeV1`.
  Outcome vocabulary: `Issued` / `Declined(proof)` /
  `Unresolved(stage evidence)` / `Rejected(stage evidence)`.
- `loop_route_policy/mod.rs`: the window assembler, selector,
  LoopTrue/LoopCond observation issuers, and candidate types are
  ungated from `#[cfg(test)]` — the spine is their first
  production-visible (still caller-zero) consumer.

### Per-family demand/producer wiring (verified against landings)

- DirectAccum: `Candidate(observation)` ->
  `issue_direct_accum_route_admission_v1` -> handoff ->
  `admission.into_policy_winner()` + `observation.into_parts()` ->
  `issue_selected_loop_recipe_demand_v1(winner, facts, source)` ->
  `produce_direct_accum_recipe_v1`. `VerifiedSelectedLoopRecipeDemandV1`
  is consumed ONLY by this producer; inside the family chain it is a
  demand product, not the excluded 19-route selection shape.
- NestedPredicate: `Candidate(projection)` ->
  `produce_nested_predicate_recipe_v1` (no separate demand type).
- LoopTrue: `Candidate(projection)` -> spine-built family schedule
  (`LoopTrueBreakContinue` = Candidate, others
  `SourceDeclined(ExcludedByVerifiedSingletonObservation)`) ->
  `issue_loop_true_break_continue_policy_demand_v1` ->
  `produce_loop_true_break_continue_recipe_v1`.
- LoopCond: `Candidate(projection)` ->
  `issue_loop_cond_break_continue_typed_source_map_v1(input, proj)` ->
  same schedule shape (`LoopCondBreakContinue` = Candidate) ->
  `issue_loop_cond_break_continue_policy_demand_v1` ->
  `produce_loop_cond_break_continue_recipe_v1`.
- GenericG0: `issue_generic_g0_recipe_demand_v1(selection)` consumes
  the sealed selection before `into_parts` ->
  `produce_generic_g0_recipe_v1`.

### Coverage-set construction (fixed by this implementation)

`RecipeBacked` marks only the canonical route the single family
candidate owns per the empirical table (AccumConstLoop/
NestedLoopMinimal/LoopTrueBreakContinue/LoopCondBreakContinue with
their attested producer ids); `GenericG0` marks NO route row because
S6G places the G0 profile outside the canonical route inventory.
Zero or 2+ candidates produce an all-`PreEffectDeclined` set (the
selector still returns `NoCandidate`/`Overlap` respectively).
`CoverageBackedWithoutCandidate` stays a defensive typed arm for
foreign coverage proofs, unreachable from this spine's own
construction.

### Empirical finding: GenericG0 unreachable through the node window

Every G0-admissible source shape (nested loop in a loop body) makes
the DirectAccum projector hit `SourceNavigation` — it tries
`child_expr_from_stmt(.., AssignmentTarget)` on the nested Loop
statement before any shape check. The row becomes
`Unresolved`, the window cannot assemble `Ready`, and the spine
terminates `Unresolved(WindowAssemble)`. Both a standalone
`function` and a `static box` method transcription reproduce it.
`Selected(GenericG0)` is therefore unreachable at node level today;
the G0 production path stays the function-level
`CanonicalLoopFamilyPlanV1::GenericG0` arm plus
`issue_generic_g0_recipe_demand_from_observation_v1`, which bypasses
the five-row window by design. The spine's G0 arm is retained: it is
the designed path for any future source where all five arms resolve.

### Focused tests (`loop_node_winner_spine_tests.rs`, 7 tests)

- Positive `Issued`: DirectAccum, NestedPredicate, LoopTrue,
  LoopCond canonical fixtures.
- `generic_g0_box_method_fixture` records the empirical Unresolved
  terminal; `generic_g0_standalone_function_is_typed_terminal`
  asserts it.
- `family_less_variable_accum_declines` asserts the D0-boundary
  `Declined` terminal for `acc += i`.

### Baseline red (known debt, not current-change)

`mir::compiler::loop_candidate_abort_p0::loop_effect_then_later_
failure_discards_candidate_and_reuses_live_compiler` stack-overflows
in debug; reproduced identically at `cb2adda501` (pre-P1). Classified
`known baseline debt`; the test exercises `compile_normal`, not the
caller-zero spine.

## P2 accepted design — caller-zero canonical physicalizer edge

Design authority fixed before implementation (design-boundary rule: the
physical input is co-sealed at Recipe issuance, not reassembled downstream
from owner/loop keys).

1. **Sole physical owner**: the existing canonical segment pipeline
   (`src/mir/builder/resolved_lowering/loop_recipe_physicalizer/`) is the
   only Builder-mutating edge. The node-level addition is one Builder-free
   admission issuer (compiler layer: demand -> prepared operation program
   -> prepared physical layout -> entry-seed input set -> internal-decl
   rows) plus one thin physicalize edge
   (`builder/resolved_lowering/`) that borrows `CanonicalCfgSessionV1`,
   `ResolvedSsaIdentityStateV2`, and `PhiTxn` — no second authority.

2. **Physical-ready products**: the four in-boundary producers
   (DirectAccum, NestedPredicate, LoopTrue, LoopCond) are migrated to
   issue `VerifiedLoopOperationEffectProductV1` (core: source claim +
   `VerifiedLoopRecipeBindingRelationV1` + `VerifiedLoopBindingEffectRelationV1`;
   plus per-op `LoopOperationSourceEvidenceV1`) and
   `VerifiedLoopInitializedLocalInputSourceSetV1` at Recipe issuance.
   `function: &VerifiedResolvedFunctionV1` is threaded into the producers
   to mint `BindingOriginV1::Source` decl sites and `InitializedLocal`
   relations — the resolver function is the decl/init authority; Facts are
   NOT extended (Facts never carry Recipe keys). GenericG0 is already
   physical-ready and stays function-level.

3. **Continuation**: the admission issuer mints
   `VerifiedLoopContinuationContractV1::from_after(owner,
   sig.require_after_binding(root))`; the root After resolves to
   `LoopPhysicalTargetV1::OpenRootAfter`, returned to the caller as the
   single continuation point.

4. **Bounded layout extensions** (`physical_layout.rs`):
   - `entry_key`: `LoopConditionV1::Always` resolves entry via the
     `BodyEntry` edge (today: `UnsupportedAlways`).
   - `build_loop`: Always loops skip the predicate/backedge requirement;
     the body-tail finish becomes `Backedge` when the edge exists and a
     dead-tail skip when the body ends in an If whose arms both Exit
     (no continuation reach — today `BackedgeMissing`).
   - `branch_arm_finish`: `LoopJoinEdgeRoleV1::Break` is admitted as
     `Jump { after_targets[exit.target_loop] }` via a per-build
     `after_targets` map (root -> `OpenRootAfter`).
   - Still typed-rejected (outside in-boundary scope): body-level Exit
     items, Return arms, foreign-target Continue.

5. **Declaration publication**: input rows
   `publish_declaration_exact` at the loop preheader with values adopted
   from the walk's `variable_map` (source name from the resolver binding
   record — missing lane is typed terminal). Loop-internal declarations
   (binding_relations minus input bindings, e.g. nested `j`) publish via
   a post-item emit observer in `segment_dispatcher` when the producing
   item is emitted — required by `read_entry`/`define_assignment`/PHI.

6. **Builder bridge**: the edge constructs
   `ResolvedSsaIdentityStateV2::new(input.function())`,
   `CanonicalCfgSessionV1::new()`, `PhiTxn::begin(..)` once per node;
   the walk's current block is adopted as the preheader (canonical
   `seal_block` accepts foreign blocks; predecessors derive from the real
   graph). Success returns `LoopNodeWinnerPhysicalContinuationV1
   { root_after }`; the caller resumes the walk there and writes back
   `variable_map` for declared bindings — the outer owner still performs
   the only publication.

7. **Typed terminal boundary**: every admission failure (uncovered
   vocabulary, missing decl/init relation, unsupported edge shape, seal
   mismatch, missing variable_map lane) is `Rejected`/Freeze — zero
   fallback, zero legacy-composer invocation for an issued profile.

8. **Family reach**: in-boundary = DirectAccum, NestedPredicate,
   LoopTrue, LoopCond. GenericG0 stays function-level (empirically
   unreachable through the node window — recorded above). Family-less
   routes remain `Declined`/`CoverageBackedWithoutCandidate` per the
   landed D0 selection boundary.

## P2-B landed — four producers emit physical-ready products

All four in-boundary producers now issue the co-sealed semantic product
at Recipe issuance (`&VerifiedResolvedFunctionV1` threaded as the
decl/init authority; Facts unchanged, no Recipe keys in Facts):

- `produce_direct_accum_recipe_v1(demand, function)` —
  `{policy_receipt, operations, inputs}`; inputs = `i`/`sum`
  `InitializedLocal` relations; operations = carrier seed + compare +
  two writes with per-op source evidence.
- `produce_loop_true_break_continue_recipe_v1(demand, function)` —
  inputs = `flag`; operations = const + compare + if-exit evidence.
- `produce_loop_cond_break_continue_recipe_v1(demand, function)` —
  inputs = `i`; operations = compare(predicate) + if/exit evidence.
- `produce_nested_predicate_recipe_v1(projection, function)` —
  inputs = `i`/`sum`; internal decl `j` is a binding relation without
  an input row (published at its producing item via the P2-D emit
  hook); operations = root/child seeds + compares + writes.

Shared helper `loop_recipe_contract/binding_declaration.rs`
(`resolve_loop_binding_declaration_v1`) maps a binding -> decl site +
initializer literal through `function.declaration_binding` /
`expression_source().initializer()` — no name inspection.

`VerifiedLoopCoreProductV1::into_recipe_sig()` (pub(crate)) added so
the legacy adapters (`physical_input.rs::from_direct_accum`, nested
topology/emission input) still unwrap recipe+join_sig from the
co-sealed core without exposing the private source-claim type.

Focused gate: `cargo test --lib -- direct_accum_producer_tests
loop_true_break_continue_producer_tests
loop_cond_break_continue_producer_tests
nested_predicate_producer_tests nested_predicate_topology_tests
nested_predicate_physical_input_tests nested_predicate_effect_plan_tests
nested_predicate_effect_adapter_tests` — 28/28 ok;
`wire_route_parity_tests wire_parity_tests
loop_node_winner_spine_tests loop_family_window` — 73/73 ok.
`cargo test --lib --no-run` green.

Next slice: P2-C layout extensions (Always entry/BodyEntry,
conditional backedge, dead-tail, Break-arm exits).

## P2-C landed — layout extensions for Always / break-arm / dead tail

`physical_layout.rs` now admits the in-boundary vocabulary:

- `entry_key` branches on the recipe's own `LoopConditionV1`: Predicate
  resolves the condition-block segment (unchanged); `Always` requires the
  `BodyEntry` edge (Header->Body ports) and resolves `{loop, body, 0}`.
- `build_loop` registers `after_targets[loop_key] = after_target` before
  descending, then classifies the body tail: an existing `Backedge` edge
  binds to the loop entry (condition segment for predicate loops, body
  segment for Always); a missing `Backedge` edge enters `DeadTail` mode.
- `DeadTail` is admitted only when the body's final item is an `if` whose
  arms both exit (no reachable continuation); any reachable tail without
  a backedge rejects as `BackedgeMissing` — never a synthetic transfer.
- `branch_arm_finish` admits `Break` exits as
  `Jump { after_targets[exit.target_loop] }` (root -> `OpenRootAfter`,
  nested break -> ancestor resume segment); `Continue` still requires the
  owning loop and jumps to its entry. `Return` arms and foreign-target
  continues stay typed-rejected.

Focused gate: `physical_layout` — 10/10 ok (new: Always entry via
BodyEntry, break->OpenRootAfter, continue->body entry, dead-tail skip;
negative: Return arm, reachable tail, missing BodyEntry, unvisited break
target).

Next slice: P2-D segment_dispatcher emit hook for in-loop declaration
publication (nested `j`), then P2-E admission issuer + physicalize edge.

## P2-D landed — loop-internal declaration publication hook

`loop_recipe_contract/internal_declaration.rs` derives
`VerifiedLoopInternalDeclarationSetV1` once inside
`VerifiedLoopOperationPhysicalDemandV1::issue` (root loop site from
`context.loop_site()` — resolver-issued, not re-derived from evidence).
A `Local` declaration is internal iff its statement path strictly
extends the root loop's path. Two adoption modes, both anchored to a
Recipe item:

- `Activate` — the local's first physical touch is a `WriteBinding` op;
  the dispatcher calls `activate_declaration_exact` (new identity API,
  no caller-supplied name/kind) before that item emits and the exact
  write path initializes it.
- `PublishWithEntry { value }` — the local is a `DerivedCarrierEntry`
  binding (nested `j`): its carrier `entry_value` is a Recipe-produced
  value (`local j = 0` -> item3 `ConstI64`), so the dispatcher calls
  `publish_declaration_exact` with the ledger's physical value after
  the producing item emits. This makes the carrier-seed row's
  `read_entry(j)` at the inner preheader succeed (active+initialized).

Typed rejects: `ProducerMissing`, `ProducerNotWrite`,
`CarrierMissing`, `EntryProducerMissing` (input-only entry values are
entry-set business, not internal), `DeclarationPlacementMissing`
(preflight: producing item must be a scheduled operation item).

Key correction vs the P2 sketch: "earliest evidence item" is wrong as
the adoption anchor — `j`'s earliest evidence is an inner-loop
`ReadBinding` (item5); the real anchor is the carrier `entry_value`
producer in the parent body. Internal-ness is judged against the
demand's root loop site, not the producing item's owner loop.

Focused gate: `internal_declaration` — 4/4 ok (nested `j` Local site +
segment prefix, PublishWithEntry mode + carrier entry + const producer,
per-item scoping; DirectAccum/LoopTrue/LoopCond -> empty set);
`operation_physical_demand segment_dispatcher physical_layout
operation_effect` — 26/26 ok.

Next slice: P2-E admission issuer (demand -> program -> layout + input
set + decl rows) + Builder physicalize edge (CFG/SSA/PHI + variable_map
bridge).

## P2-E landed — admission issuer + caller-zero Builder physicalize edge

`compiler/loop_node_physical_admission.rs` issues
`VerifiedLoopNodePhysicalAdmissionV1` Builder-free: the `Issued` winner's
resolver context (site + window-lease frame) is re-verified against the
function input (`resolved_loop_source_context`, site must re-resolve
identically), the root carrier's JoinSig After binding is required once
(`RootCarrierMissing`/`AfterBinding` rejects), then the sole demand ->
program -> layout chain runs plus the recipe's entry input set. The
`GenericG0` arm is `FunctionLevelFamily` — function-level only, never a
node-edge fallback. No re-selection, no second demand issuer.

`builder/resolved_lowering/loop_recipe_physicalizer/loop_node_lowerer.rs`
is the thin Builder bridge (caller-zero, `#[allow(dead_code)]`): one
`ResolvedSsaIdentityStateV2` + `CanonicalCfgSessionV1` + `PhiTxn` per
node, entry input rows published with `publish_declaration_exact` at the
walk's current block using the `variable_map` value resolved by
resolver binding `diagnostic_name` (missing name/value/binding -> typed
terminal, no fallback), then the shared
allocator -> dispatcher -> recursive-after pipeline. Returns
`LoopNodeWinnerPhysicalContinuationV1` = sealed root After + per-binding
writebacks read through the same identity (`read_entry` at root After);
the outer walk stays the only `variable_map` writer and publisher.

`IssuedLoopNodeWinnerV1` gained `into_parts()` and a `#[cfg(test)]`
`for_test` ctor for admission coverage; production issuance stays inside
the spine.

Focused gate: `loop_node_physical_admission` — 5/5 ok (DirectAccum
layout+inputs+empty decls, nested `j` PublishWithEntry row carried
through demand, LoopTrue/LoopCond layouts, G0 -> FunctionLevelFamily).
Regression: 186 family/spine/demand/layout/dispatcher tests +
25 physicalizer tests all green.

Next slice: R0 atomic `route_loop` switch — wire the named production
caller (loop stmt site -> spine -> admission -> lowerer -> writebacks +
`select_block(root_after)`), then retire the deletion-manifest legacy
path for the selected boundary.

## R0 deletion disposition record (recorded during landing)

The working diff removes 163 files vs the frozen manifest's 22 `file`
rows. Post-hoc audit classifies the 141 manifest-external deletions:

- **M11-scope, discharged early** (recorded disposition change,
  user-approved): `plan/generic_loop/located_representation/*` (9) and
  `plan/parts/associated_source/located_*` + `associated_source_tests.rs`
  (6). The new `route_loop` spine already feeds resolver-inventoried
  `SourceStmtSiteV1` (located source) into the StructuralFacts/Recipe
  path — the M11 "feed" half is discharged by the spine, and these files
  are the "covered shadow entries" M11 was chartered to retire. Verbatim
  restore is impossible without resurrecting manifest delete-set rows
  (`facts::extract`, `facts_types`), which they import.
- **retained-row, husk-equivalent**: `registry/live_preflight_frame.rs`.
  The manifest marked the file retained, but its only meaningful edge is
  the C05 `try_execute_route_execution_witness` Ok(None) continuation —
  and `execution_witness.rs` is itself a manifest file-delete row. After
  C05 the file is an empty husk; physical deletion is equivalent.
- **R1 generic-only dead files, folded into the R0 working diff**
  (~120): `plan/generic_loop/` non-located children,
  `plan/nested_loop_depth1/` children, `features/generic_loop_body/`
  subtree, registry scheduler machinery + its test files, and the
  callable generic-arm files (`normal_callable_loop_physical_adapter`,
  `generic/carrier_relation*`, `generic/source_admission*`). The SSOT
  allows physical removal in the immediately-following caller-zero R1;
  recording here keeps the manifest-vs-diff audit honest. Commit
  splitting is decided at commit time.
- **All deletions were caller-zero** (lib + tests compile green without
  them); no live caller was deleted to make the manifest green.

M11 residual scope after this record: prove the old located handoff has
zero production callers and enumerate any remaining shadow entries.

M11 residual enumeration (2026-09-25, post-`6e88441c0b` tree):

- Old located handoff — discharged: `plan/generic_loop/located_representation/*`
  and `plan/parts/associated_source/located_*` carry zero references; the
  deleted symbols (`verify_located_generic_loop_v1`, located input/body
  composers) have no residual caller.
- Shadow entry candidates — caller-zero but contract-pinned, so removal is
  the M11 row's own bounded work, not a free deletion:
  - `plan/located_loop.rs` (254) + `located_loop_error.rs` (14) +
    `located_loop_tests.rs` (594): `LocatedCoreLoopExecutionSessionV1` /
    `VerifiedLocatedCoreLoopPlanV1` are re-exported at `plan/mod.rs:170-173`
    under `#[allow(unused_imports)]` with zero production callers —
    **retired in M11-B** (this branch): files deleted, `plan/mod.rs`
    decls/re-exports removed, and the red-baseline receipt absorbed the
    ten removed unit tests (expected_passed 7379→7369).  The recorded
    contract-pin concern resolved as follows on inspection: the
    `callable_result_i0_*` family in `tools/checks/lib/` is dormant —
    `mirbuilder_inplace_replacement_guard.sh` lists the py files in
    `guard_require_files` existence checks but never executes them, and
    no dev_gate/guard_rows/CI surface runs them either; several of those
    checks were already stale post-R0 (they read the deleted
    `generic_loop/located_representation/` subtree).  Their
    located-artifact pins now join that dormant drift; any row that
    re-activates the family owns the re-point.  The live
    schedule/batch/ledger claims in `callable_result_representation/`
    are untouched — `ClaimedCallableResultLoopBatchV1` stays consumed by
    `lowerer/emission_port.rs`.
  - `plan/composer/coreloop_v2_nested_minimal.rs` +
    `composer/coreloop_gates.rs`: caller-zero (self + `composer/mod.rs`
    only) — **retired in M11-A** (this branch): files deleted,
    `composer/mod.rs` decls/re-export removed, and the two varmap role
    inventories (`coreplan_varmap_boundary_inventory_guard.sh`,
    `mir_verification_quick_p0_c_guard.py`) re-pointed to the live
    12-site inventory (raw=12 test_only=7 disconnected=0 live=5
    canonical=1 reseal=4).  The same re-point dropped the
    `generic_loop_located_composer*` / `nested_depth_observer_tests` /
    `generic_loop_whole_parity_tests` / `located_hook_tests` /
    `located_parity_tests` site entries that were already stale after the
    R0 file deletions.  Dormant pin noted for the record:
    `rust_lifecycle_mirbuilder_plan_composer_projection_policy_guard.sh`
    is unregistered (no dev_gate/guard_rows/CI surface) and its frozen
    decision fixture still names `coreloop_gates.rs`; it needs re-pointing
    only if it is ever re-activated.
  - `cargo_lib_red_baseline` receipt absorbed the two removed composer unit
    tests (expected_passed 7381→7379, inventory sha re-hashed).  The
    receipt is independently stale at HEAD — observed quick-lib run is
    7969 passed / 167 failed / 56 ignored against receipt 7381/133/29, and
    two failing tests emit multi-line `[llvm-mem2reg/error]` noise that the
    checker's single-line `FAILED` parser cannot extract, so the step stays
    red in this environment until a dedicated audit row re-baselines the
    whole suite.  No composer/coreloop test appears in the observed
    failure set.
- `facts/canon.rs` doc drift recorded earlier is moot: the file was deleted
  with the canon subtree; `facts/expr_generic_loop.rs` remains a live purity
  helper.
- Not shadow — retained production: `plan/expression_port.rs`
  `LocatedLoopPlan*` types and `LocatedLoopPlanExpressionPortV1` are consumed
  by the canonical source-backed path (`normal_callable_loop_source_port.rs`,
  `normal_callable_loop_source_facts/{composite_physical,loop_cond}.rs`,
  `plan/normalizer/cond_lowering_*`); `parts/associated_source/` non-located
  children (callable_loop_source drivers, dispatch, block_driver,
  raw_lowering) remain live.

### Caller-zero re-audit (2026-09-24, read-only, post-flip)

Independent re-verification of the current deletion set and remaining
callers. No new deletions; audit only.

- **Staged deletions = 163**, matching this record exactly. All within
  recorded categories (registry scheduler subtree, `generic_loop`
  subtrees across `facts/canon`/`plan`/`recipe_tree`/`skeletons`,
  `located_representation` M11 husk, `associated_source` located files,
  callable generic-arm, `features/generic_loop_body`, policies).
- **Unstaged deletions = 10** (in-flight caller-zero fold, not staged):
  `plan/nested_loop_plan*.rs` (6), `parts/loop_/nested_depth1.rs`,
  `cond_lowering_freshen/final_values_tests.rs`, and two callable
  source-route test files. These files were manifest *callers* of the
  A11/A18/A19 edges, not delete rows; their fold is the recorded
  caller-zero-R1 consequence. Verified caller-zero (`nested_loop_plan`,
  `NestedLoopPlan`, `nested_depth1` — zero live refs, parent decls
  already removed; lib compiles green).
- **Symbol census**: all retired symbols zero-referenced —
  `select_recipe_first_routes`, `observe_all_route_preflight_v1`,
  `try_execute_if_allowed`, `route_generic_loop_v{0,1}`,
  `pred_generic_loop_v{0,1}`, `LoopPhiMaterializerV1`,
  `lower_with_existing_route_v1`,
  `CallableGenericLoopSourceRouteAdmissionV1`,
  `verify_located_generic_loop_v1`, `GenericLoopFactsPolicyFrameV1`,
  `compose_generic_loop_v1_recipe`, `GenericLoopSkeleton`,
  `apply_nested_loop_preheader_freshness`,
  `try_extract_nested_loop_depth1_facts`,
  `lower_nested_loop_depth1_any`, `loop_legacy_selected`. Residual
  matches are retirement comments, error strings, or the intentional
  empty decl shells (`plan/generic_loop/mod.rs`,
  `plan/nested_loop_depth1/mod.rs`).
- **Retained correctly**: `registry/mod.rs` (route_id facade only),
  `registry/predicates.rs` (live caller:
  `CallableLoopRouteMatchV1::issue` — data-only exclusivity matcher for
  retained non-generic arms), `route_loop` (fixed order, no `Ok(None)`
  tail, no ENTRIES loop, no retry), `plan/facts/expr_generic_loop.rs`
  (purity helper, unrelated to deleted `facts/canon/generic_loop`).
- **Single-owner invariants hold**: `issue_loop_node_winner_recipe_v1`
  has exactly one production caller (`router.rs`); the canonical
  physicalizer is the single `loop_node_lowerer.rs`.
- **Minor doc drift (not fixed here)**: `facts/canon.rs` doc comment
  still lists `generic_loop` among Facts-owned modules though the
  module is deleted.
- **VM-gate row disposition landed (2026-09-24)**: all 179
  `phase29bq_fast_gate_cases.tsv` rows are dispositioned — 12 portable
  rows on the accept gate (plus 3 `main0-*` rows from other surfaces,
  15 total), 164 on the new typed-terminal gate, 3 held (`not-run`);
  see "Gate migration landed" below.

### Gate-migration scoping (2026-09-24, read-only probes)

The 40 `D0-DISPOSITION-CHECKED` portable-owner rows map onto gates as
follows: 15 rows live in `phase29bq_fast_gate_cases.tsv` (14
canonical-main + the callable-loop VAR pin); 12 `main0-*` fixture rows
live in the phase29ca/cb `*_release_adopt_vm.sh` / `strict_shadow_vm.sh`
scripts; the rest are selfhost-corpus/subset twins or script rows.

phase29bq portable-owner rows probed on the source-backed lane
(`--backend mir`, `NYASH_DISABLE_PLUGINS=1`, current `target/quick`
build):

- **12/15 preserve fixture+output+rc** (migrated, see below): VAR row
  prints `6` rc=0; 8 no-loop `selfhost_parse_program2_if_*` rows
  (`7`/`1`/`__EMPTY__`, rc=0); `blockexpr_basic_min` (`0`),
  `blockexpr_return_min` (`__EMPTY__`), `stageb_blockexpr_return_min`
  (`__EMPTY__`).
- **`selfhost_parse_program2_loop_if_return_local_min` — held**:
  expected `__EMPTY__` rc=0 on VM (flowbox shadow adopt); mir lane
  emits typed freeze `LoopCondRouteRejected(SourceItemsMissing)` rc=1.
  Classification flips accepted→rejected; cannot migrate while
  preserving classification.
- **`cond_truthiness_null_min` — held**: same terminal (`Type error:
  Void in boolean context`, rc=1) but the error text is lane-formatted
  (`[vm] VM error:` vs mir `[vm/error]`). Literal expected-output
  preservation is impossible; whether classification+rc preservation
  suffices is a gate-contract decision, not an implementer choice.
- **`cond_truthiness_value_min` — held, blocker class**: expected
  `12345`, mir lane emits `1X2X3XX45X`. Probes show the source-backed
  canonical-main path executes **local-variable writes inside if/else
  arms unconditionally** (both arms' writes land in source order),
  while `print`/`return` arms gate correctly (`if false { print("BAD")
  }` prints nothing; `if false { return 1 }` is skipped; `if c {
  out+="T" } else { out+="F" }` yields `TF`). The row cannot migrate
  with preserved expected output until the canonical-main if/else
  local-write gating is owned and fixed — a semantic slice outside
  this gate migration.
- **Non-portable rows**: the remaining ~140 TSV rows are
  `D0-TYPED-REJECT`/`D0-PRE-LOOP-EVIDENCE` corpus, not portable-owner.
  Loop-carrying ones now freeze on the VM compat lane (`zero family
  candidates`) — their gate disposition is a separate R0-owner
  decision and is not covered by the portable-owner migration.

### Gate migration landed (2026-09-24, narrow)

- Successor gate:
  `tools/smokes/v2/profiles/integration/joinir/phase29bq_portable_owner_source_backed_gate_mir.sh`
  + `phase29bq_portable_owner_source_backed_cases.tsv` (15 rows,
  fixture/expected/allowed_rc preserved; no planner-tag column —
  retired-machinery tags are not emitted on the source-backed lane)
  + `tools/smokes/v2/lib/joinir_source_backed_gate.sh` (hermetic
  `--backend mir` runner; explicitly neutralizes
  `HAKO_JOINIR_STRICT`/`HAKO_JOINIR_PLANNER_REQUIRED`, whose
  combination forces the retired planner admission and freezes the
  source-backed route).
- Registered in `tools/smokes/v2/suites/integration/joinir-bq.txt`.
- Verified: 15/15 PASS on `target/quick` build (full list + `--only`
  spot check).
- Coverage carried: the 12 phase29bq TSV portable-owner rows plus the
  3 `main0-*` `_min` fixtures whose release-adopt contracts (rc +
  empty output, no tags) were pinned by the unregistered
  `generic_loop_{carrier_type,continue,in_body_step}_release_adopt_vm.sh`
  scripts (all preserve on mir: rc 4/4/3, empty output).
- `phase29bq_fast_gate_cases.tsv` stays byte-identical to HEAD as the
  corpus inventory (consumed by `generic_legacy_corpus_universe_guard`,
  which validates `source_surface` line references 1:1). Its list
  execution step is retired from `phase29bq_fast_gate_vm.sh`; each
  row's live contract is named by the manifest `parity_gate` column.
- Typed-terminal successor gate:
  `tools/smokes/v2/profiles/integration/joinir/phase29bq_typed_terminal_source_backed_gate_mir.sh`
  + `phase29bq_typed_terminal_source_backed_cases.tsv` (164 rows;
  contract = nonzero exit + pinned `[authority/terminal]` marker on
  `--backend mir`). Registered in `joinir-bq.txt`; verified 164/164
  PASS on `target/quick`. These are the `D0-TYPED-REJECT` /
  `D0-PRE-LOOP-EVIDENCE` rows whose VM-lane acceptance came from the
  retired ordered-scheduler route; per-row terminal markers were
  censused before pinning.
- `generic-loop-legacy-disposition-v1.tsv`: `parity_gate` set to
  `phase29bq_typed_terminal_source_backed_gate_mir` for the 164 rows;
  the 3 held rows keep `not-run`.
- `generic_legacy_corpus_universe_guard.py`: `parity_gate` allowed set
  extended from `not-run`-only to the two named source-backed gates.
- `generic-loop-legacy-disposition-v1.tsv`: `parity_gate` set to the
  new gate name for the 27 corpus rows whose fixtures/contracts are
  now gated (12 primary + 8 `selfhost::` twins + 3 `main0-*` fixtures
  + 4 `release_adopt` script rows).
- Held on the VM gate pending named decisions: the 3 phase29bq rows
  above. The sibling `*_strict_shadow_vm.sh` scripts pin
  `HAKO_JOINIR_STRICT`+`HAKO_JOINIR_PLANNER_REQUIRED` — the retired
  planner lane itself — and are obsolete candidates (red on both lanes
  post-flip), not portable contracts. The remaining `main0-*` corpus
  fixture rows (bound2/renamed_locals/zero_iter/guard_never/
  upper_bound_*) were never gate-pinned; their parity evidence is a
  separate step.

### Landed record (2026-09-25)

- `6e88441c0b` — atomic `route_loop` flip to the frozen located-source
  pipeline plus the ordered-scheduler/registry retirement (the 163-file
  delete set audited above).
- `b579959a3d` — the VAR callable connection (M10b-I0-R0-VAR).
- `a97f564250` — stale-guard re-pin to post-flip canonical callers
  (`loop_node_winner_spine.rs`, projection/capability files), removal of
  retired-file anchors, `rg -P` removal (PCRE2 unavailable), baseline
  count corrections, and the `wire_parity_tests` 800-line split.
- Post-landing green: `cargo build --profile quick`, `cargo check --tests`,
  `cargo build --release --bin hakorune` (7m02s, warnings only),
  winner-spine 7/7, physical-admission 5/5, raw-entry 16/16, wire parity
  36/36, variable-accum 28/28; portable-owner gate 15/15, typed-terminal
  gate 164/164; in-place replacement guard, corpus guard, pointer guard,
  and source-route admission guard all pass. Old-symbol census on the
  committed tree is clean (residual matches are retirement comments or
  intentional empty decl shells). `route_loop` has exactly one caller
  (`routing.rs`), `issue_loop_node_winner_recipe_v1` has exactly one
  production caller (`router.rs`), and `LoopPhiMaterializerV1` is
  zero-referenced.
- Remaining open: the M11 residual scope recorded above (prove the old
  located handoff has zero production callers and enumerate any remaining
  shadow entries), then M12, each under its own row — not this card.
