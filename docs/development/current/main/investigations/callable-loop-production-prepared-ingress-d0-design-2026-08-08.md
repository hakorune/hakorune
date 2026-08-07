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
  -> existing-owner ingress receipt
       CallableSemanticSourceLedgerView
       ResolvedFunctionLoweringInputV1
       VerifiedCallableIndexV1 / VerifiedCallableHeaderV1
       owner / frame / scope brand
  -> source/facts logical issuer S0
  -> VerifiedCallableSingleLoopRecipeProductV1
  -> PreparedCallableLoopPhysicalizationV1
  -> fresh CanonicalFunctionLoweringSessionV1
```

The source/facts and Recipe/JoinSig/After issuers remain the existing owners.
The Prepared product may prove only execution compatibility; it must not
re-own Recipe, JoinSig, After, Tail, ABI, Completion, or publication meaning.

The ingress receipt is a transport view over those existing owners, not a new
semantic owner. It must be issued once from the exact selected source loan and
the already-installed callable catalog. It may retain shared borrows of the
forest/projection/index/header only while the prepared request is alive; it may
not copy AST, rebuild a forest, resolve by name, or synthesize a header.

## Repository audit: implementation is not open yet

The current host is concrete but incomplete for this boundary:

```text
NormalCallableSemanticLoanPortV1
  owns: raw child port + VerifiedNormalCallableSemanticSourceV1 borrow
  currently does: loan -> CallableSemanticLoweringState -> raw body lowering
  currently does not: issue ResolvedFunctionLoweringInputV1 or callable
                       Index/Header paired with the same owner/frame/scope
```

`VerifiedNormalCallableSemanticLoanV1::into_parts()` currently returns only
the raw lineage and request-local `CallableSemanticLoweringState`; its forest,
source projection, and exact function view are not retained there. The
installed `CompilationContext` catalog is an existing authority and may be
borrowed, but it is not an automatic pairing mechanism. Therefore the first
implementation slice must add or expose one exact source-loan expansion
receipt before any physicalizer code is enabled. Removing `cfg(test)` from the
existing prepared types is not sufficient.

For the selected top-level callable, the adapter must also prove an exact
catalog/index/header correspondence. If the selected source key cannot be
paired without name/arity re-resolution or AST re-walk, the result is typed
`NoSafeSlice` and the physical row remains parked.

The bounded candidate is:

```text
existing source loan + installed catalog
  -> one private move-only ingress receipt
       exact ledger view
       exact ResolvedFunctionLoweringInputV1
       exact callable index/header (when the profile requires them)
       one owner/frame/scope identity
```

This receipt is an adapter over existing source/facts/catalog owners. It is
not a `CallablePlan`, universal semantic product, or second resolver.

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

## Ordered implementation gate after this design stop

No step below is authorized until this D0 is accepted and the current pointer
moves to an implementation row.

```text
1. source-loan expansion receipt
   expose the exact ledger view, source/function input, and callable
   index/header from existing owners; prove owner/frame/scope identity;
   preserve the raw host path unchanged.

2. prepared ingress assembler
   consume the existing logical product and disjoint Prelude/Tail/ABI/
   Completion capabilities exactly once; issue one move-only prepared product.

3. Builder-free full-demand preflight
   verify complete operation coverage and all profile compatibility before a
   function session opens; missing/foreign/duplicate/borrowed evidence is
   typed `NoSafeSlice`.

4. bounded physical canary
   only after 1--3: open a fresh session, use the common profile-blind
   physicalizer, and prove whole-session discard plus fresh-session reuse.

5. reference closeout
   after implementation, update the relevant `docs/reference/**` contract,
   diagnostics, migration note, tests/guards, and current mirrors in the same
   closeout commit. This D0 itself changes no language/reference semantics.
```

## Acceptance

```text
one ingress owner and one receipt table
current host limitation and missing source-loan expansion are explicit
exact source/forest/projection/catalog identity is proven before I0
logical product is consumed exactly once
common physicalizer boundary remains profile-blind
fresh-session/discard owner is explicit
current production host and future old edge are named
positive/foreign/missing/late-discard/fresh-session fixtures are listed
reference/current docs and task order are updated before I0
```
