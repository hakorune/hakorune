# Callable Loop Production Logical Issuer D0

Status: accepted design stop after closed
`CALLABLE-LOOP-PRODUCTION-SOURCE-FACTS-ISSUER-S0`.
Decision: existing-owner reuse; the next implementation is a bounded logical
issuer only. No physical or production caller activation is included.

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

## Exact callable mapping

The following table is the complete S0 logical mapping. Recipe keys are
canonical semantic keys, not source ordinals; source roles are consumed once.

| Source role | Recipe placement | Logical operation/value | Relation/effect |
|---|---|---|---|
| `InitialCarrier` | input `V0`, carrier `C0`, binding `B0` | declared `i64` carrier entry | one `DerivedCarrierEntry` at Loop site |
| `ConditionBound` | condition item `I0` | `ConstI64(V2, 1)` | operation source site |
| `ConditionRead` | condition item `I1` | `ReadBinding(B0 -> V1)` | `SourceRead(0)` |
| `ConditionOperator` | condition item `I2` | `CompareI64(Less, V1, V2 -> V3)` | operation source site |
| `StepRead` | body item `I3` | `ReadBinding(B0 -> V4)` | `SourceRead(1)` |
| `StepDelta` | body item `I4` | `ConstI64(V5, 1)` | operation source site |
| `StepOperator` | body item `I5` | `BinaryI64(Add, V4, V5 -> V6)` | operation source site |
| `StepWrite` | body item `I6` | `WriteBinding(B0, V6)` | `SourceWrite(0)` |

The canonical recipe has one root Loop, condition block `[I0,I1,I2]`, body
block `[I3,I4,I5,I6]`, predicate `V3`, carrier `C0/B0 -> V0`, seven operation
items, and no explicit exits. `LoopJoinSigElaboratorV1` must elaborate the
verified Recipe and `require_after_binding(root, B0, I64)` must issue the sole
After capability. The callable prefix and terminal Tail both use their own
resolver binding, which must not be fused with the Loop After binding.

## Production promotion boundary

The production entry belongs to the existing logical compiler boundary, not a
new physicalizer:

```text
VerifiedCallableSingleLoopSourceMapV1 (move)
  -> callable source-to-Recipe relation DTO
  -> LoopRecipeVerifierV1
  -> LoopJoinSigElaboratorV1
  -> require_after_binding
  -> issue_source_bound_core_v1
  -> VerifiedCallableSingleLoopLogicalProductV1 (move)
```

The source map remains the only source identity input. The implementation may
add a production wrapper around `issue_source_bound_core_v1` that first
verifies the artifact; it must not expose private verifier state or call the
test-only `issue_source_bound_core_for_test`. The static `callable_recipe()`
fixture and test-only mutation constructors stay in tests. The
`CallableSingleLoopV1` producer id may be retained as diagnostics-only
provenance and must never select, schedule, or dispatch a route.

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

The logical issuer's positive result is a move-only source-bound product. It
does not contain AST, names, route IDs, ValueIds, BasicBlockIds, CFG, PHI,
Completion, ABI, or physical policy. All failures are typed before opening a
function session; external user-facing diagnostic mapping remains deferred.

## Stop line

Until this D0 is accepted, do not implement a production Recipe issuer,
`callable_recipe()` adapter, selector, Prepared product, physicalizer, or
production caller switch. If the mapping cannot be expressed as the existing
Recipe/JoinSig algebra, return `NoSafeSlice` and revise this design instead of
adding a callable-specific Recipe kind.
