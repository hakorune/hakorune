# Generic Loop post-effect debt classification

Status: active design stop

Decision: accepted — `JOINIR-GENERIC-POST-EFFECT-DEBT-CLASSIFICATION0-D0-S0`

This card is the detailed task order for the Generic V0/V1 debt named by the
Loop pipeline SSOT. It is a design and test-only boundary. It does not create a
portable Recipe producer, a PHI writer, a second scheduler, or a production
cutover.

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
