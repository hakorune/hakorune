---
Status: D0 closed as NoSafeSlice after worker audit; successor Recipe-authority D0 is active
Task: MIR-CALLABLE-LOOP-READY-PRODUCTION-INGRESS-D0
Date: 2026-08-22
Priority: connect one source-backed Ready ingress to one named normalizer consumer
Parent: MIR-CALLABLE-LOOP-READY-STRUCTURAL-LEASE-I0
PreviousCard: mirbuilder-callable-loop-ready-structural-lease-i0-2026-08-22.md
NextCard: mirbuilder-callable-loop-ready-generic-loop-v1-recipe-authority-d0-2026-08-22.md
---

# Callable Loop Ready production ingress D0

## Six-line brief

Decision: design one production ingress for callable `Ready` that consumes the
raw child entry exactly once, issues source Facts once, claims the retained
pre-effect receipt once, and passes it through the source-bound structural
lease to one named normalizer consumer. The current `Ready` edge to the old
route remains closed until this Decision is accepted.

Source authority + canonical issuer: `PreparedLocatedRawLoopChildEntryV1`
owns the exact located source relation;
`CallableGenericLoopSourceFactsIssuerV1::issue_once` is the sole Facts/Recipe
issuer; `CallableLoopStructuralLeaseIssuerV1` is the sole source-bound
structural handoff issuer. The future normalizer/plan consumer must consume
those products and must not become a second source or route issuer.

Non-authority: the discarded local pre-effect receipt, the old
`lower_loop_or_freeze_v1` path, `LoopRouteContext::new/with_fn_body`,
`choose_route_kind`, `route_loop`, registry re-selection, AST/name/ordinal/
pointer pairing, Builder state, `ValueId`, physical receipts, and any
fallback or retry after a source-aware rejection.

Fail-fast boundary: after the raw child entry is prepared and before any
Builder effect. `Outside` is a typed terminal; missing/invalid source Facts,
foreign lineage, non-front selection, or an unavailable named consumer are
terminal errors. No source-aware failure may continue to the old route.

Smallest next slice: census the exact existing Recipe/Join producer that can
consume the already-issued `PlanBuildOutcome`/selection without rebuilding a
`LoopRouteContext`, then write one bounded consumer contract and its finite
state. This D0 does not implement the production call, normalizer, PlanLowerer,
Builder, physical effects, or publication.

Non-claims: ordinary Loop lowering, route/registry selection, `CorePlan`
construction unless an existing producer and owner are named, physical MIR,
ledger/`ValueId`, Outside ordinary consumption, nested/multi-carrier loops,
fallback retirement, production switch, and performance.

## Current production gap

The current raw invocation path is still:

```text
RawInvocationChildPortV1
  -> PreparedLocatedRawLoopChildEntryV1
  -> Ready pre-effect receipt is consumed into a discarded local
  -> lower_loop_or_freeze_v1
  -> cf_loop_joinir_impl
  -> LoopRouteContext::new/with_fn_body
  -> route_loop
```

The caller-zero source Facts issuer and structural lease now provide the
missing transport pieces, but they are not production-connected. The old path
must not be called a fallback-compatible consumer of `Ready`; it is the
unreplaced edge that this D0 must either replace or explicitly leave outside
the selected callable source cohort.

The `None` callable-handoff case and the already-terminal `Outside` case must
be kept separate. A missing callable source admission is not permission to
reinterpret a `Ready` receipt, and an `Outside` row must not be fed to ordinary
JoinIR merely because the old API returns `Result<ValueId, String>`.

## Required design decisions

Before implementation, close these points in one accepted Decision:

1. **Exact production ingress owner.** Name the one function at the raw child
   boundary that receives `PreparedLocatedRawLoopChildEntryV1` and can move it
   into the source Facts payload. Do not add a parallel AST/source argument.
2. **Ready transition.** Fix the single transition:

   ```text
   Ready(schedule)
     -> source Facts payload
     -> issue_once
     -> claim_all
     -> structural lease prepare
     -> one named consumer
   ```

   `claim_all`, source Facts extraction, route selection, and structural lease
   preparation must each have one production caller after cutover.
3. **Consumer owner and input.** Identify an existing Recipe/Join producer or
   create a narrowly owned source-aware consumer only after its authority is
   named. It may borrow the HRTB `CallableLoopReadyStructuralViewV1`, but it
   must not reconstruct AST facts, create a `LoopRouteContext`, or select the
   registry again.
4. **Plan boundary.** Decide whether the existing `PlanBuildOutcome` plus
   `VerifiedLocatedGenericLoopV1SelectionV1` is already sufficient for the
   existing Recipe/Join producer. If it is not, name the missing canonical
   issuer and stop; do not fill the gap with route kind, function name,
   defaults, or MIR observation.
5. **Typed terminal outcomes.** Preserve `Outside`, source rejection, and
   consumer rejection as structured outcomes until the outer raw API performs
   the one final string conversion. No error may be converted to a successful
   `ValueId` or old-route retry.
6. **Effect boundary.** Prove that all source/plan/claim checks happen before
   Builder mutation. If the named consumer later lowers through an unpublished
   function session, document exactly which failures are still allowed after
   the source handoff and how the session is poisoned/discarded.

## Authority map for the next Decision

| Owner | Owns | Must not own |
| --- | --- | --- |
| `PreparedLocatedRawLoopChildEntryV1` | exact parent/condition/body source and callable disposition | route choice, Builder effect |
| `CallableGenericLoopSourceFactsIssuerV1` | one source-backed Facts/Recipe outcome and exact selection | physical lowering, fallback |
| `CallableGenericLoopSourceFactsReceiptV1` | one claimed pre-effect/source/planner relation | independent AST/source reconstruction |
| `CallableLoopStructuralLeaseIssuerV1` | same-lineage route-neutral lease and HRTB view | route classification, registry, plan meaning |
| future named normalizer consumer | consume the issued view and invoke one existing producer | source re-observation, route re-selection, physical repair |
| existing Recipe/Join producer | issue/consume its existing plan contract | source pairing, Builder-owned semantic inference |
| old raw route | unchanged non-selected legacy edge until cutover | consuming callable `Ready` after cutover |

The final table must name a single canonical issuer for any new product. If
the next consumer needs a new `Verified*` or `Prepared*` semantic receipt,
this D0 must first identify its source authority and issuance site.

## Required census and counterexamples

Record before coding:

```text
Raw Ready ingress production callers = 0 before cutover
CallableGenericLoopSourceFactsIssuerV1::issue_once = 0 before cutover
CallableGenericLoopSourceFactsV1::claim_all = 0 before cutover
CallableLoopStructuralLeaseIssuerV1::prepare = 0 before cutover
Ready -> lower_loop_or_freeze_v1 = 1 old edge
Ready -> LoopRouteContext/route_loop = 1 old edge through the old route
```

The Decision must include negative examples for:

```text
Ready with foreign source lineage
Ready with source Facts rejection
Ready with non-GenericLoopV1 selection
Ready after a second claim attempt
Outside reaching an ordinary physical consumer
source-aware rejection reaching the old route
named consumer rebuilding a route context or Facts
```

## Acceptance and reusable guards

```text
one named production Ready ingress
source Facts extraction = 1
source Facts claim = 1
structural lease preparation = 1
old Ready -> lower_loop_or_freeze_v1 = 0 after cutover
old Ready -> LoopRouteContext/route_loop = 0 after cutover
Ready rejection -> old route/fallback/retry = 0
Outside -> Builder effect = 0
source/AST second walk = 0
route-neutral planner invocation = 1
registry selection after source Facts = 0
all new touched Rust files < 760 lines
```

Focused evidence must prove one natural Ready reaches the named consumer,
each source/lineage/selection failure has zero effect, and no old route is
entered. A caller count of zero is only the pre-cutover baseline; it is not
production completion.

## NoSafeSlice conditions

Remain in `design_stop` if implementation requires any of:

```text
passing an independent LoopRouteContext beside the source receipt
re-running Facts/Recipe/selection
AST/name/ordinal/pointer/digest pairing
using Builder state or ValueId to prove source membership
turning Outside into ordinary JoinIR
converting Ready failure into old-route retry
inventing a CorePlan/Verified* product without a named issuer
physical effect before source claim/consumer validation
changing the raw route and source Facts in one unbounded patch
```

The next implementation card may be opened only after the consumer owner,
plan producer, fail-fast boundary, and production caller census are written in
one accepted Decision.

## Worker audit closure

The worker audit confirms that the existing
`RecipeComposer::compose_generic_loop_v1_recipe` is not a source-aware
consumer. It requires `&mut MirBuilder`, `&LoopRouteContext`, skeleton
allocation, and the generic-loop physical pipeline; it emits `CorePlan`
fields containing physical `ValueId`/`BasicBlockId` state. The separate
`VerifiedLocatedCoreLoopPlanV1` also requires a
`VerifiedCallableResultActivationPlanV1`, caller ledger, and `LegacyStmtInputV1`
that the current source receipt does not own.

The current `PlanBuildOutcome`/selection pair therefore proves only Facts and
the selected `GenericLoopV1` route. It is not yet a Recipe/Join product. The
raw `Ready` edge remains unchanged and all source Facts/claim/lease production
callers remain zero.

This card is closed as `NoSafeSlice`, not as a production-consumer success. The
successor must first name the sole issuer for a non-physical GenericLoopV1
Recipe/Join product and its exact relation to the existing composer.
