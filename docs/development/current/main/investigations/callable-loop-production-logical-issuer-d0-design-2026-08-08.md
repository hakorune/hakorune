# Callable Loop Production Logical Issuer D0

Status: design stop after closed `CALLABLE-LOOP-PRODUCTION-SOURCE-FACTS-ISSUER-S0`.
Decision: design only; no production Recipe/JoinSig issuer is activated here.

## Objective

Define the single source-to-logical boundary for the callable single-loop
profile before any Recipe/JoinSig implementation begins. This row must decide
the existing owners and exact mapping; it must not add a Bridge, Logical, or
After owner.

## Canonical pipeline

```text
resolver CallableSemanticSourceLedgerView / closed SourceMap
  -> profile source-to-Recipe relation DTO
  -> existing LoopRecipeArtifact + verifier
  -> existing LoopJoinSigElaborator + VerifiedLoopJoinSig
  -> JoinSig::require_after_binding
  -> VerifiedLoopContinuationContractV1
  -> existing issue_source_bound_core_v1 co-seal
```

The existing Recipe verifier is the only Recipe authority, the JoinSig
elaborator is the only JoinSig authority, and `require_after_binding` is the
only After-binding issuer. The existing source-bound Core co-seal remains the
only owner of the combined Recipe/JoinSig/source relation. A new aggregate is
allowed only as a move-only transport receipt with no semantic truth.

## Decisions required before implementation

1. Fix a complete source-role -> Recipe item/value/carrier/input/effect/After
   mapping for the seven-operation callable profile.
2. Require exact missing/duplicate/foreign/unconsumed/unsupported rejection
   before any Builder/session effect.
3. Keep Prelude and callable Tail/Completion as sibling contracts; do not
   merge them with Loop After.
4. Decide whether `LoopRecipeProducerIdV1::CallableSingleLoopV1` is
   diagnostics-only production provenance or remains test-only. It must never
   select or dispatch a route.
5. Keep `callable_recipe()` and every `issue_*_for_test` fixture-only; a
   production issuer must use resolver-backed relations and existing
   canonical verifiers.

## Non-claims

```text
Prepared / ABI / Completion physicalization = 0
CFG / SSA / PHI / ValueId / Builder / MIR = 0
DraftSeal / collector / publication = 0
selector / production caller / admission = 0
Generic G0 = 0
retry / fallback / legacy retirement = 0
runtime / backend / user diagnostics = 0
```

## Acceptance

```text
one source-to-Recipe cross-product parity table is SSOT
one JoinSig/After exact binding and owner/frame/Scope relation is proven
one positive and bounded missing/duplicate/foreign/unconsumed/unsupported matrix
existing Core/Recipe/JoinSig owners are reused; new semantic owner = 0
fixture builders remain test-only
Builder/session/caller-zero audit remains green
docs/reference and current task pointers are updated before implementation
```

## Stop line

Until this D0 is accepted, do not implement a production Recipe issuer,
`callable_recipe()` adapter, selector, Prepared product, physicalizer, or
production caller switch. If the mapping cannot be expressed as the existing
Recipe/JoinSig algebra, return `NoSafeSlice` and revise this design instead of
adding a callable-specific Recipe kind.
