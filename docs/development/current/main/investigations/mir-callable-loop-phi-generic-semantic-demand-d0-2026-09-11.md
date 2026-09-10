---
Status: accepted design; bounded Callable profile selected for implementation
Date: 2026-09-11
Decision: MIR-CALLABLE-LOOP-PHI-GENERIC-SEMANTIC-DEMAND-D0
Parent: mir-callable-loop-phi-generic-physical-demand-d0-2026-09-11
---

# MIR-CALLABLE-LOOP-PHI-GENERIC-SEMANTIC-DEMAND-D0

## Six-line brief

```text
Decision: choose the existing source-bound CallableSingleLoop product as the
  first semantic-demand profile; keep the broad GenericLoopV1 route on its
  existing compatibility consumer until that product is complete.
Source authority + canonical issuer: the selected CallableSingleLoop
  syntax-facts/source-map/Recipe issuers own the claimed source facts and
  BindingRef/site rows; the existing semantic-program/common-demand chain
  owns operation/effect/After co-sealing; CanonicalSsaFunctionSessionV2 owns
  all physical ValueId/CFG/PHI/seal issuance.
Non-authority: GenericLoopV1Facts by itself, PlanBuildOutcome, pre_effect rows,
  names, variable_map, old RecipeComposer, CorePhiInfo, composer-local maps,
  latest values, and Generic G0's source product.
Fail-fast boundary: before any Builder effect, reject unsupported expression
  shape, missing operation/value/effect/placement/After relation, owner or
  source-site drift, and incomplete binding coverage; never retry the old
  composer for a selected canonical candidate.
Smallest next slice: pass the selected callback's borrowed resolver input and
  source ledger into the existing CallableSingleLoop syntax→map→Recipe
  co-seal, then consume its common operation/effect/After demand once.
Non-claims: no broad GenericLoopV1 promotion, nested/If/BlockExpr/call/effect
  support, local-completion closure, session/OBJ/EXE, or legacy retirement.
```

### Census boundary

```text
CallableSingleLoop syntax-facts/source-map/Recipe issuers
  -> one bounded Callable semantic-demand issuer
  -> VerifiedLoopOperationPhysicalDemandV1::issue/prepare_all;
includes the selected CallableSingleLoop source profile and its
operation/value/effect/After relations; excludes broad GenericLoopV1,
GenericLoopV0, Generic G0, DirectAccum, legacy Composer, nested/Outside
routes, function session emission, and backend artifacts.
```

## Why the successor is required

The preceding physical-demand audit refuted direct reuse. The current
`GenericLoopV1Facts` contains the condition, increment, body, policy, and step
placement, while `CallableSemanticLoopHandoffPreEffectReceiptV1` contains only
condition/body read and rebind BindingRef rows. Neither owns a complete
operation item/value graph, source effect anchors, block placement, or JoinSig
After binding. `PlanBuildOutcome.recipe_contract` is only structural and is
normally absent for this source path.

The existing neutral demand is intentionally stricter: it consumes a complete
`VerifiedLoopCoreProductV1`, a complete operation/effect product, and a moved
`VerifiedLoopContinuationContractV1`. Generic G0 already supplies those parts,
but its source forest, window lease, and operation issuer belong to another
source family. Pairing them with the callable Generic Recipe would create a
second authority and is rejected. The existing CallableSingleLoop product,
by contrast, already co-seals the exact source map, operation relations,
Prelude/Tail boundary, and common demand. It is selected as a distinct
Callable profile, never by relabeling it as `GenericLoopV1`.

## Resolver-context handoff audit

The selected callable ingress already owns the metadata needed to bind this
product: `ResolvedFunctionLoweringInputV1` and its
`CallableSemanticSourceLedgerView` expose the owner, function origin,
source-kind, exact Loop membership, execution frame, and scope/region. The
current raw path does not retain that ingress; `RawInvocationSourceContextV1`
and `RawInvocationRootLineageV1` provide location/lineage only and cannot issue
resolver metadata.

The bounded producer must therefore run inside the selected callback while the
input and ledger are borrowed, and pass them directly to the one semantic-demand
issuer. It must not store a session or semantic context on
`RawInvocationChildPortV1`, rebuild metadata from a name/path, or perform a
second source scan. If that callback-borrow handoff cannot carry the exact
source rows through one co-seal, the cohort remains
`NoSafeSlice__GenericLoopResolverContextUnavailable`.

## First-cohort candidate

The first canonical candidate is the already-issued CallableSingleLoop
profile. It is deliberately narrower than the observed GenericLoopV1 grammar:

```text
one non-nested loop
no BlockExpr loop prelude
numeric progression with an exact, fixed step placement
one carrier BindingRef with condition-read, body-read, and body-rebind rows
condition and body are the exact fixed-I64 source rows already owned by
  `VerifiedCallableSingleLoopSourceMapV1` (initial `0`, condition `< 1`,
  step `+ 1`, one carrier BindingRef, and a tail read)
one resolver-issued prefix callable is retained by the existing Callable
  Prelude contract; it is not reclassified as a Generic operation
no nested Loop, If, Break, Continue, dynamic/handle result, or unowned effect
```

This shape is a design candidate, not an accepted language expansion. The
existing broad GenericLoopV1 extractor remains the source observation for
other shapes, and those shapes remain on their current explicit route until a
separate product is available. The candidate may be declined if the source
ledger does not retain enough exact sites for every operation; it must not be
silently widened or relabeled to make the demand fit. The broad
`GenericLoopV1` extractor remains on its current explicit route.

## Proposed single-owner chain

The next design must select one owner for this chain. The names below are
contract vocabulary, not implementation authorization:

```text
VerifiedCallableSingleLoopRecipeProductV1
  -> issue_callable_semantic_program_v1
       -> existing VerifiedLoopOperationEffectProductV1
       -> existing VerifiedLoopContinuationContractV1
  -> VerifiedLoopOperationPhysicalDemandV1::issue(...).prepare_all()
  -> existing canonical session/physicalizer
```

The selected callback must consume the already-issued source relation exactly
once. It may only use the existing Callable syntax-facts/source-map/Recipe
issuers and retained exact source sites. It may not call `try_build_outcome`,
reselect a route, inspect a name or MIR value, call `RecipeComposer`, or derive
a BindingRef from a variable name. If the callback cannot lend the exact
resolver input and ledger through this chain, the row remains
`NoSafeSlice__GenericLoopResolverContextUnavailable`; no partial adapter is
added.

The issued program is an aggregate of existing neutral products, not a second
physical owner. It contains no `MirBuilder`, `ValueId`, `BasicBlockId`,
`PhiTxn`, runtime token, or callable Tail. Loop After remains distinct from the
function completion/Tail capability that the later session-entry row owns.

## Finite state

| State | Owner | Effect | Allowed next step |
| --- | --- | ---: | --- |
| `RecipeReady` | existing callable Recipe issuer | 0 | one cohort-demand audit |
| `ContextReady` | selected callable ingress callback | 0 | one demand issuer may consume the borrowed input/ledger |
| `CohortUnsupported` | demand-shape verifier | 0 | explicit `NoSafeSlice`; retain current route |
| `DemandMissing` | semantic-demand D0 | 0 | design the single source-bound producer |
| `DemandIssued` | selected demand issuer | 0 | neutral Core/operation/After co-seal |
| `PreparedOperation` | existing neutral physical-demand owner | 0 | later canonical session-entry I0 |
| `RejectedBeforeEffect` | Core/demand relation verifier | 0 | terminal discard; no fallback |
| `ConsumedBySession` | future session-entry owner | physical | outside this card |

Every state has one owner and one terminal. `Option::None`, a generic
compatibility label, or an old-composer result may not merge `CohortUnsupported`
and `DemandMissing`.

## Ordered tasks

1. Confirm the selected callback can borrow the existing
   `ResolvedFunctionLoweringInputV1` and `CallableSemanticSourceLedgerView`
   through one CallableSingleLoop demand issuance. Check owner, origin,
   source-kind, exact Loop site, frame, and scope/region there; do not add a
   `RawInvocationChildPortV1` field or a second context issuer.
2. Call the existing CallableSingleLoop syntax-facts, source-map, and Recipe
   issuers from that borrowed pair. Prove that condition, body, assignment,
   local initialization, prefix, and After sites are consumed once with no
   second source scan or name lookup.
3. Consume the existing Callable semantic-program/common-demand chain and
   verify operation, effect, placement, binding class, and After relations
   before a Builder session is opened.
4. Add named rejects for foreign owner/source context, unsupported profile,
   missing/duplicate operation, foreign BindingRef/site, wrong placement,
   missing effect, and After mismatch. All rejects occur before
   `VerifiedLoopOperationPhysicalDemandV1` is handed to a Builder session.
5. Only after the product is accepted, reopen the session-entry I0. The
   canonical session remains the sole PHI issuer and must use the value-flow
   contract in `loop-recipe-contract.md`.
6. Add the valid no-manual-ledger fixture and the zero/one/multiple iteration
   mutation matrix in the later I0 row. Do not use a fixture that already fails
   liveness as negative evidence.

## Acceptance and delete set

This design row closes only when the candidate has one named source-bound
issuer, one complete neutral operation/effect/continuation product, one
pre-effect verifier, and one existing common-demand consumer. Acceptance must
show:

- no AST reparse, route reselection, name-based BindingRef repair, or old
  Composer call;
- resolver context is borrowed from the selected ingress for this one issuer;
  no semantic/session field is added to `RawInvocationChildPortV1`;
- exact owner/source-site/binding/placement/After equality;
- complete Recipe-order operation coverage and no single-operation extraction;
- builder/session effect count zero through `prepare_all`;
- explicit `CohortUnsupported` and relation-mutation rejects;
- broad GenericLoopV1, Generic G0, DirectAccum, nested, and compatibility
  callers remain unchanged; the Callable profile is not advertised as a
  GenericLoopV1 promotion.

There is no deletion set yet. The old GenericLoopV1 Composer path can be
deleted only after the new demand is production-connected, the canonical
session acceptance matrix passes, and its caller is zero for the selected
cohort. No OBJ/EXE, process exit, backend, or R7 claim is made here.
