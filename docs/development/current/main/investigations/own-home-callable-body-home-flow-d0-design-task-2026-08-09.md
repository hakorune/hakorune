---
Status: design stop; NoSafeSlice until Home source issuer exists
Date: 2026-08-09
Parent: `docs/development/current/main/investigations/own-home-callable-body-conformance-evidence-d0-design-task-2026-08-09.md`
Authority: `docs/development/current/main/design/ownership-home-model-ssot.md`
---

# CALLABLE-BODY-HOME-FLOW-D0

## Decision

The bounded Call+Return effect/control I0 is closed.  Home-flow evidence is
the next design boundary, not an implementation row.  The current resolver
has no source-backed Home-flow event issuer, complete ownership grammar, or
CFG-complete ownership-changing witness, so Home-flow remains development
`NoSafeSlice`.

`VerifiedHomeAbiV1` remains declaration-only.  The bounded
`VerifiedQueryBodyHomeFlowEvidenceV1` remains unchanged and must not be
generalized or reused as this issuer.

## Future authority and input

The future private issuer may borrow only:

```text
same owner-tree/shadow traversal source-event inventory
VerifiedResolvedFunctionV1
  BindingRef / declarations / variable refs / regions / exits
VerifiedResolvedBodyShapeInventoryV1
VerifiedSemanticOwnerForestV1
VerifiedHomeAbiV1
```

It must not infer Home flow from `BodyEffectKindV1`, method names, `MirType`,
runtime state, or ownership SSA. `HomeRelationBrandV1` is relation-batch
provenance and is not a Home-flow root identity.

The future receipt needs a private non-`Clone` root token carrying exact
owner/body-root/parser+resolver provenance. A parameter root is identified by
exact `BindingRef` plus declared Home demand; a fresh root is identified by its
exact Create site.

## Event and state vocabulary

```text
events:
  Create(root)
  Consume(root, destination, site)
  Share(root, site)
  End(root, site)
  Forward(root, result, call-edge)
  Escape(root, boundary)

state:
  Available(root)
  Consumed(root, site)
  MaybeConsumed(join provenance)
  Unknown(reason)
```

Ordinary non-owning Handle reads and aliases are not transfer events.
`MaybeConsumed` and `Unknown` cannot produce a Candidate. Share/Forward/Escape
remain reserved until their exact ABI/control boundaries are specified.
Home flow owns `BindingRef -> HomeRoot/state`; it never owns
`BindingRef -> ValueId` or reaching-value SSA.

## Readiness and first future fixture

The first future implementation may be a root-direct linear body only:

```text
one owning local/parameter with exact Home ABI
release root
terminal Unit return
```

It requires one exact `End` event and an `Available -> Consumed` transition,
with no alias or use after release. The current grammar has no landed
Home-demand/take/release source issuer, so no test-only event constructor or
fake ABI may be added now.

If/Loop/BlockExpr/Lambda/capture/Await/QMark/Throw/call transfer,
field/index/container, suspension, branch/backedge, and Maybe joins remain
`NoSafeSlice` until a dedicated source/CFG edge issuer exists.

## Fail-fast boundary

```text
NoSafeSlice:
  Home demand/root issuer absent, source grammar absent, incomplete event
  coverage, unsupported branch/loop/CFG, opaque field/index/container,
  Shared/Forward/Escape boundary absent, or capture/suspend unsupported

Rejected:
  foreign owner/parser/resolver/body root, Home ABI mismatch, duplicate root
  or event, double consume/use-after-consume, alias/partial projection,
  wrong branch/backedge, invalid Maybe join, or invalid Forward/Escape
```

Candidate requires complete event/state/CFG witnesses. No Home-flow field is
added to `VerifiedResolvedFunctionV1`, the carrier, or the general execution
aggregate by this D0. Target, Recipe/CallSlot, FunctionOwner/MIR/Builder,
physical ABI, DropPlan, fallback, and production remain closed.

## Next implementation gate

Do not open Home-flow I0 until the source grammar and Home-demand ABI issuer
can provide the exact linear fixture without a forged constructor. At that
point, add a private `body_home_flow` module and a borrowed receipt only;
the general four-axis public co-seal remains a later row.

## Ordered follow-up tasks

Keep this order shallow and do not skip the missing authority boundary:

```text
1. CALLABLE-BODY-HOME-FLOW-D0
   current design stop; keep Home flow at NoSafeSlice

2. OWN-HOME-SOURCE-EVENT-D0
   decide release/take/owning-root grammar and the parser/source event
   authority; no body Home-flow implementation yet

3. OWN-HOME-ABI-HOME-DEMAND-I0
   issue the exact owning parameter/local root capability only after the
   source contract is accepted; do not infer demand from body effects

4. CALLABLE-BODY-HOME-FLOW-LINEAR-I0
   private `body_home_flow` issuer for one root-direct linear fixture:
   Available -> End/Consumed -> terminal Unit return

5. CALLABLE-BODY-HOME-FLOW-CFG-D0
   define branch, loop, backedge, Maybe-join, and transfer authority before
   admitting non-linear ownership flow

6. CALLABLE-BODY-EXECUTION-COSEAL
   only after the four axis issuers are complete; co-seal existing receipts
   without inventing Home/effect/control meaning in the aggregate
```

The first two follow-up rows are design/authority rows. No Home-flow source
event, fake ABI, public aggregate, target, Recipe/CallSlot, FunctionOwner,
Builder, MIR, DropPlan, fallback, or production route may be added while this
card remains the current design stop.
