---
Status: design_stop; source-bound non-physical Recipe authority is not yet named
Task: MIR-CALLABLE-LOOP-READY-GENERIC-LOOP-V1-RECIPE-AUTHORITY-D0
Date: 2026-08-22
Priority: separate semantic GenericLoopV1 Recipe/Join issuance from the physical composer
Parent: MIR-CALLABLE-LOOP-READY-PRODUCTION-INGRESS-D0
PreviousCard: mirbuilder-callable-loop-ready-production-ingress-d0-2026-08-22.md
NextCard: none-until-Decision
---

# Callable Loop Ready GenericLoopV1 Recipe authority D0

## Six-line brief

Decision: before connecting production `Ready`, define one source-bound,
non-physical GenericLoopV1 Recipe/Join product. It must co-seal the existing
source receipt, canonical Facts, exact `GenericLoopV1` selection, and retained
pre-effect relation, while issuing no `CorePlan`, `ValueId`, `BasicBlockId`,
Builder mutation, or `LoopRouteContext` classification.

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

Smallest next slice: audit the existing GenericLoopV1 Facts and composer
inputs, then choose between reusing an existing semantic product and adding
one narrowly owned non-physical product. Define its move/borrow boundary and
the later physical adapter contract. This D0 authorizes no code or production
caller yet.

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

The candidate is not accepted until its fields and issuer are written down.
Do not implement a speculative `Verified*` type merely to make the old
composer compile.

## Proposed shape for review

This is a design sketch, not implementation authorization:

```rust
struct CallableGenericLoopV1RecipeAuthorityV1<'source> {
    source: CallableGenericLoopSourceFactsReceiptV1<'source>,
    facts: CanonicalLoopFacts,
    selected: VerifiedLocatedGenericLoopV1SelectionV1,
    _seal: CallableGenericLoopV1RecipeAuthoritySealV1,
}

struct CallableGenericLoopV1RecipeIssuerV1;

impl CallableGenericLoopV1RecipeIssuerV1 {
    fn issue(
        receipt: CallableGenericLoopSourceFactsReceiptV1<'_>,
    ) -> Result<CallableGenericLoopV1RecipeAuthorityV1<'_>, Reject>;
}
```

The exact shape may instead borrow `facts` from an owned outcome through an
HRTB view. In either form, the product must not expose source AST as a new
pairing input, and it must not claim that semantic Facts are already physical
MIR.

The later physical boundary should look like:

```text
CallableGenericLoopV1RecipeAuthorityV1
  -> one named physical composer adapter
  -> prepared physical plan/effect owner
  -> sole physical writer
```

The adapter must be designed separately if it needs `LoopRouteContext`. A
constructor that silently calls `choose_route_kind` is not a source-aware
adapter.

## Task sequence

1. **Census existing authority.** Record every field read by
   `RecipeComposer::compose_generic_loop_v1_recipe`,
   `generic_loop_pipeline`, GenericLoopV1 body lowering, and nested-loop
   helpers. Mark each field semantic, source, diagnostic, or physical.
2. **Census re-observation.** Count source Facts extraction, route selection,
   `LoopRouteContext` construction, registry selection, and `PlanLowerer` calls
   that the proposed consumer would trigger. The target is one source Facts
   issuer, one selection issuer, and zero re-observation in the new lane.
3. **Close the product contract.** Select existing `CanonicalLoopFacts`/
   `GenericLoopV1Facts` reuse or name the new non-physical product, with owner,
   source site, selection, pre-effect relation, and typed reject vocabulary.
4. **Close the physical adapter boundary.** Specify how the existing composer
   receives the accepted semantic product without receiving independent AST,
   route, or source keys. If this requires a route-neutral lowering context,
   make that a separate named authority in the same Decision.
5. **Add guards and evidence plan.** Cover natural GenericLoopV1, absent Facts,
   non-Generic selection, foreign source, duplicate claim, second issuer,
   route-context reclassification, physical-ID escape, and old-route fallback.
6. **Only then open implementation.** Change `work_mode` to `fast` with one
   bounded production edge. Keep the current caller-zero guards until the
   same slice has a named consumer and old Ready edge is zero.

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
source-aware reject -> old Ready route/fallback/retry = 0
production callers remain 0 until Decision acceptance
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

The production ingress D0 remains closed as NoSafeSlice until this card has an
accepted Decision. This is the smallest honest blocker: one missing semantic
Recipe authority, not a reason to widen the Builder or add another fallback.
