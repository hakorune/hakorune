---
Status: D0, caller-zero issuer P0, and terminal-only port P0 complete; ordinary Ready consumer D0 is design_stop
Task: MIR-CALLABLE-LOOP-ORDINARY-READY-D0
Date: 2026-08-22
Priority: carry one source-bound GenericLoop Facts/Recipe outcome before any consumer
Parent: MIR-CALLABLE-PROGRAM-REGION-CONTAINMENT-P0
PreviousCard: mirbuilder-callable-physical-header-completion-value-d0-2026-08-22
NextCard: MIR-CALLABLE-LOOP-ORDINARY-READY-PORT-P0
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
Non-claims: Outside admission, ordinary consumer, route cutover, PostEffectRetryDebt removal, physical/publication work, fallback/retry, performance, production switch, and nested-loop support.
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
root lineage from the prepared raw invocation
exact parent Loop source context
exact condition child source context
exact body-root source context
condition AST reference and body AST slice from the same prepared Loop
VerifiedCallableSemanticLoopBindingScheduleV1 (Ready only)
GenericLoopFactsPolicyFrameV1
```

This bounded raw port does not yet carry an opaque parser-invocation witness.
Therefore P0 claims same prepared raw-root lineage only; it must not claim
parser-invocation identity from path, name, digest, ordinal, or AST pointer.
Adding that stronger identity is a separate NoSafeSlice/Design-stop condition.

The raw child entry moves these fields into one private
`PreparedCallableGenericLoopSourceFactsPayloadV1`. The issuer accepts that
aggregate only; there is no public/peer API that accepts independently supplied
AST and source-context parts.

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
same prepared raw-root lineage
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

1. `MIR-CALLABLE-LOOP-SOURCE-FACTS-ISSUER-P0` — caller-zero foundation
   (complete).
   Add the private input/aggregate and the sole issuer around an explicit
   policy-aware planner call. Prove one extraction, same prepared raw-root source
   ownership, exact GenericLoop-only selection, Ready move-only shape, and
   typed terminal states. Do not call it from the production raw Loop port.
2. `MIR-CALLABLE-LOOP-GENERIC-TERMINAL-PORT-P0` — caller-zero route seam
   (complete). Add one terminal port that consumes the Ready aggregate without
   exposing `PostEffectRetryDebt`, legacy suffix, fallback, or retry.
3. `MIR-CALLABLE-LOOP-ORDINARY-READY-D0` — fix the structural-port ownership
   and HRTB handoff before adding an ordinary consumer. The current issuer
   creates a `LoopRouteContext` and drops it; the later ordinary path creates
   another one. This row stays at design_stop until one traversal owns the
   structural view and passes it into the named consumer without AST/Facts/
   Recipe re-observation.
4. `MIR-CALLABLE-LOOP-ORDINARY-READY-PORT-P0` — one non-nested Ready fixture.
   Add the scoped normalizer consumer after D0 is accepted. Consume exact
   callable rows during the existing normalizer traversal while structural
   expression lookup stays on its existing port.
5. `MIR-CALLABLE-LOOP-BODY-ONLY-REBIND-I0` — first Outside-derived cohort.
   Extend the named consumer only after the source relation is complete; do
   not relabel body-only rows as the existing Ready carrier.
6. `MIR-CALLABLE-LOOP-ORDINARY-BRIDGE-R0` — production cutover.
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
foreign owner or raw-root lineage
foreign/missing/duplicate child source context
Outside rows passed as Ready
Facts extraction None
Facts extraction error/deferred
policy frame drift or ambient policy reread
GenericLoop overlap or non-front selection
attempt to enter lower_loop_or_freeze_v1 or registry suffix
```

The explicit-policy planner path also passes the captured frame through the
GenericLoop V0 extractor and body validator. Their legacy environment wrappers
remain only for legacy/test callers; the source-aware issuer does not enter
those wrappers.

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
claiming parser-invocation identity without an opaque parser witness
adding an optional callable relation to LoopRouteContext
passing Ready into the existing retry-capable registry witness
mapping FactsAbsent to SourceUnavailable or a default/empty Facts product
consuming any source row before route terminality is fixed
admitting body-only rebind as Ready
inventing a second GenericLoop/Recipe authority
```

This D0 is accepted by the top-down review and the source-boundary audit. The
terminal-only P0 below is now complete; no ordinary consumer or production
switch is implied by that completion.

## Caller-zero P0 implementation receipt (2026-08-22)

The caller-zero foundation is integrated into `main` at merge commit
`aafda19d86`; this is not a production switch.

```text
PreparedLocatedRawLoopChildEntryV1
  -> private PreparedCallableGenericLoopSourceFactsPayloadV1
  -> CallableGenericLoopSourceFactsIssuerV1::issue_once
  -> one explicit-policy planner/Facts outcome
  -> one retained RecipeFirstRouteSelectionV1
  -> exact [GenericLoopV1] Ready or typed terminal
```

The payload is move-only and has no constructor that accepts independently
supplied AST/source-context parts. The explicit policy frame is passed through
the planner, GenericLoop V0 extraction, and body validator; the issuer does
not reread the environment. The issuer has no production caller, no Builder
or ledger effect, no registry suffix, fallback, retry, or parser-invocation
identity claim. The bounded identity claim remains the same prepared raw-root
lineage only.

Evidence recorded for this slice:

```text
CARGO_BUILD_JOBS=4 cargo check --profile quick --lib                 PASS
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib \\
  normal_callable_loop_source_facts -- --nocapture                  3 passed
bash tools/checks/rust_mirbuilder_callable_loop_source_facts_issuer_p0_guard.sh PASS
bash tools/checks/rust_mirbuilder_callable_loop_generic_facts_policy_p0_guard.sh PASS
bash tools/checks/rust_mirbuilder_callable_loop_outside_disposition_p0_guard.sh PASS
bash tools/checks/current_state_pointer_guard.sh                     PASS
git diff --check                                                     PASS
```

The next authorized design cell is
`MIR-CALLABLE-LOOP-ORDINARY-READY-D0`: fix the ownership of the existing
structural traversal and name the scoped HRTB handoff. The implementation P0
must remain closed until that relation is accepted. Body-only rebind admission,
route cutover, fallback/retry, physical/publication work and parser witness
strengthening remain closed.

## Ordinary Ready consumer D0 (2026-08-22)

### Six-line brief

```text
Decision: keep Ready as the only source/Facts/Recipe product and add one scoped HRTB normalizer consumer; do not reuse the terminal consumer or add a callable field to LoopRouteContext.
Source authority + canonical issuer: CallableLoopSourceProjectionV1 and the existing Facts/Recipe authorities remain owners; CallableGenericLoopSourceFactsIssuerV1::issue_once is the sole co-seal issuer; CallableGenericLoopSourceFactsNormalizerConsumerV1::consume is only a consumer.
Non-authority: AST-only contexts rebuilt after Ready, GenericLoopAdmissionObservationV1, Builder/ValueId, registry/RouteExecutionWitness, PostEffectRetryDebt, names, ordinals, pointers, cloned Facts, and a second Recipe/route selection.
Fail-fast boundary: exact Ready, owner/lineage/child coverage, policy, retained Facts/Recipe, and exact [GenericLoopV1] selection must be closed before the HRTB callback; the callback must receive one existing structural traversal port and must not construct a new context or enter Builder.
Smallest next slice: D0 fixes the structural-port ownership and callback shape; only after acceptance does P0 add one non-nested caller-zero normalizer consumer in a new sibling module.
Non-claims: no physical lowering, PlanLowerer, registry, ledger, route cutover, fallback/retry, publication, body-only rebind, nested loops, parser witness strengthening, or production switch.
```

### Why the extra design stop is required

`CallableGenericLoopSourceFactsIssuerV1::issue_once` currently creates a
`LoopRouteContext` only to run the one explicit-policy Facts/Recipe extraction,
then drops that structural view. The ordinary path later constructs another
context inside `cf_loop_joinir_impl`. Passing `Ready` into that later route
would therefore allow a second structural observation even if Facts and the
route selection themselves were not recomputed.

The HRTB boundary must close this gap without making `LoopRouteContext` a
callable semantic owner and without storing a self-referential borrow in
`CallableGenericLoopSourceFactsReadyV1`. The existing traversal must lend its
structural view to the consumer for one callback scope. The callback may see
the retained Facts/Recipe, exact selection seal, source schedule, and the
already-located condition/body relation, but it may not return an AST or source
borrow whose lifetime escapes the scope.

The intended named API is:

```text
CallableGenericLoopSourceFactsNormalizerConsumerV1::consume(
    Ready,
    existing_structural_port,
    use_normalizer_input,
) -> R
```

The exact `existing_structural_port` type and its construction owner are the
D0 decision. It must be borrowed from the same normalizer traversal, not
reconstructed from `Ready`, an AST pointer, a route name, or a second
`LoopRouteContext::new` call. If this cannot be expressed without one of
those forbidden pairings, the row remains `NoSafeSlice` and no P0 code is
allowed.

### Authority and handoff shape

The P0 consumer must receive one private aggregate from the existing issuer,
not independently supplied fragments:

```text
Ready
  = exact source contexts + condition/body
  + retained PlanBuildOutcome
  + retained RecipeFirstRouteSelectionV1
  + VerifiedLocatedGenericLoopV1SelectionV1
  + Ready callable schedule
```

The scoped normalizer input may borrow those fields together with the one
existing structural port. It must not issue new Facts, clone or reconstruct
`GenericLoopV1Facts`, rerun `select_recipe_first_routes`, or call the terminal
consumer. The callback result must not carry the borrowed AST/source view
outside the HRTB scope.

The normalizer consumer is therefore not a second authority and not a
production bridge. It only proves that the source-bound Ready product can
reach the existing normalizer seam exactly once, with no Builder/ledger/
registry effect. Physical lowering and production selection remain a later R0.

### Ordered design and implementation tasks

1. `MIR-CALLABLE-LOOP-ORDINARY-READY-D0` (current, design_stop)

   Identify the one existing structural traversal that owns the ordinary
   normalizer view. Fix the handoff as a private scoped callback/HRTB API.
   Record the exact input/output relation, lifetime rule, and the proof that
   no second `LoopRouteContext` or AST walk is introduced.

2. `MIR-CALLABLE-LOOP-ORDINARY-READY-PORT-P0` (blocked until D0 accepted)

   Add `normal_callable_loop_ready_normalizer.rs` with exactly one named
   `CallableGenericLoopSourceFactsNormalizerConsumerV1::consume`. Keep the
   new file below 760 lines and leave `raw_loop_child_entry.rs` and
   `normal_callable_loop_handoff.rs` untouched. Add one non-nested positive
   fixture and typed negative cases; do not connect the production raw port.

3. `MIR-CALLABLE-LOOP-ORDINARY-BRIDGE-R0` (parked)

   Only after the P0 consumer is proven may a later card decide whether the
   existing normalizer/PlanLowerer can be connected. That card must separately
   own Builder effects, route cutover, no-fallback/no-retry, and old caller-zero
   evidence.

### D0 acceptance / NoSafeSlice

Accept D0 only when all of these are written as one contract:

```text
one named existing structural traversal owner
one private HRTB/scoped consumer boundary
Ready is moved once and never cloned
Facts/Recipe/selection extraction counts remain one
no AST/source borrow escapes the callback
no second LoopRouteContext construction or structural walk
no terminal consumer, registry, retry, fallback, Builder, ledger, or publication edge
```

Return to `NoSafeSlice` if the only available handoff requires a new
`LoopRouteContext` from Ready, an AST pointer/name/ordinal pairing, a parallel
optional product, a second Facts/Recipe extraction, or any Builder effect in
this row. A green test or a terminal-only consume does not waive this stop.

## Terminal-only Ready consumption P0 (2026-08-22)

Decision: represent the terminal port as a move-only state transition, not as
a second Facts/Recipe or registry terminality authority.

```text
CallableGenericLoopSourceFactsReadyV1
  -> CallableGenericLoopSourceFactsTerminalConsumerV1::consume
  -> CallableGenericLoopSourceFactsConsumedV1
```

The existing source-aware issuer remains the sole authority for owner, source
lineage, Facts/Recipe, and exact `[GenericLoopV1]` selection. The named
terminal consumer only consumes that already-sealed Ready product. It retains
the existing source schedule and exact selection seal, while intentionally
dropping AST/Facts/Recipe at the no-effect terminal. It does not expose a
`RouteExecutionWitnessV1`, registry suffix, `PostEffectRetryDebt`, Builder,
ledger, or physical receipt.

The consumed type is non-`Clone` and has no constructor or conversion from an
AST, `LoopRouteContext`, route list, name, ordinal, `ValueId`, or physical
product. The transition is infallible because every fallible source/Facts/
route check is complete before `Ready` exists. There is no production caller
in this P0; the focused test is the only consumer.

Acceptance for this slice:

```text
Ready -> exactly one Consumed move
owner and exact GenericLoopV1 seal are retained
AST/Facts/Recipe cannot be observed through Consumed
registry/lowering/Builder/ledger edge count = 0
production terminal-consumer caller count = 0
```

The next authorized row is the ordinary Ready port. It must not reuse the
terminal state, and it must first name the normalizer consumer and its
source-backed row relation before any physical or production edge is opened.
