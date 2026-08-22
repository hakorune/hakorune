---
Status: D0 accepted; next caller-zero source-aware Facts issuer P0
Task: MIR-CALLABLE-LOOP-SOURCE-FACTS-ISSUER-P0
Date: 2026-08-22
Priority: carry one source-bound GenericLoop Facts/Recipe outcome before any consumer
Parent: MIR-CALLABLE-PROGRAM-REGION-CONTAINMENT-P0
PreviousCard: mirbuilder-callable-physical-header-completion-value-d0-2026-08-22
NextCard: MIR-CALLABLE-LOOP-SOURCE-FACTS-ISSUER-P0 (this rolling card)
---

# Callable Loop source-aware Facts issuer D0

## Decision

Adopt one invocation-scoped source-aware GenericLoop Facts issuer at the
existing `PreparedLocatedRawLoopChildEntryV1` boundary. The issuer aggregates
already-owned source and planner products exactly once and returns a private,
move-only source-located outcome. It does not extend `LoopRouteContext` and it
does not enter the current retry-capable registry route.

```text
RawInvocationChildPortV1
  -> exact located parent/condition/body source contexts
  -> grouped Ready callable coverage rows
  -> explicit GenericLoopFactsPolicyFrameV1
  -> same AST condition/body view
  -> CallableGenericLoopSourceFactsIssuerV1::issue_once
  -> one existing Facts/Recipe outcome
  -> one front-selected route observation
  -> private source-located terminal/Ready outcome
```

The existing GenericLoop Facts extractor remains the authority for GenericLoop
shape, Recipe construction, and the final `BodyLoweringPolicy`. The new issuer
only co-seals that result with source identity and route terminality. It must
not infer a carrier, policy, route, or binding role itself.

## Six-line brief

```text
Decision: use one source-aware issuer at PreparedLocatedRawLoopChildEntryV1; keep LoopRouteContext structural and do not add an optional callable field.
Source authority + canonical issuer: CallableSemanticLoweringState/CallableLoopSourceProjection own grouped rows; RawInvocationSourceContext owns location/lineage; the private issuer co-seals those with one explicit-policy Facts/Recipe extraction and route selection.
Non-authority: AST-only LoopRouteContext, GenericLoopAdmissionObservationV1, Builder/ValueId, names/ordinals/pointers, cloned Facts/Recipe, and the existing raw registry continuation.
Fail-fast boundary: source ownership, Ready coverage, policy frame, one Facts extraction, final policy, and exact front-selected route close before lower_loop_or_freeze_v1 or any Builder effect; every reject is terminal with no retry.
Smallest next slice: caller-zero source-aware issuer P0 for one non-nested Ready cohort; issue the private aggregate and prove extraction count=1 without wiring a production consumer.
Non-claims: Outside admission, ordinary consumer, route cutover, PostEffectRetryDebt removal, physical/publication work, fallback/retry, performance, main integration, and nested-loop support.
```

## Authority map

| Owner | Owns | Must not own |
| --- | --- | --- |
| `CallableSemanticLoweringState` | callable owner, exact source read/rebind ledger, binding identity | GenericLoop shape, route selection, physical lowering |
| `CallableLoopSourceProjectionV1` | grouped `binding + class + (site, role)` rows and Ready/Outside classification | AST/Recipe reconstruction, ValueId, route policy |
| `RawInvocationSourceContextV1` | exact invocation root, Loop site, condition/body child lineage | binding semantics, Facts, route choice |
| `GenericLoopFactsPolicyFrameV1` | one captured strict/debug/planner/step policy frame | loop meaning, source identity |
| GenericLoop Facts extractor | one GenericLoop extraction, Recipe, final body policy | callable source pairing, terminal route |
| `RecipeFirstRouteSelectionV1` | deterministic selection from the retained Facts outcome | source scan, fallback authorization |
| `CallableGenericLoopSourceFactsIssuerV1` | one aggregate/co-seal and one terminal/Ready disposition | new binding meaning, second extraction, physical effect |
| existing `LoopRouteContext` | borrowed structural AST view for a later consumer | callable source authority or optional semantic lane |
| `CallableGenericLoopSourceFactsReadyV1` | private transport of the co-sealed outcome | ordinary lowering, route retry, publication |

The issuer is the only production constructor for the private aggregate. It
receives a policy frame issued at the outer boundary; it does not call
`from_environment()` or recompute route policy downstream. The source rows are
the existing grouped rows from the completed Outside-coverage P0. P0 accepts
only the existing `Ready` schedule; body-only rebind remains terminal Outside.

## Required co-sealed input

The input is invocation-scoped and non-reconstructible from a later AST walk:

```text
owner
root lineage / parser provenance
exact parent Loop source context
exact condition child source context
exact body-root source context
condition AST reference and body AST slice from the same prepared Loop
VerifiedCallableSemanticLoopBindingScheduleV1 (Ready only)
GenericLoopFactsPolicyFrameV1
```

The issuer performs one explicit-policy planner/Facts call that returns the
existing `PlanBuildOutcome` (including its canonical Facts and Recipe contract)
and then one `RecipeFirstRouteSelectionV1` observation from that retained
outcome. It must not call the lower-level GenericLoop extractor separately
before or after the planner outcome. The final policy in the outcome is
transported verbatim.

The source-located aggregate is non-`Clone` and has no parallel `Option` fields
for required evidence. Its required shape is a private contract sketch:

```text
CallableGenericLoopSourceFactsReadyV1
  = owner + exact source contexts + Ready rows
  + PlanBuildOutcome
  + RecipeFirstRouteSelectionV1
  + VerifiedLocatedGenericLoopV1SelectionV1
```

The `VerifiedLocatedGenericLoopV1SelectionV1` seal is issued only when the
retained route list is exactly `[GenericLoopV1]`. Any overlapping or different
route is a typed terminal in this bounded slice; it cannot advance to the
legacy suffix. `PlanBuildOutcome` remains the existing Facts/Recipe authority,
not a second source product.

## Finite states

| State | Issuer | Meaning | Effects | Allowed next step |
| --- | --- | --- | ---: | --- |
| `SourceUnavailable` | source-entry validator | missing/foreign root, child, owner, or lineage | 0 | typed terminal |
| `Outside` | callable source projection | complete source observation outside Ready cohort | 0 | existing Outside terminal |
| `FactsAbsent` | one-shot issuer | one explicit extraction returns no Facts | 0 | typed terminal; no retry |
| `FactsRejected` | one-shot issuer | extraction/policy validation error or deferred condition | 0 | typed terminal; no fallback |
| `RouteNotFrontSelected` | route verifier | retained selection is not exactly GenericLoopV1 | 0 | typed terminal |
| `Ready` | private source issuer | all source/Facts/Recipe/route evidence co-sealed | 0 | later named terminal consumer only |
| `Consumed` | future named consumer | Ready aggregate moved once into the normalizer seam | later slice | no second route |
| `PostEffectRetryDebt` | legacy registry | old retry-capable route outcome | forbidden for source-aware lane | never reachable |

`FactsAbsent` is not `SourceUnavailable`; an extraction returning `None` is a
complete observation with no candidate, not missing source. `FactsRejected` is
not `FactsAbsent`; an error/deferred outcome cannot be represented by an empty
Facts product. No state is represented by `Option::None`, a default catalog, or
an empty route that grants continuation.

## Fail-fast and route rule

All of these must finish before `lower_loop_or_freeze_v1`,
`PlanLowerer`, `MirBuilder`, callable ledger consumption, or any module draft
effect:

```text
same invocation/root lineage
exact parent/condition/body ownership
Ready grouped coverage validation
explicit policy frame identity
one Facts/Recipe extraction
final policy retained from that extraction
exact GenericLoopV1-only front selection
```

The current `RouteExecutionWitnessV1::execute_selected_in_order` is not a
consumer for this product: its `PostEffectRetryDebt` and suffix advancement
remain inaccessible. A source-aware Ready result must not be passed to
`lower_loop_or_freeze_v1`, because that route would rebuild Facts from an
AST-only `LoopRouteContext` and could re-enter the old schedule.

## Ordered tasks

1. `MIR-CALLABLE-LOOP-SOURCE-FACTS-ISSUER-P0` — caller-zero foundation.
   Add the private input/aggregate and the sole issuer around an explicit
   policy-aware planner call. Prove one extraction, same-invocation source
   ownership, exact GenericLoop-only selection, Ready move-only shape, and
   typed terminal states. Do not call it from the production raw Loop port.
2. `MIR-CALLABLE-LOOP-GENERIC-TERMINAL-PORT-P0` — caller-zero route seam.
   Add one terminal port that consumes the Ready aggregate without exposing
   `PostEffectRetryDebt`, legacy suffix, fallback, or retry.
3. `MIR-CALLABLE-LOOP-ORDINARY-READY-PORT-P0` — one non-nested Ready fixture.
   Consume exact callable rows during the existing normalizer traversal while
   structural expression lookup stays on its existing port.
4. `MIR-CALLABLE-LOOP-BODY-ONLY-REBIND-I0` — first Outside-derived cohort.
   Extend the named consumer only after the source relation is complete; do
   not relabel body-only rows as the existing Ready carrier.
5. `MIR-CALLABLE-LOOP-ORDINARY-BRIDGE-R0` — production cutover.
   Require one named caller, old bypass caller-zero, no retry/fallback, and
   the merged probe beyond the current Outside terminal.

## Acceptance and reusable guards

Positive:

```text
one non-nested Ready fixture -> one issuer call -> one extraction -> Ready
explicit policy survives unchanged into the aggregate
source owner/lineage/parent/condition/body all match
selection is exactly [GenericLoopV1]
Ready aggregate cannot be cloned or consumed twice
existing non-callable route behavior is unchanged
```

Negative/no-effect:

```text
foreign owner or parser lineage
foreign/missing/duplicate child source context
Outside rows passed as Ready
Facts extraction None
Facts extraction error/deferred
policy frame drift or ambient policy reread
GenericLoop overlap or non-front selection
attempt to enter lower_loop_or_freeze_v1 or registry suffix
```

Every negative must show zero Builder/ledger/route effects. The reusable guard
must enforce issuer call site=1, explicit policy input, extraction call count=1,
no `LoopRouteContext` callable field, no `GenericLoopAdmissionObservationV1`
authority, no AST/name/ordinal/ValueId pairing, no `PostEffectRetryDebt` path,
and production caller=0 for P0. New production files stay below 760 lines and
the 800-line hard boundary.

## NoSafeSlice

Return to design stop if any implementation requires:

```text
constructing the policy frame inside the issuer from ambient environment
calling try_build_outcome and a lower-level GenericLoop extractor separately
rebuilding source rows from cloned Facts, names, ordinals, pointers, or ValueId
adding an optional callable relation to LoopRouteContext
passing Ready into the existing retry-capable registry witness
mapping FactsAbsent to SourceUnavailable or a default/empty Facts product
consuming any source row before route terminality is fixed
admitting body-only rebind as Ready
inventing a second GenericLoop/Recipe authority
```

This D0 is accepted by the top-down review and the source-boundary audit. The
next authorized work is the caller-zero P0 only; no ordinary consumer or
production switch is implied by this acceptance.
