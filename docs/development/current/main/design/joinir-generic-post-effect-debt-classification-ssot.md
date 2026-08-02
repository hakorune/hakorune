# Generic Loop post-effect debt classification

Status: active design stop

Decision: accepted — `JOINIR-GENERIC-POST-EFFECT-DEBT-CLASSIFICATION0-D0-S0`

This card is the detailed task order for the Generic V0/V1 debt named by the
Loop pipeline SSOT. It is a design and test-only boundary. It does not create a
portable Recipe producer, a PHI writer, a second scheduler, or a production
cutover.

The scoped non-Generic bridge decision is tracked separately in
`joinir-loop-scoped-nongeneric-cutover-ssot.md`: broad non-Generic-first
production cutover is rejected; only a proven singleton/disjoint pilot may be
considered after the shared M6 owners exist. D2 remains required for global
M10b.

## Authority and non-claims

The current execution authority is still:

```text
route_loop
-> live preflight
-> ordered legacy route witness
-> Generic V0/V1 handler
-> composer/CorePlan/PlanVerifier/PlanLowerer
-> legacy JoinIR/JoinModule path
```

`RecipeTree + Parts` remains the in-builder implementation and parity oracle.
`LoopRecipeArtifactV1` remains the portable Rust/.hako semantic contract. A
verified Recipe has no production physical consumer yet. The current PHI
materializers and JoinIR route are legacy execution authority until M10.

M4 may record a target disposition, but it must not issue a
`VerifiedLoopRecipeV1`, `LoopJoinSigV1`, `LoopPhiMaterializerV1`, or candidate
publish. Those belong to M5--M10 in the parent pipeline card.

## Downstream production-authority handoff map

The portable Recipe contract is the accepted semantic target, but it is not
yet the production authority. Live execution remains the legacy
`RecipeComposer -> CorePlan/PlanVerifier/PlanLowerer -> JoinIR/JoinModule`
path until the atomic M10 cutover.

| task | production meaning | required gate |
| --- | --- | --- |
| M5 Accum pilot | caller-zero end-to-end consumer of one verified Recipe; parity oracle only | recipe/verifier/JoinSig/PHI/physicalizer parity, candidate abort, fresh reuse |
| M6 shared owners | establish one new-subtree CFG/JoinSig/`LoopPhiMaterializerV1` owner | route-specific PHI/block writers have zero new-subtree callers; global production count remains zero until M10 |
| M7/M8 closure | five migration families and all 19 producers emit the same recursive Recipe | adapters cannot select, retry, allocate PHIs, or publish |
| M9 Rust/.hako parity | prove the portable producer contract across hosts | no `.hako` physicalizer or default-route authority claim |
| M10 cutover | make the verified Recipe physicalizer the single production consumer and delete the selected JoinIR fallback caller/old PHI edges atomically | winner equivalence, one terminal physicalizer, Retry/fallback/old selected callers = 0 |
| M12 retirement | reduce temporary family adapters after M10/M11 | family adapter/first-mutation/duplicate physical authorities = 0; retained rows are data-only policy inputs |

“Recipe is the SSOT” therefore means the semantic target and replacement
contract before M10; it does not claim that current production lowering already
consumes it. Wiring one family into production before M10 is out of order and
would create a second authority.

## Post-M4 production-authority handoff — taskized

The clean production connection is deliberately staged. The following is a
handoff plan, not a claim that any step is already wired:

1. **M5 — one caller-zero vertical pilot.** Use `AccumConstLoop` to consume one
   verified portable Recipe inside the existing unpublished compile candidate.
   The pilot must exercise Recipe verification, JoinSig elaboration, the new
   PHI/materialization seam, terminal physicalization, candidate discard, fresh
   reuse, and MIR parity. It must not become a `route_loop` caller or a second
   production scheduler.
2. **M6 — one Recipe-only physicalization owner.** Establish the shared
   `LoopCfgSkeletonLoweringV1`, `LoopJoinSigV1`, and
   `LoopPhiMaterializerV1` services. The PHI service consumes verified JoinSig
   plus logical-to-physical mapping only; legacy repair and route-specific PHI
   writers remain explicitly outside this owner. New-subtree duplicate writers
   must be zero, while the global production consumer count remains zero.
3. **M7/M8/M9 — semantic closure before wiring.** Move the five migration
   families, all 19 producer rows, and the `.hako` producer onto the same
   recursive Recipe contract. Adapters remain data-producing migration seams;
   they cannot select, retry, allocate PHIs, or publish.
4. **M10 — first production consumer and one atomic deletion.** Switch
   `route_loop` to the frozen facts → one policy winner → verified Recipe → one
   candidate physicalizer path. In the same commit, delete the selected old
   JoinIR/JoinModule fallback caller and its selected legacy PHI/Retry edges.
   This is the first point at which Recipe becomes the execution authority;
   before it, the old path remains the honest authority.
5. **M11 — located-source handoff.** Feed located Loop provenance into the same
   Recipe path and remove the source-erasing handoff. No new producer or
   fallback is allowed.
6. **M12 — adapter retirement.** After M10/M11, delete migration-only family
   wrappers and duplicate physical authorities. Retain only source-policy rows
   that produce the common recursive Recipe. A remaining adapter is not “done”
   while it still owns allocation, PHI repair, retry, AST rematch, or route
   selection.

The handoff gates are therefore: **M5 caller-zero**, **M6 shared Recipe-only
PHI owner**, **M10 exactly one production consumer plus one same-commit old
fallback deletion**, and **M12 zero duplicate family physical authorities**.
Until those gates are green, the statements “Recipe is the execution SSOT”,
“PHI has one global writer”, or “JoinIR is historical only” are prohibited.

## Closed vocabulary for the census

Every observed Generic path is assigned exactly one disposition, with evidence
for the first Builder effect:

| disposition | meaning | allowed continuation |
| --- | --- | --- |
| `PreEffectDeclined` | facts or policy do not select Generic and no Builder effect occurred | legacy suffix may be observed only by the test oracle |
| `PreEffectBlocked` | a source/policy precondition is unavailable before mutation, but no winner equivalence is proven | unresolved until D3; never silently relabel as decline |
| `TerminalFreezeTarget` | composer/plan/lower work has effected the candidate; retry would reuse dirty state | future candidate abort/Freeze; no next route |
| `ImpossibleEdge` | a closed invariant proves the branch cannot occur for a valid selected row | fail-fast if the invariant is violated |
| `UnresolvedStop` | evidence is insufficient to choose one of the above | M4 remains stopped |

`LegacyComposerResultReceiptV1` remains diagnostic migration evidence only. It
is not imported into the pure policy evaluator and is not a semantic Recipe
field.

## Current D0-S0 execution brief — stage matrix contract

### Change

Fix the Generic V0/V1 stage-matrix contract before adding a test bridge. The
matrix has one row for each V0/V1 × mode × contract arm: facts absent or
mismatched, composer precondition/allocation/`Err`, strict shadow
`Some`/`None`/`Err`, release verifier `Ok`/`Err`, release lower
`Some`/`Ok(None)`/`Err`, nested Generic calls, and legacy receipts. Each row
records a source anchor, first-effect owner, legacy outcome, receipt (if any),
and evidence level `Observed`, `NotYetObserved`, or `UnresolvedStop`.

### Contract

This is docs-only. It fixes the meaning of `PreEffectDeclined`,
`PreEffectBlocked`, `TerminalFreezeTarget`, `ImpossibleEdge`, and
`UnresolvedStop`. It does not change code, fixtures, scheduler, policy,
Recipe, JoinSig, PHI, physicalization, or candidate publication. The existing
M3-F synthetic non-Generic fixture is explicitly excluded from Generic
evidence.

### Done

Every current Generic branch has a matrix row with an owner, legacy outcome,
receipt or explicit unresolved marker. An unobserved effect boundary is never
labelled decline. The matrix is sufficient input for the next test-only slice:
actual AST → facts → selector → `V0-only`/`V1-only`/`Both`/`Neither` fixtures.

### Stop

Do not infer first effect, V0/V1 precedence, or winner from route names,
comments, or a static effect matrix. If closing an `UnresolvedStop` would
require touching a Generic handler, `all_route_preflight`, or an M5/M6/M10
owner, stop and return to the design boundary.

Gate: `git diff --check`,
`bash tools/checks/current_state_pointer_guard.sh`,
`bash tools/checks/lib/joinir_logical_demand_contract.sh`, and this card below
800 lines. The docs-only diff is limited to this card and current-state mirrors.

### D0-A1 — source-to-selection observation (test-only)

The first test-only slice now consumes the actual facts builder and
`select_recipe_first_routes`. It stops before `RouteExecutionWitness`, Generic
handlers, composers, verifier, and lowerer. The observed release fixtures are:

| fixture | facts | raw selection | evidence boundary |
| --- | --- | --- | --- |
| simple while | V0 + V1 | `LoopSimpleWhile, GenericLoopV0` | V0 is an unreached tail, not a V0-only winner |
| local plus step | V1 only | `GenericLoopV1` | source-to-selection only |
| nested loop plus step | V0 + V1 | `GenericLoopV0, GenericLoopV1` | Both is real; precedence remains unresolved |
| unsupported import plus step | neither | empty | no facts/no schedule |

The planner-required fixture additionally records that V0 extraction is
suppressed before selection. These tests are observation evidence, not a
winner decision. Strict-mode and post-effect handler stages remain
`NotYetObserved` or `UnresolvedStop` until the next slice.

### Current D3 execution brief

Change:
: Add a `#[cfg(test)]` observer that feeds the A1 `Both` input through the real
  facts/selection result, `RouteExecutionWitnessV1`, `ENTRIES`, and Generic
  handlers, then records the attempted prefix, receipt stage, and terminal.

Contract:
: The existing witness scheduler and handlers remain the only execution
  authority. The observer is test-only; it does not alter policy, Recipe,
  JoinSig, PHI, candidate publication, or `all_route_preflight`.

Done:
: The trace proves the raw `[GenericLoopV0, GenericLoopV1]` schedule reaches
  the real handler path and records success, outer error, or an actual
  `LegacyComposerResultReceiptV1`. A debt-to-later-winner trace is compared
  with the current pure-policy disposition; absence of such a trace remains
  explicit evidence, not a fabricated proof.

The bounded mode extension now runs the same observer under release, strict,
and strict+planner-required configuration. Release and strict retain the raw
`[GenericLoopV0, GenericLoopV1]` overlap and both stop at the first V0 attempt
(success or outer error) without a debt receipt or V1 continuation.
Planner-required suppresses Generic V0 before the witness and observes the
single Generic V1 route. Each mode is repeated on a fresh candidate and must
produce the same raw schedule and trace. This is execution evidence only; the
strict/planner error rows remain `UnresolvedStop` and do not close winner
equivalence.

Stop:
: If the real handler cannot be reached without production setup changes,
  synthetic outcomes, or forced failure injection, retain `UnresolvedStop`,
  keep the legacy scheduler, and do not advance to M5.

## Ordered M4 tasks

### M4-D0 — exhaustive stage matrix (`...-D0-S0`)

Inventory V0 and V1 separately, for strict/dev, release, planner-required, and
each contract disposition. The matrix must include all of these stages:

1. facts absent or non-matching;
2. composer precondition failure before allocation;
3. composer success and the first allocation/body/pipeline mutation;
4. composer `Err` after mutation (where reachable);
5. strict shadow lower `Some`, `None`, and `Err`;
6. release `PlanVerifier` `Ok` and `Err`;
7. release `PlanLowerer` `Some`, `Ok(None)`, and `Err`;
8. nested Generic composer calls and the direct nested-depth route.

Each row records the first effect owner, cursor/block changes, current legacy
continuation or terminal error, receipt kind, and proposed disposition. The
matrix must contain no implicit or unknown arm. Composer errors and strict
lower errors are recorded even when the current scheduler already stops on
outer `Err`; they are not incorrectly counted as post-effect retry debt.

Gate: focused registry/generic facts tests, the shared logical-demand guard,
pointer guard, release build, and `git diff --check` are green. All touched
source/check files stay below 800 lines.

### M4-D1 — stage owner and target disposition (`...-D0-S1`)

Assign every D0 row to the closed vocabulary. A row may be called
`PreEffectDeclined` only when the same decision is Builder-free and proven
equivalent to the current source/policy facts. Composer, verifier, or lower
failure after mutation cannot be renamed to decline. Rows without a proof stay
`PreEffectBlocked` or `UnresolvedStop`.

Gate: a test rejects any Builder-mutating row labelled Builder-free; the pure
policy subtree imports neither registry handlers nor legacy receipts. No
production caller or Recipe/PHI type is added.

### M4-D2 — V0/V1 overlap and precedence (`...-D0-S2`)

Audit the actual facts extractors and selection predicates using fixtures for
`V0-only`, `V1-only`, `Both`, and `Neither`, under strict, release, and
planner-required modes. V0 and V1 are migration policy adapters, not semantic
Recipe kinds.

The task closes only with one of these proofs:

- a production-derived invariant proves that valid facts never produce a
  simultaneous Generic V0/V1 schedule; or
- a real `Both` fixture fixes source-only precedence/blocked behavior before
  any effect, with no V0 post-effect fallback to V1.

Do not infer disjointness from route names or from `all_route_preflight`; that
function is an observer, not a winner oracle.

### M4-D3 — execution-path winner equivalence (`...-D0-S3`)

Extend the existing `cfg(test)` legacy witness bridge with a Generic debt
disposition. The fixture must use the actual AST/facts builder, selector, raw
schedule, Generic handler, and receipt path. It must capture the attempted
prefix, debt stage, and final winner.

The required proof is either a production-derived `V0 debt -> V1 success`
trace whose target winner is identical, or the closed D2 disjointness proof.
The existing synthetic CharMap decline/success fixture is not evidence for this
task. If the target cannot select the winner before the first effect, mark M4
`UnresolvedStop` and do not advance to Generic Recipe production.

Current observation: the real A1 `Both` trace is
`[GenericLoopV0, GenericLoopV1] -> GenericLoopV0 success`, with no debt receipt
and no V1 attempt. This proves the handler path is reachable, but it does not
prove pre-effect Generic qualification or the required debt-to-later-winner
equivalence; M4 remains `UnresolvedStop`.

Gate: compare winner and attempted prefix with the legacy witness oracle;
`all_route_preflight` is forbidden as the oracle. The test bridge is
test-only, and route IDs/receipts do not enter the pure policy result.

### Next bounded evidence slice — `JOINIR-GENERIC-ACCEPTED-PLAN-REACHABILITY0-D1-S1`

Choose the accepted-plan reachability audit before any V0-precedence policy
change. Run the existing D2 `V0-only`/`V1-only`/`Both`/`Neither` source fixtures
through the real facts and selector once, then observe each selected Generic
row on a fresh test candidate:

```text
RecipeComposer::compose_generic_loop_v0/v1_recipe
-> CorePlan::Loop root assertion
-> release PlanVerifier
-> release PlanLowerer (or strict shadow equivalent)
```

The observer records only stage evidence (`Accepted`, terminal-like `Err/None`,
or `UnresolvedStop`), first-effect owner, and candidate snapshot/fresh-reuse
result. It must not become a route policy, precedence oracle, Retry scheduler,
Recipe/JoinSig/PHI producer, or production caller. Composer errors remain outer
errors; legacy receipts remain diagnostic only.

Done means every fixture × mode row has an observed stage or an explicit
`UnresolvedStop`, with production caller count zero and no synthetic malformed
plan or forced failure injection. A green corpus is evidence about the known
Generic grammar only; it does not close D2 or prove all Generic inputs. Any
natural verifier/lower failure becomes the next concrete classification row.

Only after this slice may `JOINIR-GENERIC-V0-PRE-EFFECT-WINNER-TERMINAL0-D2-S2`
consider fixing the real `Both` overlap before effect. If V0 debt → V1 success
is observed, or V0 acceptance has a coverage hole, D2 remains `UnresolvedStop`
and the legacy scheduler stays authoritative.

### D2-A observation — accepted-plan reachability corpus

The test-only corpus now exercises the real source → facts → selector boundary
for `V1-only`, `Both`, `simple-while`, and `Neither` under release, strict, and
strict+planner-required configurations. Every accepted Generic composer row in
the known corpus produces a `CorePlan::Loop` root and identifies the Generic
composer as the first observed candidate effect. Composer errors are retained
as either precondition stops (no candidate effect) or effectful unresolved
stops; they are never relabelled as decline. Verifier/lower stage results,
candidate snapshots, and fresh-candidate repeatability are recorded without a
production caller; at least three rows reach `PlanLowerer::lower -> Some`.

This closes only the known-corpus reachability observation. It does not prove
the complete Generic grammar, make V0 a pre-effect winner, classify a natural
debt-to-V1 continuation, or close D2/D3. The next real verifier/lower failure,
or any coverage hole, remains an explicit `UnresolvedStop`.

### D2-A1 — one bounded Generic corpus extension

Extend the observer by exactly one natural V0 candidate: an additive condition
(`j + m < n`) with the existing numeric progression body. Reuse the same
facts→selector→composer→`CorePlan::Loop`→verifier→lower path and a fresh
candidate seed for `j`, `m`, and `n`. This is a reachability row, not a new
policy or semantic recipe kind.

Done: the raw selection and actual stage are recorded, and the fresh repeat is
stable. Stop if a specialized route wins, facts cannot be selected, setup needs
synthetic catalog/AST state, or any verifier/lower failure appears; retain the
row as `UnresolvedStop` instead of broadening the corpus. Receiver/scanner,
state-machine, RecipeOnly-break, and other facts-only fixtures remain outside
this slice until their production route ownership is separately proven.

Observation: release and strict additive rows reach a `CorePlan::Loop` root and
terminal `PlanLowerer::lower -> Some` with a stable fresh repeat. In
strict+planner-required mode, the selected Generic V1 row can reach a lower
error after candidate effects; that is an effectful `UnresolvedStop`, not a
pre-effect decline or an accepted terminal row. This is one more bounded data
point; it does not authorize V0 precedence or close D2-B.

### D2-A2 — true-condition body-derived step (bounded probe)

Probe one natural V1 candidate with `loop(true)` and a body-derived numeric
step. First record the real selector result. If `LoopTrue*`, LoopCond, or any
other specialized family owns the row, classify it as `Expected-NonGeneric` and
stop; do not force it through Generic. Only a raw `GenericLoopV1` selection may
enter the existing reachability observer, using the same fresh candidate and
stage/repeat gates as D2-A1.

This probe is intentionally one source shape. It does not widen the Generic
semantic contract, alter overlap policy, or convert extractor-only facts into
D2-B evidence.

Observation: the true-condition row is selected as Generic V1 and reaches a
terminal `PlanLowerer::lower -> Some` outside planner-required mode, with stable
fresh repeat. Any strict+planner-required composer or lower failure remains an
explicit unresolved row, classified by whether the fresh candidate changed.
This adds one natural body-derived-step row; it still does not close the
recursive Generic grammar or D2-B.

### D2-A3 — Generic structural grammar boundary census

Stop adding one-off fixtures after the bounded probes. Build one test-only
structure table from the actual facts/lower owners:

```text
condition arms
× step dispositions
× body/control items
× supported value-expression arms
× recursive depth boundary
```

Each row maps to the smallest natural AST fixture and the same actual
facts→selector→fresh candidate composer→`CorePlan::Loop`→verifier→lower
observer. Record `Observed-LowerSome`, `Observed-Composer+Verifier`,
`FactsOnly-Unwired`, `Expected-NonGeneric/Suppressed`, or `UnresolvedStop`;
facts-only extractor tests are not promoted. The table must include near-misses
such as unsupported value/lower arms and preserve their first-effect owner
rather than hiding them as decline. `all_route_preflight`, malformed
synthetic plans, and failure injection remain forbidden.

The stage-only observer does not establish a winner. For every `Both` row, D3
must additionally compare the actual witness attempted prefix and any debt
receipt with the legacy terminal; no stage-only row may be promoted to D2-B.

This census is the boundary for D2-B. A recursive or expression arm that is
accepted by facts but has no lower-stage observation keeps D2-B stopped; do not
claim universal V0 winner equivalence from the finite known corpus.

#### A3 boundary snapshot

The first census snapshot is intentionally small and source-anchored:

| axis | observed through composer/verifier/lower | facts-only boundary | non-Generic/suppressed boundary | status |
| --- | --- | --- | --- | --- |
| condition | numeric comparison (`i < 3`), additive numeric comparison (`j + m < n`), literal `true` | extended boolean/block-expression conditions | specialized condition families | `Observed-LowerSome` / `UnresolvedStop` |
| step placement | final numeric assignment; body-derived step | `InBody`, `InContinueIf`, `InBreakElseIf`, `BodyManaged` variants | scanner/state-machine-owned steps | `Observed-LowerSome` / `FactsOnly-Unwired` |
| body items | local + assignment, nested Loop + assignment | local + `env.console.error(i)` + step, If/Exit/Program/ScopeBox and other call/effect bodies | LoopCond/LoopBreak-owned exits | `Observed-LowerSome` / `Observed-ComposerError` / `FactsOnly-Unwired` |
| value expressions | integer literals, variables, arithmetic | `env.console.error(i)` effect-call row; field/index/array/map/Match/ThisField/Grouped/Await/QMark arms | unsupported source family gates | `Observed-LowerSome` / `Observed-ComposerError` / `UnresolvedStop` |
| recursion | one nested Loop in the Both fixture | deeper nesting and Loop+If+Return combinations | nested-specialized route ownership | `Observed-LowerSome` / `UnresolvedStop` |
| stage failures | no natural verifier/lower failure observed | `ReleaseVerifierRejected`, `ReleaseLowerFailed`, and strict shadow failure | — | `UnresolvedStop` |

The table is a coverage ledger, not a new semantic vocabulary. Rows marked
facts-only must not be counted by D2-B; rows that are accepted by facts but not
measured at lower remain unresolved. `GenericLoopV1ShapeId` is a hint policy,
not an exhaustive grammar, so it cannot close this table by itself.

The first explicit facts/lower mismatch set is `ThisField`, `MeField`,
`GroupedAssignmentExpr`, `Await`, `QMark`, `MatchExpr`, `New` with field
initializers, and unresolved/invalid `FromCall` forms. Facts-side predicates
can accept these recursively, while the current normalizer/lowerer has no
matching direct arm or can fail before a terminal plan. They are therefore
`FactsOnly-Unwired` or `UnresolvedStop`, never silently `PreEffectDeclined` or
`Expected-NonGeneric`.

#### A3 owner inventory

| owner | consumes | emits | non-authority |
| --- | --- | --- | --- |
| `facts/extract/v0.rs`, `facts/extract/v1.rs` | AST condition/body, canonical policy | Generic V0/V1 facts or `None`/`Freeze` | no MIR IDs, effects, or winner |
| `facts/expr_generic_loop.rs`, `facts/stmt_classifier`, `body_check/validation_{v0,v1}.rs` | expression/statement subtrees | acceptance, shape, step disposition | facts acceptance is not lower proof |
| `registry/predicates.rs`, `selection.rs` | `CanonicalLoopFacts` | raw route list | no V0/V1 winner equivalence |
| `recipe_tree/generic_loop_composer.rs` + `generic_loop_pipeline` | selected facts + `LoopRouteContext` | `CorePlan::Loop` or `Freeze`, candidate mutation | no retry/policy authority |
| `verify/verifier/core.rs` and recursive validators | `CorePlan` | verifier `Ok`/`Err` | no source/facts interpretation |
| `lower/core.rs`, `loop_lowering.rs`, `loop_completion.rs` | verified plan + Builder | MIR mutation, `Some(Void)`/`Err` | no route selection |
| `execution_witness.rs` + Generic handlers (D3 observer only) | raw schedule, env, facts | attempted prefix, receipt, terminal | no pure policy authority |

The Generic condition predicate and nested RecipeBlock condition predicate are
separate owners (`facts/expr_generic_loop.rs` versus `facts/expr_bool.rs`). A
Generic fact accepted by the first may still fail an `ExitAllowed`/`NoExit`
RecipeBlock precondition; A3 records that as a boundary, never as a silent
decline.

### D2-A4 — V1-only local-plus-effect row

Change:
: Observe one natural V1-only body containing `local tmp = 0`, a supported
  `env.console.error(i)` effect call, and the final numeric step. The local must make
  V0 reject before selection; the real V1 facts→selector→composer→verifier→
  lower path is observed on a fresh candidate in all three modes.

Contract:
: Test-only evidence. Reuse the existing Generic observer and scope boundary;
  no new route, policy, Recipe, PHI, retry, or production caller.

Done:
: Raw selection is exactly Generic V1, the stage and first-effect owner are
  recorded, and a fresh repeat is identical. A composer/lower error after an
  effect is an explicit `UnresolvedStop` row.

Stop:
: If facts do not select V1, setup needs synthetic catalog state, or the
  effect call exposes a facts/lower mismatch, retain the row as unresolved and
  do not broaden to arbitrary calls or close D2-B.

Observation: all three modes select exactly `GenericLoopV1`, but the current
composer enters its pipeline and returns `Err` after candidate effects for this
body. The fresh-candidate trace therefore classifies the row as an effectful
`ComposerPipelineError`/`UnresolvedStop`, not as a pre-effect decline or a
successful V1 terminal. This is the intended A3 evidence: facts acceptance and
lowering support are currently separate boundaries.

### D2-B-E1 observation — effect body without local

The smallest natural overlap probe removes only `local tmp = 0` from A4 and
keeps `env.console.error(i)` plus the numeric step. It does **not** produce a
`Both` row: the actual facts boundary reports V0 absent, V1 present, and raw
`[GenericLoopV1]` in release, strict, and planner-required modes. The fresh
candidate reachability row enters the Generic composer, changes candidate
state, and stops at an outer `ComposerError`; the witness attempts exactly V1,
records no typed Generic debt, and does not retry. This row is therefore a
bounded V0-negative facts/lower boundary, not V0/V1 precedence or winner
equivalence evidence. Keep it explicit so future V0 grammar changes cannot
silently turn an effect body into an overlap claim.

#### D0 invariant refinement — lower `None`

Two stage-matrix arms can now be classified by an existing code invariant:

- both successful Generic composers construct `CorePlan::Loop` only;
- `PlanLowerer::lower` dispatches that root to `lower_loop`, whose completion
  emits the loop `Void` value and returns `Ok(Some(_))`.

Therefore `ReleaseLowerReturnedNone` and `StrictShadowLowerReturnedNone` are
`ImpossibleEdge` for a valid selected Generic plan. The legacy receipt arms
remain as fail-fast diagnostics if that invariant is violated; this statement
does not authorize deleting them before M10. `ReleaseVerifierRejected` and
`ReleaseLowerFailed` still require natural structural rows and remain
`UnresolvedStop`.

### D2-B — gated V0 overlap decision (`JOINIR-GENERIC-V0-PRE-EFFECT-WINNER-TERMINAL0-D2-S2`)

This is the next design stop, not an implementation permission. It may begin
only after D2-A has named the accepted Generic coverage boundary and every
selected `Both` row has a production-derived stage result. The candidate
policy is an effect-free V0 winner certificate; it is not a new route, Recipe
kind, or scheduler.

The decision closes only if all of these hold:

- V0 acceptance covers the `Both` rows being claimed, including strict and
  planner-required dispositions;
- no observed V0 post-effect debt reaches a V1 success with a different winner;
- V0 failure, if any, is classified before or at the candidate abort boundary,
  never silently reselected through a dirty Builder;
- the pure policy result and the legacy witness agree on route and attempted
  prefix for every accepted fixture.

If any condition is missing, keep Generic debt as a legacy diagnostic and keep
the ordered scheduler authoritative. No `GenericDebt -> Blocked` rewrite,
`Retry` deletion, or production caller is allowed under D2-B.

#### D2-B-E1 — overlap evidence matrix (design stop)

The next bounded task is to join the three existing observations for each
claimed `Both` row into one evidence matrix. This is an observation/oracle
product, not a new policy owner.

| column | required evidence |
| --- | --- |
| source/mode | exact A3 grammar row, release/strict/planner-required mode |
| frame input | production-derived `RouterEnv` (`strict_or_dev`, `planner_required`, `has_body_local`) and recipe-contract disposition |
| selection | actual facts, raw schedule, and `Both`/`V0Suppressed`/`V1-only`/`Neither` disposition |
| direct stage | fresh-candidate V0 and V1 composer/verifier/lower stage, first-effect owner, candidate delta |
| legacy witness | attempted prefix, typed debt receipt (if any), outer error or terminal winner |
| comparison | pure pre-effect disposition versus legacy route, prefix, and terminal |
| classification | `PreEffectDeclined`, `PreEffectBlocked`, `TerminalFreezeTarget`, `ImpossibleEdge`, or `UnresolvedStop` |

Run the direct V0 and V1 paths on separate fresh candidates before reading the
legacy witness. Then run the real witness for the same source row and mode.
The observer now constructs the same `LivePreflightFrameV1` through the shared
production preparation helper, capturing `strict_or_dev`, `planner_required`,
`has_body_local` from facts, recipe-contract presence from the outcome, and
`recipe_first_allowed`. The test-only witness accessor borrows only the frame's
raw schedule and environment; it cannot bypass a release gate. The strict
planner ambiguity rejection remains a production pre-frame gate and is not
fabricated by the observer.
`all_route_preflight`, synthetic outcomes, malformed plans, and failure
injection are forbidden. A4's `env.console.error(i)` row remains a separate
`effectful outer Err / UnresolvedStop` row; it is not a debt receipt and is not
evidence for V0/V1 precedence.

Done requires every claimed `Both` row to have a real stage, first-effect
owner, candidate snapshot, and witness terminal. Every observed typed debt must
also record its receipt kind and whether a later suffix was actually attempted.
The bounded decision may close only by either a source-derived pre-effect V0
winner certificate covering the claimed modes/rows with no debt-to-different-
winner edge, or a complete disjointness invariant. A V0 success with no debt
is only one legacy terminal observation; it does not prove pre-effect
qualification or global winner equivalence.

Stop and retain the legacy scheduler/receipts when a facts-only or
lower-unmeasured row remains, a candidate abort boundary is missing, a natural
debt receipt is not observed, an outer error needs semantic relabelling, V0
failure would require retry through a dirty candidate, production and test
frame/env/contract inputs differ, or policy/witness route/prefix/terminal
disagree. `V0Suppressed` planner-required rows are a separate pre-effect gate,
not overlap proof. No Generic Recipe, PHI, JoinSig, Retry rewrite, or
`route_loop` production caller is permitted in this row.

Implementation observation: the test-only `GenericOverlapEvidenceRowV1` now
stores, for the actual `Both` fixture in release/strict/planner-required modes,
the direct fresh-candidate V0/V1 stages and before/after candidate snapshots
beside the frame-bound witness trace.
Those snapshots are stage-owner hints only (cursor/block/map counts); they are
not the M5 whole-candidate isolation proof and must not be promoted as one.
Release/strict retain raw `[GenericLoopV0, GenericLoopV1]` and the witness
terminates at V0 without a debt receipt; planner-required retains only V1.
This is a joined evidence matrix, not a winner oracle: the direct stage and
legacy terminal are recorded together, while pre-effect policy equivalence and
disjointness remain open.

#### D2-B-E2 — semantic parity for real Generic overlap

The current `Both` row is a real source overlap, not merely an incomplete
observer row. In this row, the common body/step conditions and the named V1
shape guard still leave both Generic facts present. This does not claim that
the V0 extractor has only one rejection rule; it means this concrete overlap
survives the existing extraction gates. Therefore an evidence-only
disjointness claim is unavailable for this class.

The next design task is
`JOINIR-GENERIC-OVERLAP-SEMANTIC-PARITY0-D2-B2`. It remains test-only and
must use the existing Facts -> selection boundary and fresh candidates. For
each claimed natural overlap row (starting with `Both`, then existing nested
loop/If/exit fixtures that actually reach both facts), record release and
strict-mode rows:

- the frame-derived environment/contract and raw selection;
- independent V0 and V1 compose -> verify -> lower stages, first-effect owner,
  and before/after candidate snapshots;
- an alpha-normalized semantic CorePlan digest that erases physical IDs while
  retaining loop carrier, body/condition operations, PHI inputs, exits, and
  result binding semantics;
- the real legacy witness route, attempted prefix, debt receipt, and terminal.

This task may close only when every claimed row has a pre-effect V0 winner
certificate, V0 terminal semantic parity with the accepted V1 meaning, no
V0-debt -> different-V1-winner edge, and fresh-repeat stability. Any semantic
digest difference, natural V0 failure/debt, unmeasured grammar arm, or missing
candidate delta keeps the row `UnresolvedStop`. Planner-required V0 suppression
is a separate pre-effect gate; it does not require a V0/V1 digest and is not
overlap proof. A V1 direct-stage error cannot serve as an accepted-V1 semantic
baseline. Any additional fixture must pass through full `try_build_outcome` and
the actual raw selector before it is called a production overlap. Do not add a
new route, Recipe kind, scheduler, or production consumer for this task.

The pure evaluator for that certificate must consume only the frame-derived
environment/contract, canonical facts, and the already-issued raw schedule.
`raw_schedule.first() == GenericLoopV0` alone is not a certificate; the V0
qualification predicate and its covered grammar rows must be fixed in the same
test-only matrix.

The existing `normalized_semantic_plans` helper is not by itself sufficient:
it preserves `BasicBlockId`/`ValueId`/PHI/fragment identifiers. D2-B2 must add
an explicit deterministic first-seen remapping layer (test-only or a narrowly
owned shared helper) before treating two fresh candidates as semantically
equal. Capture the digest before moving the plan into `PlanLowerer` (including
the strict shadow path). Digest equality is sufficient evidence for the
recorded shape, not a substitute for runtime/MIR parity; digest difference
keeps the row `UnresolvedStop`.

Current evidence: the test-only
`registry/generic_semantic_digest_tests.rs` remaps ValueId, BasicBlockId,
LocalSlotId, LoopId, PHI inputs, Frag edges, and branch arguments before the
plan is moved into `PlanLowerer`. Fresh release/strict `Both` candidates are
stable, but their V0 and V1 digests differ (the planned nested-carrier and
step shapes are not semantically identical). Planner-required suppresses V0
before selection and is recorded separately. D2-B2 therefore remains an
explicit `UnresolvedStop`; no Generic winner or production cutover is
authorized.

#### D2-B2-NESTED-CARRIER-MEANING — external binding witness

The `Both` digest difference is a semantic boundary, not an alpha-remapping
or CFG-noise problem. Its body contains an inner loop that writes the outer
binding `j`. V0 only observes top-level body writes, while V1 recursively
collects nested assignment targets, creates the outer `loop_carrier_j` and
`loop_step_in_j` PHIs, and publishes `j` in the outer `final_values`. The
lowerer restores the pre-loop binding map and reapplies only those outer final
values; therefore treating the two plans as equivalent would hide a real
post-iteration binding difference and could lose `j` across outer iterations.

The first follow-up is test-only `nested_carrier_semantic_witness`. It must
use the real `Both` AST, `try_build_outcome`, the actual raw selector, and
fresh V0/V1 candidates. The evidence records the outer and nested
`final_values`/PHI tags, verifies both plans, and confirms the existing
composer/verifier/lower path on release and strict fresh candidates. It is a
meaning witness, not a runtime oracle or a production policy.

The next policy gate is a facts-only
`GenericOverlapDisposition::V1ForNestedCarriers`: recursively observed writes
to an outer binding (including writes in nested Loop/If/Exit paths) suppress
V0 before Builder effects and select V1. Planner-required remains a separate
pre-effect gate. V0 remains eligible for overlap rows without a recursive
carrier. Every other overlap class (including natural If/Exit cases) stays
`UnresolvedStop` until its own witness exists. If V1 cannot verify/lower a
claimed row, retain the legacy scheduler and classify that row as unresolved;
do not convert the failure into a retry or a global Freeze.

Done for this boundary requires the structural witness, fresh-repeat
stability, and a source-level observable expectation for the outer `j`
binding. It does not authorize Generic production cutover, Retry deletion,
PHI ownership, or broad non-Generic cutover. Those remain downstream gates
after all overlap classes have a policy owner.

#### D2-B2b — facts-only carrier observation contract

The policy must not inspect `ASTNode`, `RecipeBody`, route IDs, or CorePlan.
The selected extraction owner is the Generic V1 facts transaction after
`flatten_scope_boxes`; it emits one owned Builder-free observation with the
four dispositions `Complete::NoRecursiveCarrier`,
`Complete::RecursiveCarrier(targets)`, `Unavailable(container)`, and
`Ambiguous(reason)`. Targets are deterministic source-binding labels. Empty
targets never mean complete coverage by themselves. The neutral policy then
consumes only this observation plus the already-frozen raw schedule and
mode/contract snapshot. This preserves the M3 rule that Facts observe and
Policy selects, and prevents a second AST matcher from appearing in
`loop_route_policy`.

The extractor recurses through the accepted `If`/`Loop`/`Program` grammar after
ScopeBox flattening; every preserved but unsupported container is
`Unavailable`, never silently skipped. Cross-mode Both/simple, unsupported,
and fresh-repeat fixtures are the schema-acceptance gate.

The schema gate is green for real cross-mode `Both`/simple observations,
preserved-container `Unavailable`, and fresh repeat stability. D1 now emits
this observation from `GenericLoopV1Facts` as a facts-only field; it does not
change selection or lowering. Missing, unsupported, or ambiguous coverage is
`UnresolvedStop`, never an implicit V1 winner. `V1ForNestedCarriers` may
suppress V0 only when the observation is complete, the raw schedule contains
the claimed overlap, and V1 has a separate natural stage result.
Planner-required V0 suppression remains a distinct gate.

Do not add a route-name branch, re-read source AST in policy, or use this field
as a winner. D1 must leave all overlap classes unresolved and production
selection unchanged. A test-only neutral probe may consume only the observation,
opaque overlap/mode/contract snapshots, and an independently accepted V1 stage;
it may return `V1ForNestedCarriers` only for complete recursive coverage, and
otherwise returns `UnresolvedStop`. Policy promotion remains a later D2 gate.

#### `JOINIR-LOOP-ACCUM-PORTABLE-RECIPE0-D0` — design/test-only pilot

Change:
: Fix the Accum pilot contract only: real `try_build_outcome` -> shared
  `LivePreflightFrameV1` -> raw selector, passive portable Recipe
  verification, and the caller-zero/authority guard. Do not add a composer,
  physicalizer, candidate mutation, or `route_loop` caller.

Contract:
: Source authority is the production-derived facts/frame/raw schedule. The
  portable artifact/verifier is a passive semantic contract. The legacy
  Accum composer/CorePlan/PlanLowerer/JoinIR path is parity evidence only;
  `diagnostic_effective`, route-name certificates, and legacy receipts are
  non-authority.

Done:
: A real direct Accum row proves exact raw schedule `[AccumConstLoop]`, no
  Generic suffix, fresh-repeat stability across release/strict/planner modes,
  and zero portable production callers. The existing golden Recipe verifies
  independently. All touched files remain below 800 lines.

Stop:
: Any Generic suffix, missing shared M6 CFG/JoinSig/PHI owner, candidate
  mutation, legacy composer import into the portable path, or request for
  runtime MIR parity returns this row to design. M6 establishes the shared
  physical owners first; only afterward may the caller-zero vertical pilot
  exercise candidate abort/fresh reuse. M10a is the separate future singleton
  production bridge; Generic D2 remains required for M10b.

This row is the only M5 work permitted while D2-B is open. Generic Recipe
consumption, Retry deletion, shared CFG/JoinSig/PHI implementation, and all
production cutover remain prohibited; this evidence does not bypass M4.

### M4-D4 — handoff and close (`...-D0-S4`)

Seal a disposition for every D0 row, decide V0/V1 precedence, and make the
winner-equivalence or disjointness proof green. Confirm that all debt is either
Builder-free decline, an explicitly terminal Freeze target, or a proven
impossible edge. Keep legacy behavior and production callers unchanged.

M4 Done does **not** claim a Generic Recipe, PHI ownership, candidate publish,
JoinIR fallback deletion, or all-route cutover. Those claims require:

```text
M5 caller-zero Accum vertical pilot
-> M6 shared CFG/JoinSig/PHI owners
-> M7 five-family recursive Recipe producer
-> M8 all 19 adapters
-> M9 Rust/.hako producer parity
-> M10 atomic route_loop cutover and old-authority deletion
```

If any row remains `UnresolvedStop`, retain the old Generic scheduler and
receipts, record the exact blocker, and stop. Do not add a Loop-local Builder,
undo journal, symbolic MIR replacement, or another route-specific proof loop.

## Required guard additions after D0

The shared guard should eventually check: exactly eight existing Generic debt
branches plus the two Generic composers; receipt ownership remains in the
legacy receipt module; policy code has zero registry/receipt imports; M4
helpers have zero production callers; and any future semantic target has zero
`Option`, `Retry`, or raw suffix. M10-only deletion checks stay inactive until
the atomic cutover card is active.
