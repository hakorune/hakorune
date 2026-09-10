---
Status: design stop; successor to generic physical-demand D0
Date: 2026-09-11
Decision: MIR-CALLABLE-LOOP-PHI-GENERIC-SEMANTIC-DEMAND-D0
Parent: mir-callable-loop-phi-generic-physical-demand-d0-2026-09-11
---

# MIR-CALLABLE-LOOP-PHI-GENERIC-SEMANTIC-DEMAND-D0

## Six-line brief

```text
Decision: choose one source-bound semantic-demand producer for a bounded
  GenericLoopV1 shape; keep the broad GenericLoopV1 route on its existing
  compatibility consumer until that product is complete.
Source authority + canonical issuer: CallableGenericLoopSourceFactsIssuerV1 /
  CallableGenericLoopV1SemanticRecipeIssuerV1 own the claimed source facts and
  BindingRef/site rows; one selected demand issuer owns Recipe/Core/JoinSig/
  operation/effect/After co-sealing; CanonicalSsaFunctionSessionV2 owns all
  physical ValueId/CFG/PHI/seal issuance.
Non-authority: GenericLoopV1Facts by itself, PlanBuildOutcome, pre_effect rows,
  names, variable_map, old RecipeComposer, CorePhiInfo, composer-local maps,
  latest values, and Generic G0's source product.
Fail-fast boundary: before any Builder effect, reject unsupported expression
  shape, missing operation/value/effect/placement/After relation, owner or
  source-site drift, and incomplete binding coverage; never retry the old
  composer for a selected canonical candidate.
Smallest next slice: audit and accept a single non-nested straight-line I64
  GenericLoopV1 cohort whose existing source facts can issue complete neutral
  operation/effect/continuation rows in one semantic pass.
Non-claims: no broad GenericLoopV1 promotion, nested/If/BlockExpr/call/effect
  support, local-completion closure, session/OBJ/EXE, or legacy retirement.
```

### Census boundary

```text
CallableGenericLoopSourceFactsIssuerV1::issue_once
  -> CallableGenericLoopV1SemanticRecipeIssuerV1::issue
  -> one bounded semantic-demand issuer
  -> VerifiedLoopOperationPhysicalDemandV1::issue/prepare_all;
includes the source-bound GenericLoopV1 Ready branch and its operation/value/
effect/After relations; excludes GenericLoopV0, Generic G0, DirectAccum,
legacy Composer, nested/Outside routes, function session emission, and backend
artifacts.
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
second authority and is rejected.

## First-cohort candidate

The first canonical candidate is deliberately narrower than the observed
GenericLoopV1 grammar:

```text
one non-nested loop
no BlockExpr loop prelude
numeric progression with an exact, fixed step placement
one carrier BindingRef with condition-read, body-read, and body-rebind rows
condition and body are straight-line I64 operations only
  (integer literal, variable read, Add/Sub, Less/LessEqual/Equal, assignment,
   and local initialization)
no method call, external call, If, nested Loop, Break, Continue, Return in the
  body, string/bool/handle value, or effectful operation
```

This shape is a design candidate, not an accepted language expansion. The
existing broad GenericLoopV1 extractor remains the source observation for
other shapes, and those shapes remain on their current explicit route until a
separate product is available. The candidate may be declined if the source
facts do not retain enough exact sites for every operation; it must not be
silently widened to make the demand fit.

## Proposed single-owner chain

The next design must select one owner for this chain. The names below are
contract vocabulary, not implementation authorization:

```text
claimed CallableGenericLoopV1SemanticRecipeV1
  -> CallableGenericLoopV1SemanticDemandIssuerV1
       -> source-bound LoopRecipeArtifact + JoinSig
       -> VerifiedLoopCoreProductV1
       -> VerifiedLoopOperationEffectProductV1
       -> VerifiedLoopContinuationContractV1
  -> VerifiedLoopOperationPhysicalDemandV1::issue(...).prepare_all()
  -> existing canonical session/physicalizer
```

The demand issuer must consume the already claimed source relation exactly
once. It may only use the selected source shape and its retained exact source
sites. It may not call `try_build_outcome`, reselect a route, inspect a name or
MIR value, call `RecipeComposer`, or derive a BindingRef from a variable name.
If operation relations require a second AST traversal or a second semantic
issuer, the candidate remains `NoSafeSlice` and the source Facts issuer must be
reopened as the authority decision; no partial adapter is added.

The issued program is an aggregate of existing neutral products, not a second
physical owner. It contains no `MirBuilder`, `ValueId`, `BasicBlockId`,
`PhiTxn`, runtime token, or callable Tail. Loop After remains distinct from the
function completion/Tail capability that the later session-entry row owns.

## Finite state

| State | Owner | Effect | Allowed next step |
| --- | --- | ---: | --- |
| `RecipeReady` | existing callable Recipe issuer | 0 | one cohort-demand audit |
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

1. Inventory the retained source sites and BindingRef rows for the candidate
   shape. Prove that condition, body, assignment, local initialization, and
   After can each be named without a second source scan or name lookup.
2. Compare the candidate with the existing `LoopRecipeV1` operation/value
   schema and JoinSig rules. Record every operation row, source anchor,
   placement, binding class, and continuation relation required by
   `VerifiedLoopOperationEffectProductV1::issue`.
3. Select the single issuer boundary. Prefer extending the existing callable
   source issuance/co-seal so the operation rows are issued alongside the
   already selected source shape. A separate compiler-side issuer is allowed
   only if it consumes the claimed Recipe once and publishes no competing
   source meaning.
4. Add named rejects for unsupported shape, missing/duplicate operation,
   foreign BindingRef/site, wrong placement, missing effect, and After mismatch.
   All rejects occur before `VerifiedLoopOperationPhysicalDemandV1` is handed
   to a Builder session.
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
- exact owner/source-site/binding/placement/After equality;
- complete Recipe-order operation coverage and no single-operation extraction;
- builder/session effect count zero through `prepare_all`;
- explicit `CohortUnsupported` and relation-mutation rejects;
- broad GenericLoopV1, Generic G0, DirectAccum, nested, and compatibility
  callers remain unchanged.

There is no deletion set yet. The old GenericLoopV1 Composer path can be
deleted only after the new demand is production-connected, the canonical
session acceptance matrix passes, and its caller is zero for the selected
cohort. No OBJ/EXE, process exit, backend, or R7 claim is made here.
