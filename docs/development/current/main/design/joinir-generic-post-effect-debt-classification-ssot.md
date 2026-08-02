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

Gate: compare winner and attempted prefix with the legacy witness oracle;
`all_route_preflight` is forbidden as the oracle. The test bridge is
test-only, and route IDs/receipts do not enter the pure policy result.

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
