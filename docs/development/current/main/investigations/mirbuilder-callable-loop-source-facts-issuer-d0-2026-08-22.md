---
Status: D0 accepted after worker audit; route-neutral planner I0 complete; Ready claim I0 complete; structural lease I0 complete; next normalizer consumer remains design_stop
Task: MIR-CALLABLE-LOOP-ORDINARY-READY-D0
Date: 2026-08-22
Priority: carry one source-bound GenericLoop Facts/Recipe outcome before any consumer
Parent: MIR-CALLABLE-PROGRAM-REGION-CONTAINMENT-P0
PreviousCard: mirbuilder-callable-physical-header-completion-value-d0-2026-08-22
NextCard: MIR-CALLABLE-LOOP-READY-NORMALIZER-CONSUMER-D0
---

# Callable Loop source-aware Facts issuer D0

## Decision

Adopt one invocation-scoped source-aware GenericLoop Facts issuer at the
existing `PreparedLocatedRawLoopChildEntryV1` boundary, with a strict split
between source planning and later structural traversal:

1. The source-aware issuer receives a route-neutral planner input and performs
   the one Facts/Recipe extraction and one exact route selection. It must not
   construct `LoopRouteContext`, call `choose_route_kind`, reread ambient
   route policy, or enter the registry.
2. The later ordinary normalizer may borrow one already-existing structural
   traversal view through a private HRTB callback. That view is never stored in
   the source product and is not a second source/Facts authority.

The issuer aggregates already-owned source and planner products exactly once
and returns a private, move-only source-located outcome. The current
terminal-only `Ready -> Consumed` experiment is not the eventual ordinary
state model: the production connection slice must transform the existing
`Ready` product in place into one claimable source-facts product and remove
the terminal consumer in the same slice. No parallel source-facts authority is
allowed.

```text
RawInvocationChildPortV1
  -> exact located parent/condition/body source contexts
  -> grouped Ready callable coverage rows
  -> explicit GenericLoopFactsPolicyFrameV1
  -> route-neutral CallableLoopFactsPlannerInputV1
  -> CallableGenericLoopSourceFactsIssuerV1::issue_once
  -> one existing Facts/Recipe outcome
  -> one front-selected route observation
  -> private source-located source-facts outcome
  -> one existing structural normalizer port (HRTB, later)
```

The existing GenericLoop Facts extractor remains the authority for GenericLoop
shape, Recipe construction, and the final `BodyLoweringPolicy`. The new issuer
only co-seals that result with source identity and route terminality. It must
not infer a carrier, policy, route, or binding role itself.

## Six-line brief

```text
Decision: use one route-neutral source-aware issuer at PreparedLocatedRawLoopChildEntryV1, then one scoped HRTB handoff to an existing structural normalizer traversal; never create a second context.
Source authority + canonical issuer: CallableSemanticLoweringState/CallableLoopSourceProjection own grouped rows; RawInvocationSourceContext owns location/lineage; CallableGenericLoopSourceFactsIssuerV1 co-seals one route-neutral Facts/Recipe outcome and exact selection.
Non-authority: LoopRouteContext construction inside the source issuer, choose_route_kind, GenericLoopAdmissionObservationV1, Builder/ValueId, names/ordinals/pointers, cloned Facts/Recipe, registry continuation, and AST-only re-pairing.
Fail-fast boundary: source ownership, Ready coverage, captured policy, one route-neutral Facts extraction, exact front-selected route, and the structural HRTB handoff close before any Builder effect; Ready never returns to lower_loop_or_freeze_v1.
Smallest next slice: design and accept the route-neutral planner input plus one existing structural-port HRTB relation; only then implement the in-place claimable source-facts consumer.
Non-claims: ordinary physical lowering, route cutover, PostEffectRetryDebt removal, publication, fallback/retry, performance, parser witness strengthening, body-only rebind admission, and nested-loop support.
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
| `CallableGenericLoopSourceFactsIssuerV1` | one route-neutral aggregate/co-seal and one exact selection | new binding meaning, second extraction, physical effect, structural route construction |
| existing `LoopRouteContext` | one borrowed structural view owned by the existing normalizer traversal | source Facts authority, source pairing, optional callable semantic lane |
| `CallableGenericLoopSourceFactsV1` (eventual in-place name) | one claimable source/Facts/Recipe product | ordinary route rebuild, route retry, publication |

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
AST and source-context parts. The current 11-element `into_parts()` tuple is
not an acceptable long-term boundary: the next slice must replace it with one
opaque `CallableGenericLoopSourceFactsInputV1` (or a payload method that issues
that input) owned by the source-facts module. No caller may receive parallel
AST/source/schedule pieces and re-pair them.

The issuer performs one route-neutral planner/Facts call through a bounded
input such as:

```text
CallableLoopFactsPlannerInputV1<'a>
  = condition: &'a ASTNode
  + body: &'a [ASTNode]
  + captured GenericLoopFactsPolicyFrameV1
```

`single_planner::try_build_source_outcome(input)` must not construct a
`LoopRouteContext`, call `choose_route_kind`, apply a function-name
whitelist, run canonicalizer parity, reread ambient environment, or consult a
runtime registry. If a diagnostic label or static-box bit is genuinely needed
by Facts semantics, it must be an explicit member of an already-owned policy
frame; it may not re-enter a route context as an implicit authority. The call
returns the existing `PlanBuildOutcome` (including its canonical Facts and
Recipe contract), followed by one `RecipeFirstRouteSelectionV1` observation
from that retained outcome. It must not call a lower-level GenericLoop
extractor separately before or after the planner outcome. The final policy in
the outcome is transported verbatim.

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
| `Ready` / eventual `SourceFacts` | private source issuer | all source/Facts/Recipe/route evidence co-sealed | 0 | one `claim_all()` move |
| `Claimed` | named source-facts consumer | the same product is consumed once into a scoped receipt/view | 0 | normalizer/plan seam only |
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
one existing structural traversal owner identified for the later HRTB view
```

The current `RouteExecutionWitnessV1::execute_selected_in_order` is not a
consumer for this product: its `PostEffectRetryDebt` and suffix advancement
remain inaccessible. A source-aware Ready result must not be passed to
`lower_loop_or_freeze_v1`, because that route would rebuild Facts from an
AST-only `LoopRouteContext` and could re-enter the old schedule. The existing
`LoopRouteContext` may be borrowed only by the later named normalizer callback;
the source issuer itself must never construct one.

## Ordered tasks

1. `MIR-CALLABLE-LOOP-SOURCE-FACTS-ISSUER-P0` — caller-zero foundation
   (complete). Preserve the existing evidence as the baseline, but do not
   treat its current `LoopRouteContext::new` call or 11-element tuple as the
   accepted design.
2. `MIR-CALLABLE-LOOP-GENERIC-TERMINAL-PORT-P0` — caller-zero terminal seam
   (complete as an experiment). It is not the ordinary consumer and must not
   become a second source-facts authority. Its `ConsumedV1`/terminal product
   is retired in the same production connection slice as the in-place source
   facts claim model.
3. `MIR-CALLABLE-LOOP-READY-PLANNER-D0` — route-neutral planner contract
   (accepted). Name and audit
   `CallableLoopFactsPlannerInputV1` and
   `single_planner::try_build_source_outcome`. Remove
   `LoopRouteContext::new`, `choose_route_kind`, function-name whitelist,
   canonicalizer parity, ambient environment reread, and registry access from
   the source-aware issuer. Keep diagnostics separate from Facts authority and
   make any semantic static-box input explicit in the captured policy frame.
   `MIR-CALLABLE-LOOP-READY-PLANNER-I0` is the current bounded implementation:
   add the input type, route-neutral kernel adapter, source-issuer call, and
   reusable guard only. It must remain caller-zero and must not touch the raw
   Ready route.
4. `MIR-CALLABLE-LOOP-READY-STRUCTURAL-HANDOFF-D0` — one existing traversal
   owner and private HRTB handoff (current design subcell). Preserve the
   `CallableSemanticLoopHandoffPreEffectReceiptV1` instead of discarding it;
   move it into an opaque structural handoff such as
   `PreparedCallableLoopStructuralHandoffV1`. The existing normalizer
   traversal may lend one structural view for a callback, but the handoff and
   source product hold no `LoopRouteContext`, AST pointer, Builder, or
   `ValueId`, and no borrow escapes the callback.
   Worker audit accepted the owner/lease split above. The bounded next task is
   design-only contract/guard work; no caller-zero implementation may enter
   `route_loop`.
5. `MIR-CALLABLE-LOOP-READY-SOURCE-SHAPE-D0` — close the aggregate boundary
   before implementation. Replace the 11-element `into_parts()` escape with
   one source-facts-owned opaque input or a typed payload-to-input method. In
   parallel, separate observed binding rows from admitted Ready classes;
   replace the discarded Outside-branch `Verified...ScheduleV1::seal` with a
   private `validate_ready_remainder`, and retain `Outside` as a typed reason
   until the outermost raw API stringifies it.
6. `MIR-CALLABLE-LOOP-ORDINARY-READY-CLAIM-P0` — in-place source-facts claim,
   caller-zero. Transform the existing `ReadyV1` shape into the one eventual
   `CallableGenericLoopSourceFactsV1` product and add one `claim_all()` move
   into a private receipt/view. Do not add a parallel Facts type; do not use
   `ConsumedV1` as the normalizer handoff; do not connect Builder, registry,
   physical lowering, or production route yet.
   The bounded implementation name is
   `MIR-CALLABLE-LOOP-READY-CLAIM-I0`; it is a caller-zero type/receipt slice,
   not the later ordinary consumer or production bridge.
7. `MIR-CALLABLE-LOOP-ORDINARY-READY-PORT-P0` — one non-nested named
   normalizer consumer. Consume the claimed product exactly once through the
   existing structural HRTB port, with no Facts/Recipe/selection rerun and no
   `route_loop` continuation. Add focused positive/negative/zero-effect
   evidence; production raw caller remains closed until the later bridge.
8. `MIR-CALLABLE-LOOP-BODY-ONLY-REBIND-I0` — first Outside-derived cohort.
   Extend the named consumer only after the source relation is complete; do
   not relabel body-only rows as the existing Ready carrier.
9. `MIR-CALLABLE-LOOP-ORDINARY-BRIDGE-R0` — production cutover. Require one
   named caller, old `lower_loop_or_freeze_v1`/`try_cf_loop_joinir` Ready
   callers at zero, no retry/fallback, preserved receipt through the lowerer,
   and the merged probe beyond the current Outside terminal.
10. `MIR-LOOP-COMPARE-TRANSACTION-HARDENING-D0` — parked separate lane.
    Cursor EOF, pre-effect claims, prepare-before-reserve, affine destination,
    co-sealed commit, and OuterReturn/Header-current checks must not be mixed
    into the Callable Ready handoff.

## Acceptance and reusable guards

Positive:

```text
one non-nested Ready fixture -> one route-neutral issuer call -> one extraction -> Ready/SourceFacts
explicit policy survives unchanged into the aggregate
source owner/lineage/parent/condition/body all match
selection is exactly [GenericLoopV1]
Ready/SourceFacts aggregate cannot be cloned or consumed twice
one existing structural traversal lends one HRTB view
the HRTB callback result cannot carry an AST/source borrow
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
source-aware issuer constructs LoopRouteContext
source-aware issuer calls choose_route_kind, registry, or route whitelist
Ready enters lower_loop_or_freeze_v1, cf_loop_joinir, or route_loop
pre-effect receipt is dropped before the named consumer
attempt to enter lower_loop_or_freeze_v1 or registry suffix
```

The explicit-policy planner path also passes the captured frame through the
GenericLoop V0 extractor and body validator. Their legacy environment wrappers
remain only for legacy/test callers; the source-aware issuer does not enter
those wrappers.

Every negative must show zero Builder/ledger/route effects. The reusable guard
must enforce issuer call site=1, route-neutral planner invocation=1,
explicit policy input, extraction call count=1, `LoopRouteContext::new` inside
the source issuer=0, `choose_route_kind` inside the source issuer=0, no
`GenericLoopAdmissionObservationV1` authority, no AST/name/ordinal/ValueId
pairing, no `PostEffectRetryDebt` path, no dropped pre-effect receipt, and
production caller=0 for P0. New production files stay below 760 lines and the
800-line hard boundary.

## NoSafeSlice

Return to design stop if any implementation requires:

```text
constructing the policy frame inside the issuer from ambient environment
constructing `LoopRouteContext` inside the source-aware issuer
using a second `LoopRouteContext` instead of one existing structural port
letting a source-aware Ready route into `route_loop` after the HRTB callback
calling try_build_outcome and a lower-level GenericLoop extractor separately
rebuilding source rows from cloned Facts, names, ordinals, pointers, or ValueId
escaping an 11-element tuple that lets callers re-pair AST/source/schedule rows
issuing `VerifiedCallableSemanticLoopBindingScheduleV1` only to validate and drop an Outside remainder
classifying Outside body-only rebind rows as an admitted Ready Carrier
stringifying Outside before the outermost raw boundary
claiming parser-invocation identity without an opaque parser witness
adding an optional callable relation to LoopRouteContext
passing Ready into the existing retry-capable registry witness
mapping FactsAbsent to SourceUnavailable or a default/empty Facts product
consuming any source row before route terminality is fixed
admitting body-only rebind as Ready
inventing a second GenericLoop/Recipe authority
```

The route-neutral planner portion of this D0 is accepted by the worker audit
and is the bounded I0 below. The remaining structural-owner/HRTB relation and
in-place source-facts state model remain at design_stop. The terminal-only P0
below is complete as a caller-zero experiment; it does not authorize an
ordinary consumer or a production switch.

## Route-neutral planner I0 (2026-08-22, complete)

Decision: implement only the accepted planner subcell. The source issuer gets
one `CallableLoopFactsPlannerInputV1` containing borrowed `condition/body`, the
captured `GenericLoopFactsPolicyFrameV1`, and diagnostic-only function name /
debug values. `single_planner::try_build_source_outcome` shares the existing
planner kernel with the Context adapter; it does not classify a route or enter
the registry.

```text
source issuer
  -> route-neutral planner input
  -> PlannerContext::from_generic_loop_policy
  -> build_plan_with_facts_ctx(condition, body)
  -> one RecipeMatcher observation
  -> existing PlanBuildOutcome
```

Acceptance for this I0:

```text
source issuer LoopRouteContext construction = 0
source issuer choose_route_kind/registry access = 0
route-neutral planner kernel = 1
Facts/Recipe extraction is not duplicated
existing Context callers retain the same adapter/kernel behavior
source Ready -> lower_loop_or_freeze_v1/route_loop new edge = 0
Builder/ledger/physical/publication effect = 0
```

The source issuer's current `in_static_box` value is deliberately not copied
into this planner input: the audited Facts/Recipe kernel does not use it. If a
future semantic rule needs it, that is a separate policy-authority Decision;
it must not be recovered by constructing `LoopRouteContext` here.

This I0 does not claim the Ready production handoff. The next design cell is
the existing structural owner and private HRTB transport, including the
currently discarded pre-effect receipt.

Evidence recorded for the route-neutral planner I0:

```text
CARGO_BUILD_JOBS=4 cargo check --profile quick --lib                         PASS
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib \\
  normal_callable_loop_source_facts                                      4 passed
bash tools/checks/rust_mirbuilder_callable_loop_source_facts_issuer_p0_guard.sh PASS
bash tools/checks/rust_mirbuilder_callable_loop_generic_facts_policy_p0_guard.sh PASS
bash tools/checks/current_state_pointer_guard.sh                             PASS
rustfmt --edition 2021 --check (changed Rust files)                          PASS
git diff --check                                                             PASS
```

## Caller-zero P0 implementation receipt (2026-08-22)

The caller-zero foundation is integrated into `main` at merge commit
`aafda19d86`; this is not a production switch. The route-neutral planner I0
above supersedes the earlier context-construction shape.

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
not reread the environment. This is the landed caller-zero baseline plus the
route-neutral planner I0 above. The issuer has no production caller, no Builder
or ledger effect, no registry suffix, fallback, retry, or parser-invocation
identity claim. The bounded identity claim remains the same prepared raw-root
lineage only.

Evidence recorded for this slice:

```text
CARGO_BUILD_JOBS=4 cargo check --profile quick --lib                 PASS
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib \\
  normal_callable_loop_source_facts -- --nocapture                  4 passed
bash tools/checks/rust_mirbuilder_callable_loop_source_facts_issuer_p0_guard.sh PASS
bash tools/checks/rust_mirbuilder_callable_loop_generic_facts_policy_p0_guard.sh PASS
bash tools/checks/rust_mirbuilder_callable_loop_outside_disposition_p0_guard.sh PASS
bash tools/checks/current_state_pointer_guard.sh                     PASS
git diff --check                                                     PASS
```

The next authorized design cell is
`MIR-CALLABLE-LOOP-READY-STRUCTURAL-HANDOFF-D0`: fix the ownership of the existing
structural traversal and name the scoped HRTB handoff. The implementation P0
must remain closed until that relation is accepted. Body-only rebind admission,
route cutover, fallback/retry, physical/publication work and parser witness
strengthening remain closed.

## Ordinary Ready consumer D0 (revised after top-down worker audit)

### Six-line brief

```text
Decision: split the source-aware route-neutral Facts issuer from the one existing structural normalizer traversal; use one private HRTB handoff and one in-place claimable source-facts product.
Source authority + canonical issuer: CallableLoopSourceProjectionV1 and the existing Facts/Recipe authorities remain owners; CallableGenericLoopSourceFactsIssuerV1 is the sole source co-seal issuer; the structural traversal is only a borrowed consumer port.
Non-authority: LoopRouteContext construction inside the source issuer, choose_route_kind, AST-only re-planning, GenericLoopAdmissionObservationV1, Builder/ValueId, registry/RouteExecutionWitness, names, ordinals, pointers, cloned Facts, and the terminal ConsumedV1 product as a parallel lane.
Fail-fast boundary: route-neutral Facts/Recipe/selection, exact source rows, preserved pre-effect receipt, and the existing structural-port relation close before the HRTB callback; Ready never returns to lower_loop_or_freeze_v1 or route_loop.
Smallest next slice: accept the planner input, structural handoff, opaque aggregate, Outside typing, and in-place claim_all state model; only then implement caller-zero claim/normalizer evidence.
Non-claims: no physical lowering, PlanLowerer commit, registry, ledger, route cutover, fallback/retry, publication, body-only rebind admission, nested loops, parser witness strengthening, or production switch.
```

### Verified current call graph and blocker

The P0 finding is present in the code, not only in the card:

```text
RawInvocationChildPortV1::lower_loop
  -> PreparedLocatedRawLoopChildEntryV1::lower_with_existing_route_v1
  -> handoff.consume_pre_effect(...)        // receipt is discarded
  -> lower_loop_or_freeze_v1(condition, body)
  -> cf_loop_joinir_impl
  -> LoopRouteContext::new/with_fn_body
  -> route_loop
  -> Facts/Recipe/route observation again
```

The source-aware issuer is currently caller-zero, and its `issue_once` also
constructs a `LoopRouteContext` before the later ordinary path constructs one.
Therefore the current graph is `NoSafeSlice` for Ready production use: the
source product is not connected to its normalizer, and the old route remains a
second observation path. The dropped receipt is a concrete ownership hole, not
an authorization to pass Ready into the old route.

### Accepted design boundary

The source issuer gets a route-neutral input and never constructs a structural
route context:

```rust
struct CallableLoopFactsPlannerInputV1<'a> {
    condition: &'a ASTNode,
    body: &'a [ASTNode],
    policy: GenericLoopFactsPolicyFrameV1,
}

single_planner::try_build_source_outcome(input)
```

This entry is the only source-aware planner call. It must not call
`LoopRouteContext::new`, `choose_route_kind`, a function-name whitelist,
canonicalizer parity, an ambient environment lookup, or the runtime registry.
Any diagnostic label is separate. If static-box state is semantic input rather
than diagnostic state, it must be made explicit in the captured policy frame;
it may not be recovered by constructing a context.

The later normalizer handoff uses one existing structural traversal owner. The
minimum transport is a private, move-only product in a new sibling module:

```rust
struct PreparedCallableLoopStructuralHandoffV1<'source> {
    parent_source: &'source RawInvocationSourceContextV1,
    condition_source: RawInvocationSourceContextV1,
    body_source: RawInvocationSourceContextV1,
    pre_effect: CallableSemanticLoopHandoffPreEffectReceiptV1,
    policy: GenericLoopFactsPolicyFrameV1,
}
```

It holds no AST, `LoopRouteContext`, `Builder`, `ValueId`, registry witness, or
physical receipt. A private callback may borrow the existing structural view
and the retained source-facts product for one scope:

```text
CallableGenericLoopSourceFactsNormalizerConsumerV1::consume_in_existing_port(
    source_facts,
    structural_port,
    use_view: impl for<'view> FnOnce(
        CallableGenericLoopSourceFactsReadyViewV1<'view, 'source>
    ) -> R,
) -> Result<R, CallableLoopReadyHandoffErrorV1>
```

### Worker-audited structural handoff decision (2026-08-22)

The read-only Aristotle audit changed no repository files. It compared the
ordinary boundary with `PreparedSourceBackedDynamicLoopIngressV1` and rejected
reusing that Dynamic physical ingress as an ordinary semantic consumer.

The existing structural owner is fixed as the context construction already
owned by `cf_loop_joinir_impl`. It creates one `LoopRouteContext` for the
ordinary traversal, but that context is not itself a source-facts handoff and
must never be stored in the source product. `route_loop` is not a consumer for
the source-aware product: it re-enters Facts/Recipe observation, the registry,
and Builder lowering.

The structural boundary is a private opaque lease minted immediately before
the ordinary `route_loop` edge:

```rust
with_existing_structural_port<R>(
    ctx: &LoopRouteContext<'_>,
    use_port: impl for<'view> FnOnce(
        CallableLoopStructuralPortV1<'view>,
    ) -> R,
) -> R
```

The helper belongs to the existing routing owner. `CallableLoopStructuralPortV1`
has no public fields or conversion from AST/name/ordinal/`ValueId`; it exposes
only the narrow structural inputs needed by the later normalizer. It does not
expose `route_kind`, Facts/Recipe APIs, registry access, Builder, or physical
receipts. The `for<'view> FnOnce` contract makes the borrow callback-scoped:
the returned `R` cannot carry the structural borrow.

The source product instead moves its source evidence into one private handoff:

```text
CallableGenericLoopSourceFactsV1
  -> claim_all()
  -> CallableGenericLoopSourceFactsReceiptV1
       + CallableSemanticLoopHandoffPreEffectReceiptV1
  -> PreparedCallableLoopStructuralHandoffV1
  -> with_existing_structural_port(..., |port| ...)
```

`claim_all()` is the only place that consumes the existing schedule's
`consume_pre_effect(...)`. It must retain that receipt in the move-only
source-facts receipt; assigning it to `_pre_effect_receipt` is forbidden. The
structural port is borrowed only inside the callback and is never co-sealed
into the source product. A source-aware callback returns before the old
`route_loop` edge; it does not call `route_loop` or
`lower_loop_or_freeze_v1`.

This decision is accepted for design/task purposes only. It does not authorize
the Ready production consumer, physical lowering, registry, fallback, or a
production switch. If a future implementation can only pass
`&LoopRouteContext` directly, construct a second context, or continue into
`route_loop`, return to `NoSafeSlice`.

### Structural handoff finite state

| State | Owner | Effects | Allowed next step |
| --- | --- | ---: | --- |
| `Ready` | source-facts issuer | 0 | one `claim_all()` move |
| `Claimed` | source-facts consumer | 0 | prepare one structural handoff |
| `HandoffPrepared` | private handoff owner | 0 | one HRTB lease callback |
| `BorrowedStructuralView` | routing owner | 0 | callback return only |
| `RejectedBeforeEffect` | typed handoff validator | 0 | terminal discard |
| `Consumed` | future named normalizer | 0 | later plan/lower decision; not this slice |

`BorrowedStructuralView` is not a storable product. The callback cannot return
the view, source AST borrow, or a `LoopRouteContext` reference through `R`.

### `MIR-CALLABLE-LOOP-STRUCTURAL-LEASE-I0` (complete fast BoxShape slice)

Change: add one private `CallableLoopStructuralPortV1` and one
`with_existing_structural_port` HRTB callback at the existing
`cf_loop_joinir_impl` Context owner. Keep the source claim receipt move-only;
do not connect the Ready product to the old route.

Contract: the port is a callback-scoped borrowed structural view with private
fields and no `Deref`; it exposes no route kind, Facts/Recipe, registry,
PlanLowerer, Builder, `ValueId`, AST, or physical receipt. The source-aware
issuer and terminal consumer remain caller-zero. This is a BoxShape seam, not
a new semantic accepted shape.

Done: focused callback-scope evidence, one reusable lane guard, source and
module README receipt, all touched Rust files below 800 lines, and no
Ready-scoped caller to `route_loop`, `lower_loop_or_freeze_v1`, registry, or
`PlanLowerer`.

Stop: return to design if the lease needs a second `LoopRouteContext`, exposes
AST/route/Facts authority, lets the borrow escape the callback, pairs a foreign
source receipt, or requires any physical/production effect.

Evidence for this slice: `structural_port` focused test 1 passed; the existing
source/Facts lane guard and current-state pointer guard pass; the new lease
module is 46 lines and its focused test is 20 lines. No production caller,
route continuation, or physical effect was added.

Closeout: `MIR-CALLABLE-LOOP-READY-NORMALIZER-CONSUMER-D0` is the next design
frontier. It must name the consumer owner and exact source-receipt/structural
port relation before any normalizer, PlanLowerer, registry, Builder, or
production edge is opened.

### `MIR-CALLABLE-LOOP-READY-CLAIM-I0` (complete)

This bounded slice implements only the already-accepted source-facts state
transition. It renames the current `CallableGenericLoopSourceFactsReadyV1`
in place to `CallableGenericLoopSourceFactsV1` and adds exactly one private
`claim_all(self)` operation. The operation moves the existing schedule through
`consume_pre_effect(...)` and returns one non-`Clone`
`CallableGenericLoopSourceFactsReceiptV1` containing that receipt plus the
existing source contexts, AST slice, `PlanBuildOutcome`, policy, and exact
selection. It issues no new semantic fact.

Acceptance:

```text
Ready variant stores CallableGenericLoopSourceFactsV1
claim_all caller count = 1 in focused test only
pre-effect receipt is a required receipt field, never `_pre_effect_receipt`
claim failure is typed and occurs before any Builder/ledger effect
receipt cannot be cloned or claimed twice
terminal ConsumedV1 remains historical only and is not the structural handoff
old raw lower_loop_or_freeze_v1 edge is unchanged and explicitly not claimed
as fixed by this caller-zero slice
```

Evidence recorded for this I0:

```text
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib normal_callable_loop_source_facts  PASS (5 passed, 0 failed)
bash tools/checks/rust_mirbuilder_callable_loop_source_facts_issuer_p0_guard.sh  PASS
bash tools/checks/current_state_pointer_guard.sh                              PASS
python3 TOML parse check                                                     PASS
rustfmt --edition 2021 on touched Rust files                                  PASS
git diff --check                                                              PASS
```

The focused test proves that the claimed pre-effect receipt is retained in
the non-`Clone` receipt and that the claim path itself performs no physical
effect. The source guard proves the in-place type name, one private claim
operation, and the absence of the discarded `_pre_effect_receipt` shape in
the issuer. This is caller-zero evidence, not a production-route claim.

Non-claims: no `CallableLoopStructuralPortV1` implementation, no raw-route
connection, no `route_loop`/registry continuation, no Builder/physical/
publication effect, no fallback/retry change, and no production switch. The
next design stop reopens after this I0 to implement the opaque HRTB lease.

### In-place source-facts state and shape cleanup

The eventual source state is one product, not `Ready + Facts + Consumed` in
parallel:

```text
CallableGenericLoopSourceFactsReadyV1   // current name only
  -> CallableGenericLoopSourceFactsV1   // in-place production name
  -> claim_all() exactly once
  -> CallableGenericLoopSourceFactsReceiptV1 / scoped view
```

The current `CallableGenericLoopSourceFactsConsumedV1` and
`CallableGenericLoopSourceFactsTerminalConsumerV1` remain only as caller-zero
historical evidence until the production connection slice, where they are
removed together. They are never reused as the ordinary handoff and no second
source-facts type is introduced.

The 11-element `Prepared...Payload::into_parts()` must become one opaque
`CallableGenericLoopSourceFactsInputV1` or a typed payload-to-input method.
The caller must not receive parallel AST, source, owner, schedule, and policy
values and re-pair them.

Outside observation is also distinct from Ready admission:

```text
CallableLoopObservedBindingRowV1 { binding, receipts }
CallableLoopReadyBindingClassV1 { Carrier, ReadOnlyOperand, IterationLocal }
CallableLoopOutsideRowV1 { observed, kind: BodyOnlyRebind }
```

Body-only rebind remains typed Outside and must not be labeled `Carrier` merely
because it has a rebind. The Outside branch must validate its Ready remainder
with a private `validate_ready_remainder(...) -> Result<(), ...>`; it must not
issue and immediately drop a `Verified...ScheduleV1`. Its structured reason
stays typed through the raw boundary and becomes `String` only at the outermost
API.

### D0 acceptance / NoSafeSlice

Accept this D0 only when one contract names all of the following:

```text
one route-neutral planner input and one source-aware planner call
zero LoopRouteContext construction inside the source issuer
one existing structural traversal owner
one private HRTB/scoped consumer boundary
one preserved pre-effect receipt; zero discard at the Ready boundary
Ready moves once and is never cloned or paired through a tuple
Facts/Recipe/selection extraction counts remain one
no AST/source borrow escapes the callback
no route_loop/lower_loop_or_freeze_v1 continuation after Ready handoff
no terminal consumer, registry, retry, fallback, Builder, ledger, or publication edge
```

Return to `NoSafeSlice` if the only handoff requires a new
`LoopRouteContext` from Ready, an AST pointer/name/ordinal pairing, a parallel
optional source-facts product, a second Facts/Recipe extraction, dropped source
evidence, or any Builder effect in this row. A green test or the existing
terminal-only consume does not waive this stop.

## Terminal-only Ready consumption P0 (historical caller-zero experiment, 2026-08-22)

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

This experiment is not the ordinary Ready port and is not a production
handoff. The eventual in-place source-facts claim slice retires this
`ConsumedV1`/terminal consumer rather than reusing it. No physical or
production edge is opened until the revised D0 contract above is accepted.
