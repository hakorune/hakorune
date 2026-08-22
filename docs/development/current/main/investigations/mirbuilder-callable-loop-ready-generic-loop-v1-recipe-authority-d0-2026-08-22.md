---
Status: Decision accepted; implementation is opened at route-neutral context S0
Task: MIR-CALLABLE-LOOP-READY-GENERIC-LOOP-V1-RECIPE-AUTHORITY-D0
Date: 2026-08-22
Priority: separate semantic GenericLoopV1 Recipe/Join issuance from the physical composer
Parent: MIR-CALLABLE-LOOP-READY-PRODUCTION-INGRESS-D0
PreviousCard: mirbuilder-callable-loop-ready-production-ingress-d0-2026-08-22.md
NextCard: MIR-CALLABLE-LOOP-READY-GENERIC-LOOP-V1-ROUTE-NEUTRAL-CONTEXT-S0 (this card)
---

# Callable Loop Ready GenericLoopV1 Recipe authority D0

## Six-line brief

Decision: accepted. Before connecting production `Ready`, define one
source-bound, non-physical GenericLoopV1 Recipe/Join product. It co-seals the
existing claimed source receipt, canonical Facts, exact `GenericLoopV1`
selection, and retained pre-effect relation, while issuing no `CorePlan`,
`ValueId`, `BasicBlockId`, Builder mutation, or `LoopRouteContext`
classification.

Source authority + canonical issuer: the existing
`CallableGenericLoopSourceFactsIssuerV1::issue_once` and `claim_all` remain the
source/Facts authority. A new issuer is allowed only if this D0 names it as
the sole source-bound GenericLoopV1 Recipe authority and proves it consumes
those existing products without a second Facts/Recipe/route observation.

Non-authority: `PlanBuildOutcome` by itself, `RecipeFirstRouteSelectionV1` by
itself, `VerifiedLocatedGenericLoopV1SelectionV1` by itself, the old
`RecipeComposer::compose_generic_loop_v1_recipe`, `LoopRouteContext`, registry
selection, AST/name/ordinal/pointer pairing, `CorePlan`, `ValueId`,
`BasicBlockId`, Builder state, and empty/default `RecipeContract` values.

Fail-fast boundary: before any physical composer or Builder method. Reject if
Facts are absent, the selection is not exactly GenericLoopV1, source/pre-effect
lineage is foreign, the semantic Recipe issuer cannot prove its complete
cohort, or the downstream physical composer would need to re-observe route
meaning. Rejection is terminal; it cannot return to the old Ready route.

Smallest next slice: first perform the behavior-neutral route-neutral context
split, then implement the private semantic Recipe authority and its named
physical adapter for one non-nested cohort. The same implementation series
must connect `Ready`, remove the old Ready edge, and retain zero fallback.

Non-claims: production Ready ingress, ordinary Loop lowering, `CorePlan`
construction, `PlanLowerer`, CFG/SSA/physical IDs, nested-loop expansion,
Outside ordinary consumption, fallback/retry retirement, production switch,
and performance.

## Why the previous ingress D0 stopped

The only close existing producer is:

```text
RecipeComposer::compose_generic_loop_v1_recipe(
    &mut MirBuilder,
    &CanonicalLoopFacts,
    &LoopRouteContext,
) -> LoweredRecipe/CorePlan
```

It allocates the loop skeleton and invokes `generic_loop_pipeline`; the
result contains physical `ValueId`/`BasicBlockId` state. It is therefore a
physical composer, not a consumer of a source-bound semantic receipt.

The separate `VerifiedLocatedCoreLoopPlanV1` is not a shortcut. It requires a
`VerifiedCallableResultActivationPlanV1`, caller ledger, and
`LegacyStmtInputV1` that the current raw callable source receipt does not own.

The current source Facts path provides:

```text
CallableGenericLoopSourceFactsReceiptV1
  - source parent/condition/body contexts
  - retained pre-effect receipt
  - PlanBuildOutcome { CanonicalLoopFacts, recipe_contract: None }
  - RecipeFirstRouteSelectionV1
  - VerifiedLocatedGenericLoopV1SelectionV1
```

This proves a selected GenericLoopV1 route, but it is not yet a semantic
Recipe/Join artifact that a physical composer can consume without reopening
route authority.

## Required authority decision

The D0 must answer all of these before implementation:

1. **What is the semantic Recipe product?** Decide whether the existing
   `CanonicalLoopFacts`/`GenericLoopV1Facts` can be moved or borrowed inside a
   source-bound aggregate, or whether a new closed product is necessary. It
   must not contain `CorePlan`, `LoweredRecipe`, `ValueId`, `BasicBlockId`, or
   `MirBuilder`.
2. **Who issues it?** Name exactly one private issuer. It may consume the
   claimed source receipt and verify the exact selected route; it may not call
   `try_build_outcome`, `try_build_source_outcome`, registry selection, or
   `LoopRouteContext::new` again.
3. **What relation is co-sealed?** Prove owner, loop site, source lineage,
   pre-effect receipt, Facts cohort, and exact GenericLoopV1 selection belong
   to one callable source invocation. Independent tuple fields or route keys
   are not enough.
4. **How does the physical composer consume it?** Specify a later named
   adapter that receives the semantic product and a Builder effect owner. If
   the existing composer still requires route classification or independently
   reconstructs Facts, identify the exact route-neutral input seam instead of
   hiding an adapter around it.
5. **What is the first cohort?** Prefer one non-nested GenericLoopV1 shape if
   nested lowering would re-enter `LoopRouteContext::new`. Any exclusion must
   be a typed source/Recipe disposition, not a default or silent fallback.
6. **What is the failure transaction?** All source/semantic checks and
   semantic Recipe issuance happen before physical mutation. Later physical
   failures remain in the unpublished function session and do not retry the
   old route.

## Candidate comparison

| Candidate | Decision | Reason |
| --- | --- | --- |
| Reuse `PlanBuildOutcome` + selection directly | reject | They are separate observations, not a co-sealed Recipe/Join product. |
| Pass source receipt into existing `RecipeComposer` | reject | Composer still owns Builder/route-context/physical issuance and can re-observe. |
| Rebuild `CorePlan` from source Facts before lowering | reject | Introduces a second semantic/physical issuer and leaks physical IDs early. |
| Reuse `VerifiedLocatedCoreLoopPlanV1` | reject for this slice | Its activation/caller-ledger/source statement authority is absent from this lane. |
| **New source-bound non-physical GenericLoopV1 Recipe authority** | **candidate** | Keeps source/Facts/selection together and leaves physical IDs to the later composer adapter. |

The candidate is accepted with the bounded shape below. Do not implement a
second product with duplicated `facts`, `selection`, or AST fields merely to
make the old composer compile.

## Accepted Decision

The semantic product is a move-only wrapper around the claimed source receipt;
it does not copy the receipt's Facts, selection, AST, or pre-effect fields.
The sole issuer is:

```rust
CallableGenericLoopV1SemanticRecipeIssuerV1::issue(
    CallableGenericLoopSourceFactsReceiptV1<'source>,
) -> Result<CallableGenericLoopV1SemanticRecipeV1<'source>, Reject>
```

```rust
struct CallableGenericLoopV1SemanticRecipeV1<'source> {
    receipt: CallableGenericLoopSourceFactsReceiptV1<'source>,
    _seal: CallableGenericLoopV1SemanticRecipeSealV1,
}
```

The product is an aggregate of already-issued authority, not a second Facts
extractor. Its private HRTB view may lend only these relations from that one
receipt:

```text
owner and exact loop source site
source/pre-effect lineage evidence
&CanonicalLoopFacts
&GenericLoopV1Facts
&VerifiedLocatedGenericLoopV1SelectionV1
&CallableSemanticLoopHandoffPreEffectReceiptV1
```

The view does not lend an independent AST, route key, registry entry,
`RecipeContract`, `CorePlan`, `ValueId`, `BasicBlockId`, `MirBuilder`, or a
second source tuple. The GenericLoop facts remain the sole semantic syntax
view; the receipt remains the sole owner of the source relation.

The issuer performs only pre-effect validation. It does not call
`try_build_outcome`, `try_build_source_outcome`,
`select_recipe_first_routes`, `LoopRouteContext::new`, `choose_route_kind`,
`route_loop`, or a physical composer. It verifies that the existing outcome
contains exactly one GenericLoopV1 fact, the existing selection is exactly
`[GenericLoopV1]`, the source owner/site/pre-effect relation is intact, and
the first physical cohort is non-nested and has no BlockExpr loop prelude.

The physical boundary is a separate named adapter:

```text
CallableGenericLoopV1SemanticRecipeV1
  -> CallableGenericLoopV1PhysicalAdapterV1
  -> GenericLoopV1LoweringContextV1 (route-neutral)
  -> prepared CorePlan / physical session
```

`GenericLoopV1LoweringContextV1` carries only diagnostic settings and a
source-borrowed syntax view plus an explicit nested policy. It has no
`route_kind` and no route registry. The legacy `LoopRouteContext` remains on
the old lane; the source-aware adapter never constructs it. Nested loops and
BlockExpr loop preludes are typed `UnsupportedFirstCohort` before Builder
effects, rather than being reclassified or routed through a fallback.

This closes the design stop. It does not claim that the physical adapter,
production caller, or old-route retirement is already landed.

## Physical boundary

```text
CallableGenericLoopV1SemanticRecipeV1
  -> one named physical composer adapter
  -> prepared physical plan/effect owner
  -> sole physical writer
```

The adapter must consume only the semantic product and a Builder effect owner.
A constructor that silently calls `choose_route_kind` is not a source-aware
adapter.

## Task sequence

1. **Route-neutral context S0 (BoxShape).** Introduce the narrow
   `GenericLoopV1LoweringContextV1` seam and route the existing GenericLoopV1
   pipeline/body helpers through it without changing legacy behavior. The
   nested callback is an explicit policy: legacy may re-enter its old lane;
   the source-aware policy returns typed `UnsupportedFirstCohort`.
2. **Semantic Recipe I0.** Add the private move-only authority wrapper and
   HRTB view. Its reject cases are FactsAbsent, selection mismatch, foreign
   source/pre-effect, nested first cohort, and BlockExpr prelude. Add the
   named physical adapter in the same bounded series so the product has a
   consumer and cannot become an orphan receipt.
3. **Production Ready cutover R0.** Consume Ready exactly once, issue the
   semantic Recipe exactly once, call the named adapter exactly once, and
   delete the `Ready -> lower_loop_or_freeze_v1` edge in the same series.
   Canonical rejection is terminal and never retries the old route.
4. **Evidence and closeout.** Add positive/negative focused tests, one
   reusable lane guard, module README/reference contract updates, source-size
   checks, and the pointer/commit/push receipt.

## Finite state table

| State | Sole owner | Effect | Allowed next state | Old route |
| --- | --- | --- | --- | --- |
| `Located` | source handoff | none | `Ready` / `Outside` / terminal reject | not entered |
| `ReadyUnclaimed` | source Facts issuer | none | `Claimed` | forbidden |
| `Claimed` | `claim_all` | none | `SemanticRecipeReady` / typed reject | forbidden |
| `SemanticRecipeReady` | semantic Recipe issuer | none | `PhysicalPrepared` | forbidden |
| `UnsupportedFirstCohort` | semantic Recipe issuer | none | terminal discard | forbidden |
| `PhysicalPrepared` | named physical adapter | none | `Lowered` / unpublished failure | forbidden |
| `Lowered` | existing sole physical owner | unpublished MIR only | terminal success | forbidden |
| `RejectedBeforeEffect` | relevant issuer | none | terminal discard | forbidden |

No state uses `Option`, default, or compatibility text to merge absent,
unsupported, and invalid coverage. `Outside` remains the source projection's
terminal state and is not converted into a GenericLoopV1 Recipe.

## Acceptance / guards

```text
semantic Recipe issuer = exactly 1
source Facts extraction = 1
GenericLoopV1 selection = 1
source/owner/pre-effect co-seal = 1
semantic product contains CorePlan/LoweredRecipe = 0
semantic product contains ValueId/BasicBlockId/MirBuilder = 0
new issuer -> LoopRouteContext::new/with_fn_body = 0
new issuer -> choose_route_kind/route_loop/registry = 0
new issuer -> PlanLowerer/physical writer = 0
source-aware physical adapter -> LoopRouteContext::new/with_fn_body = 0
source-aware physical adapter -> choose_route_kind/route_loop/registry = 0
source-aware reject -> old Ready route/fallback/retry = 0
semantic Recipe product has exactly one named consumer
Ready production caller switch = 1
old Ready production caller = 0 after R0
all touched Rust source files < 760 lines
```

Focused evidence must show that the same claimed source receipt reaches the
semantic Recipe authority, that every reject occurs before Builder effect, and
that the later physical adapter cannot be called with a foreign source or
selection product.

## NoSafeSlice conditions

Remain in `design_stop` if any of these are true:

```text
the only Recipe authority is the physical `RecipeComposer`
Facts/selection must be reconstructed from AST or Builder state
the product needs a default/empty RecipeContract to appear complete
CorePlan or physical IDs must be issued before source Recipe authority
the existing composer has no route-neutral input seam
nested-loop handling requires a second LoopRouteContext observation
source receipt and semantic Facts can be independently paired
old Ready route must remain reachable after source-aware rejection
```

If the route-neutral context cannot be introduced without a second source
scan, route classification, or nested re-entry, return to `design_stop` and
split the missing context authority before adding a Recipe type. If the
first-cohort physical adapter needs a `CorePlan`/physical ID before the
semantic product is consumed, return to `NoSafeSlice`; do not move those IDs
upstream or revive the old route.
