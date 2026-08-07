# Callable Loop Production Prepared Ingress D0

Status: design stop opened after the accepted logical issuer S0
(2026-08-08). No physical implementation is authorized by this card.

## Purpose

Define the one production-side ingress that may later assemble the already
verified callable logical product into a Prepared physicalization product.
This is a boundary design, not a selector or a caller switch.

## Sole authority

```text
NormalCallableSemanticLoanPortV1
  -> ResolvedFunctionLoweringInputV1
  -> source/facts logical issuer S0
  -> VerifiedCallableSingleLoopRecipeProductV1
  -> PreparedCallableLoopPhysicalizationV1
  -> fresh CanonicalFunctionLoweringSessionV1
```

The source/facts and Recipe/JoinSig/After issuers remain the existing owners.
The Prepared product may prove only execution compatibility; it must not
re-own Recipe, JoinSig, After, Tail, ABI, Completion, or publication meaning.

## Receipts to fix before implementation

```text
owner / function frame / scope brand
logical callable product (move-only)
Prelude argument receipt
common Loop demand (move-only)
Loop After continuation
callable Tail
exact Return ABI
VerifiedFunctionCompletionV1
fresh-session / discard terminal
```

The common physicalizer must receive only the common Loop demand and a
session-local entry receipt. Callable Tail, ABI, Completion, DraftSeal,
selector, and module publication remain outside it.

## Fail-fast boundary

Reject before opening a function session when any receipt is missing, foreign,
duplicated, borrowed instead of moved, or cannot be tied to the same resolver
owner/frame/scope. After a fresh session opens, any physical failure discards
the whole unpublished function session and restores the caller once. Retry and
fallback are not allowed.

## Non-claims

```text
Prepared physicalization = 0
CFG / SSA / PHI / ValueId / BasicBlockId = 0
production caller switch = 0
Generic G0 parity = 0
selector / retry / fallback = 0
DraftSeal / collector / publication = 0
legacy deletion = 0
```

## Acceptance

```text
one ingress owner and one receipt table
logical product is consumed exactly once
common physicalizer boundary remains profile-blind
fresh-session/discard owner is explicit
current production host and future old edge are named
positive/foreign/missing/late-discard/fresh-session fixtures are listed
reference/current docs and task order are updated before I0
```
