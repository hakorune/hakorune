---
Status: open__gap_resolved_by_d0_boundary__p1_resumes__2026-09-24
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
- **Corpus boundary precision**: the sole accepted `acc+=i`-shape
  corpus fixture (`loop_simple_while_inline_explicit_step_min.hako`)
  resolves to `callable-loop` -> `CallableSingleLoopV1` (Installed
  mode, callable ledger). The Compatibility-mode reach of that shape
  (`route_loop -> generic_loop_v1` today) is outside the corpus
  census; post-switch it Freezes by the recorded boundary. Residual
  regression risk on non-corpus Compatibility programs is real but
  accepted by D0 and checked by the R0 corpus/backend parity gate.
- **Function-level arms are only 3**: `CanonicalLoopFamilyPlanV1` has
  DirectAccum/NestedPredicate/GenericG0 arms only — LoopTrue/LoopCond
  window families have no function-level production arm. At node
  level their caller-zero producers gain a consumer only through the
  P1 spine.
- **No documented growth path**: no doc plans a sixth
  `LoopFamilyTagV1`, `GenericG0PolicyProfileV1::G1+`, or a new family
  arm; S6E/S6G cards explicitly deny window widening. The five
  families are the complete selectable domain at node level.
