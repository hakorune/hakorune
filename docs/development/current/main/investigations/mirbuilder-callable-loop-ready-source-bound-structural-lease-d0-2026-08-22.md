---
Status: design_stop; NoSafeSlice accepted after worker audit
Task: MIR-CALLABLE-LOOP-READY-SOURCE-BOUND-STRUCTURAL-LEASE-D0
Date: 2026-08-22
Priority: create the missing source-lineage/structural-owner relation before any Ready consumer or production edge
Parent: MIR-CALLABLE-LOOP-READY-NORMALIZER-CONSUMER-D0
PreviousCard: mirbuilder-callable-loop-ready-normalizer-consumer-d0-2026-08-22.md
NextCard: none-until-Decision
---

# Callable Loop Ready source-bound structural lease D0

## Six-line brief

Decision: the current diagnostic-only structural port is insufficient. Design
one route-neutral structural owner that move-receives the already-claimed
source-facts receipt and issues one private, callback-scoped handoff. This is
the missing authority relation, not a new Facts/Recipe or physical product.

Source authority + canonical issuer: `CallableGenericLoopSourceFactsIssuerV1`
and its `CallableGenericLoopSourceFactsReceiptV1` remain the source authority.
The new private lease issuer is owned by the existing `cf_loop_joinir_impl`
structural owner, but must be separated from `LoopRouteContext::new`,
`choose_route_kind`, and `route_loop`.

Non-authority: the current `CallableLoopStructuralPortV1` by itself,
`LoopRouteContext`, `route_loop`, registry selection, `PlanLowerer`, AST/name/
ordinal/pointer/digest pairing, Builder state, `ValueId`, and the old raw
Ready route.

Fail-fast boundary: before lending the callback view, co-seal the receipt's
owner, parent/condition/body sites, parser/source lineage, exact selected
route seal, and retained pre-effect receipt. Any missing or foreign relation
rejects with effect 0; it must not continue to the old route.

Smallest next slice: design and document one opaque
`PreparedCallableLoopStructuralHandoffV1` transport aggregate and one
`CallableLoopReadyViewV1<'view, 'source>` HRTB callback boundary. No
production caller, normalizer, PlanLowerer, registry, Builder, or physical
effect is opened in this D0.

Non-claims: normalizer consumption, CorePlan, route selection, physical
lowering, publication, raw Ready cutover, fallback/retry, body-only rebind,
nested loops, and production switch.

## Why the previous consumer D0 stopped

The exact census on `main` at `0cc92c3703` is:

| Surface | Production callers | Test callers | Meaning |
| --- | ---: | ---: | --- |
| `CallableGenericLoopSourceFactsIssuerV1::issue_once` | 0 | 5 | issuer is caller-zero |
| `into_callable_generic_loop_source_facts_payload` | 0 | 5 | only the method definition is production code |
| `CallableGenericLoopSourceFactsV1::claim_all` | 0 | 1 | receipt claim is test-only |
| `with_existing_structural_port` | 0 | 1 | current port has no production consumer |
| `lower_with_existing_route_v1` | 1 | 0 | raw invocation enters old route |
| `cf_loop_joinir_impl` | 1 | 0 | old route owner is reached separately |

The live raw invocation path is currently:

```text
RawInvocationChildPortV1
  -> PreparedLocatedRawLoopChildEntryV1
  -> Ready pre-effect is consumed into a discarded local
  -> lower_loop_or_freeze_v1
  -> cf_loop_joinir_impl
  -> LoopRouteContext::new/with_fn_body
  -> route_loop
```

The source-facts receipt never reaches `cf_loop_joinir_impl`. The current
structural port contains only diagnostic label/debug state, so passing it
beside a receipt would allow a foreign pair. `route_loop` also re-runs
`try_build_outcome`, registry selection, and route preflight, so it cannot be
the source-aware consumer.

This is a real missing owner, not a reason to add an adapter. The old raw edge
must remain explicitly unclaimed until a later production-ingress Decision;
the new lease D0 must not silently turn it into fallback.

## Authority map

| Owner | May own | Must not own |
| --- | --- | --- |
| `CallableGenericLoopSourceFactsIssuerV1` | source contexts, Facts/Recipe outcome, exact selection, pre-effect claim | structural route classification, Builder, physical lowering |
| source-facts receipt | one move-only co-sealed source/planner observation | independent AST reconstruction or re-pairing |
| route-neutral structural lease issuer | same-owner relation and callback-scoped transport handoff | Facts/Recipe/registry/route selection or physical meaning |
| `CallableLoopStructuralPortV1` | borrowed diagnostic structural view | source identity, route kind, AST, Builder, `ValueId` |
| `cf_loop_joinir_impl` owner | existing structural context/port lifetime | second semantic issuer or old-route fallback |
| future normalizer consumer | consume the handoff exactly once | source re-observation, registry, physical effect in this D0 |

The lease aggregate may only co-seal existing receipts. It must not invent a
new `Verified*` or `Prepared*` semantic fact. If the aggregate needs a source
lineage witness that does not already exist in the receipt, stop and name that
missing issuer instead of using a pointer, name, ordinal, digest, or AST
equality check.

## Required route-neutral shape

The future design must provide a structural input that does not execute the
old route classifier:

```text
source receipt
  -> exact already-owned condition/body source
  -> route-neutral structural owner
  -> private handoff aggregate
  -> HRTB (receipt view, structural port)
```

`LoopRouteContext::new` is not an acceptable constructor for this path because
it calls `choose_route_kind`. `with_fn_body` is not acceptable if it first
delegates to that classifier. A route-neutral shape may borrow only the
structural inputs needed by the later callback; it must not expose a route
decision or reclassify the loop.

The design must answer, with existing source-owned fields:

1. Which exact owner receives the move-only receipt?
2. Which existing source lineage/identity proves that the structural view is
   the same loop, without AST/name/ordinal/pointer pairing?
3. Which private issuer creates the handoff aggregate?
4. Which callback receives the receipt view and port, and why can neither
   borrow escape?
5. What typed reject represents a foreign/missing/contradictory relation?

If any answer requires a production fallback or a second source walk, retain
`NoSafeSlice`.

## Proposed type/transition sketch

This is design vocabulary only; it is not implementation authorization:

```rust
struct PreparedCallableLoopStructuralHandoffV1<'source> {
    // move-only existing receipt; no new semantic fields
    receipt: CallableGenericLoopSourceFactsReceiptV1<'source>,
    // private same-owner relation and structural lease seal
    _seal: CallableLoopStructuralHandoffSealV1,
}

struct CallableLoopReadyViewV1<'view, 'source> {
    // borrowed, opaque views of the existing receipt and port
    _source: PhantomData<&'source ()>,
    _view: PhantomData<&'view ()>,
}

fn with_source_bound_ready_view<R>(
    handoff: PreparedCallableLoopStructuralHandoffV1<'_>,
    use_view: impl for<'view, 'source> FnOnce(
        CallableLoopReadyViewV1<'view, 'source>,
    ) -> R,
) -> R;
```

The actual fields and visibility must be resolved before code. The receipt
must not be destructured into parallel public arguments. `PhantomData` is
illustrative only; do not use it to hide an absent source-lineage witness.

Finite state:

| State | Owner | Effect | Next |
| --- | --- | ---: | --- |
| `ClaimedReceipt` | source-facts owner | 0 | relation validation |
| `RelationReady` | lease issuer | 0 | handoff aggregate |
| `HandoffPrepared` | structural owner | 0 | one HRTB callback |
| `BorrowedReadyView` | callback consumer | 0 | callback return only |
| `RejectedBeforeEffect` | typed validator | 0 | terminal discard |
| `Consumed` | future named consumer | 0 | later D0; not this slice |

No state permits `route_loop`, `lower_loop_or_freeze_v1`, registry, or
Builder mutation.

## Acceptance / guards for this D0

```text
source receipt -> lease issuer relation = named once
route-neutral structural owner = 1
LoopRouteContext::new/with_fn_body in lease path = 0
choose_route_kind in lease path = 0
route_loop in lease path = 0
Facts/Recipe/selection rerun = 0
source AST/name/ordinal/pointer pairing = 0
pre_effect discard = 0
callback borrow escape = 0
Builder/ValueId/physical effect = 0
production caller added = 0
```

The design evidence must include a counterexample for a foreign receipt and a
missing/contradictory source site. A green caller-zero test alone is not an
authority proof.

## Exit / NoSafeSlice

Exit `design_stop` only when the source lineage relation, exact private issuer,
route-neutral owner, HRTB lifetime boundary, typed reject vocabulary, and
caller-zero guard are all written down. Then change the mode to `fast` only
for the bounded BoxShape implementation of this lease.

Remain in `design_stop` if the only available relation is:

```text
same AST address
same function name
same parser ordinal
same digest/path
same route kind
same independently-built LoopRouteContext
```

This D0 deliberately does not solve the production ingress. After it closes,
the next separate Decision must connect the raw Ready source to the named
consumer and retire the old route edge atomically; no silent fallback is
allowed.
