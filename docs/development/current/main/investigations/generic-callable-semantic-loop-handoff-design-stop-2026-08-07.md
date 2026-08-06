# Generic callable-semantic Loop handoff design stop

Status: `Decision: accepted design stop; implementation is not authorized until the handoff product is sealed`

Date: `2026-08-07`

Parent rows:

- `GENERIC-STATIC-CALL-PUBLICATION-SOURCE-BOUND-ISSUER-S0`
- `GENERIC-SOURCE-TO-PORTABLE-RECIPE-D0`
- `GENERIC-G0-RECIPE-S4-I0-R0`

## Observed blocker

The source-bound static-call result publication slice can lower the direct
initializer call, but a callable containing a Loop still fails its semantic
ledger at the callable boundary:

```text
[freeze:contract][callable-semantic-lowering/incomplete-consumption]
entry=true locals=2/2 variables=2/4 assignments=0/1 lambdas=0/0
```

The exact missing source sites in the focused prelude-free fixture are:

```text
Body(2).LoopCondition.Lhs
Body(2).LoopBody(0).Value.Lhs
Body(2).LoopBody(0).Target
```

This is not a GenericLoop type-inference problem.  The live path is:

```text
RawInvocationChildPortV1::lower_loop
  -> PreparedLocatedRawLoopChildEntryV1
  -> lower_loop_or_freeze_v1
  -> legacy Generic route/composer/pipeline
  -> PlanLowerer
```

`PreparedLocatedRawLoopChildEntryV1` currently retains the condition/body
receipts only until the route call and then discards them.  The Generic
normalizer/plan pipeline uses `RawLoopPlanExpressionPortV1` and Builder
`variable_map` values; it does not call the callable ledger's exact
`read_variable` or `rebind` operations.  `CorePlan`/`CoreEffectPlan` also lose
the variable/assignment source sites, so post-effect counting cannot repair
the mismatch.

## Decision

Introduce one source-bound handoff product before Generic route composition:

```text
VerifiedCallableSemanticLoopBindingScheduleV1
```

It is AST-free, non-Clone, source-owned, and single-use.  It co-seals:

```text
callable owner / source frame
loop condition and body source sites
variable-read BindingRefs
assignment-target/value BindingRefs
loop step/after relation
exact plan-node or logical Recipe relation
```

The handoff is passed from the raw Loop entry into an AST-free
`GenericSourceProjector`.  The projector only issues the verified source
obligations; it does not consume the callable ledger and does not allocate a
`ValueId`.  The portable Recipe and its verifier retain the exact obligations
without reading AST, `variable_map`, or Builder state.

After Recipe verification, the canonical physicalizer receives a narrow
move-only `CallableBindingMaterializer` capability.  Inside the same
BindingSSA/PHI transaction it consumes each obligation exactly once:

```text
verify source-site + BindingRef
  -> ledger consume_read(site, binding)
  -> BindingSSA read/define

verify assignment-site + BindingRef
  -> ledger consume_rebind(site, binding)
  -> BindingSSA rebind/PHI publication
```

The physicalizer is the sole BindingRef-to-`ValueId`/PHI owner.  The current
`CallableSemanticLoweringState::read_variable(site) -> ValueId` and
`rebind(site, ValueId)` surface is a legacy bridge only; it must not become a
second production ValueId map.  The eventual semantic ledger consumes
site/BindingRef receipts, while BindingSSA owns all physical values.

## Upper architecture review

The worker review confirms that this is a boundary problem, not a missing
branch inside the legacy Generic loop lowerer.  The clean authority chain is:

```text
resolved callable owner/frame + exact source projection
  -> GenericSourceProjector
  -> VerifiedCallableSemanticLoopBindingScheduleV1
  -> common Generic LoopRecipe / JoinSig
  -> RecipeVerifier
  -> one canonical CFG / BindingSSA / PHI physicalizer
  -> function draft seal
  -> atomic module publication
```

The projector observes source sites and BindingRefs but never allocates
`ValueId`; the Recipe is AST-free and never inspects `variable_map`; the
physicalizer is the only BindingRef-to-ValueId/PHI owner.  A narrow move-only
materializer capability may consume the callable ledger at that physical
boundary, but a `Rc<RefCell<CallableSemanticLoweringState>>` must not leak
into Facts, Recipe, route scheduling, or `PlanLowerer`.

The D0 source-to-Recipe relation is therefore fixed as a role matrix, not an
implementation guess:

| source obligation | portable Recipe relation | physicalizer obligation |
|---|---|---|
| condition read site + BindingRef | loop condition read role | one ledger read, then BindingSSA read |
| body value read site + BindingRef | body value role | one ledger read, then BindingSSA value |
| assignment target/value sites + BindingRefs | recurrence rebind role | one ledger rebind, then PHI/BindingSSA publish |
| loop-local declaration site | local/edge carrier, only if profile admits it | one declaration materialization |
| step/after relation | exit/continue edge relation | one canonical CFG edge publication |

No row may be inferred from a name, span, `ValueId`, or AST after the
projector closes.  A row that cannot be proven is a typed pre-effect decline,
not an invitation to widen the profile during lowering.

The existing `RawLoopChildEntry -> route_generic_loop_v1` path therefore stays
legacy evidence until the selected callable caller reaches zero.  Retirement
requires, in order:

```text
new source-bound callable-loop caller = 1
old callable Generic route callers = 0
AST-bearing Generic composer/normalizer production callers = 0
CorePlan/ValueId legacy production callers = 0
reference fixtures and migration dispositions updated
legacy files deleted or archived
```

No broad Generic support, name-based source inference, second SSA/PHI owner,
post-effect repair, or compatibility fallback is claimed by this stop.

## Ownership map

| Concern | Sole owner | Must not own |
|---|---|---|
| source loop sites and BindingRefs | resolver/source observation | AST reread, names |
| handoff lifetime and single use | raw Loop entry/session | route retry |
| source-aware Recipe input | GenericSourceProjector/Recipe producer | Builder `ValueId` allocation |
| callable ledger consumption | canonical physicalizer materializer transaction | `PlanLowerer` or Recipe access |
| physical CFG/PHI | canonical physicalizer + Binding SSA | source names |
| candidate rollback | outer module candidate session | route-local snapshots |

## First implementation slice

Only one explicitly inventoried callable-loop profile is admitted initially.
The focused diagnostic fixture contains locals, calls, and four variable reads;
it must not be relabeled as a broader Generic G0 shape by implementation
convenience.  The profile is admitted only after its exact role inventory is
sealed; otherwise it declines before effects:

```text
condition: exact source loop condition and read roles
body: the explicitly admitted local/call/assignment roles
step/after: exact relation for the selected profile
```

Before any Builder effect or provisional PHI, the route must verify:

```text
handoff owner/frame matches the callable
all source sites are covered exactly once
all BindingRefs belong to the callable
plan/Recipe relation is complete
```

If the handoff or profile is missing, foreign, duplicated, or incomplete,
return a typed pre-effect `Freeze`/`PreEffectDeclined`.  There is no retry,
fallback, name lookup, AST rewrite, or GenericLoop type backfill.

## Explicit non-goals

```text
PlanLowerer or Recipe reading CallableSemanticLoweringState = 0
post-effect ledger repair = 0
route-local AST scan = 0
variable-map/name fallback = 0
CorePlan source-site reconstruction = 0
Builder `LoopPlanExpressionPortV1` as semantic ledger owner = 0
second production ValueId/BindingRef map = 0
generic production cutover = 0
legacy route deletion = 0
```

The existing source-bound static-call publication adapter remains valid and
independent.  It publishes the call result type; this row supplies the missing
Loop source/BindingRef contract.  Neither row may infer the other's facts.

## Ordered task ladder

1. `GENERIC-CALLABLE-SEMANTIC-LOOP-HANDOFF-D0` — close the exact schedule
   schema and source/Recipe relation matrix (this document).
2. `GENERIC-CALLABLE-SEMANTIC-LOOP-HANDOFF-S0` — issue one non-Clone
   source handoff envelope and verify the selected role profile before the
   legacy route. This is a pre-effect evidence row only; it does **not** yet
   claim the portable Recipe/JoinSig mapping or physical ledger consumption.
   Add positive and foreign, duplicate, missing, incomplete, and unsupported
   profile negatives.
3. `GENERIC-CALLABLE-SEMANTIC-LOOP-PHYSICAL-S1` — first close the
   source-to-Recipe/JoinSig relation, then let the canonical physicalizer
   consume the verified schedule through one materializer transaction and
   prove ledger `4/4` variables, `1/1` assignment, and successful callable
   finish without a second ValueId owner.
4. `GENERIC-CALLABLE-SEMANTIC-LOOP-PORT-S2` — remove the selected callable
   dependency on the name-only Builder `LoopPlanExpressionPortV1`; keep that
   port compatibility-only and co-seal the Recipe relation with Binding SSA.
5. Re-run the canonical strict receipt.  Only after it is green may the
   Generic production-selection/cutover rows reopen.

Each implementation row must update the exact `docs/reference/**` entry,
immutable fixture/receipt, current pointer, and active workstream in the same
commit.  No production claim follows from the design stop alone.
