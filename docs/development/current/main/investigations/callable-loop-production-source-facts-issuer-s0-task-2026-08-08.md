# Callable Loop Production Source/Facts Issuer S0

Status: implementation task after the accepted
`CALLABLE-LOOP-PRODUCTION-SOURCE-FACTS-BRIDGE-D0` design.

Implementation receipt (S0-A/S0-B, 2026-08-08): resolver
`CallableSemanticSourceLedgerView::only_loop_site()` and owner-branded
`FunctionSourceViewV1::stmt_at(membership)` are sealed with positive and
zero/multiple/inventory-negative tests. The neutral SyntaxFacts, source-shape,
and SourceMap modules now compile in production scope; fixture constructors,
mutation helpers, and tests remain test-only. Production issuer integration and
source-to-ledger parity are the remaining S0 scope.

## Objective

Move only the neutral callable single-loop SyntaxFacts/source-shape/SourceMap
issuers needed for one production source/facts boundary. Do not implement
Recipe/JoinSig physicalization, Prepared production admission, selector,
retry, fallback, Generic G0 substitution, or legacy deletion.

## Required structure

```text
resolver source/facts authority
  -> production VerifiedSourceSyntaxFactsV1
  -> production VerifiedCallableSingleLoopSourceMapV1
  -> later Recipe co-seal row
```

`CallableSemanticSourceLedgerView` remains the sole BindingRef/source target
authority. `CallableSemanticLoweringState` remains lowering-only. The exact
Loop site must come from
`CallableSemanticSourceLedgerView::only_loop_site()` (exactly one site;
zero/multiple sites are typed `NoSafeSlice`), and source navigation must use
owner-branded `FunctionSourceViewV1::stmt_at(membership)`. Raw AST path, name, route
label, and ordinal recovery are forbidden. Verified root-body traversal may
observe extra-body totality, but may not mint identity or reconstruct paths.

## Scope

- split `cfg(test)` neutral observer/source-shape/map code from test fixtures;
- keep test-only mutation constructors and fixture builders under tests;
- expose only AST-free, move-only products in production;
- add typed rejects for missing/duplicate/foreign/opaque/nested evidence;
- preserve PrefixBoundary and terminal Tail as separate from Loop After; the
  SourceMap retains only the sealed resolver-exit fact, not a second exit
  authority;
- no Builder/session/ValueId/BasicBlockId effects;
- keep each touched Rust/check file under 800 lines.

## Acceptance

```text
one positive production source/facts fixture
missing/duplicate/foreign/cross-brand rejection fixtures
nested/opaque/extra-body rejection fixtures
SourceMap <-> resolver ledger parity
resolver-issued site/owner/frame/Scope-Region receipts
caller-zero construction
Builder-effect-zero construction
fresh request reuse after failure
selected/legacy behavior unchanged
README/reference/diagnostic/current-pointer updates in the same commit
```

## Explicit non-claims

```text
Recipe/JoinSig/Core/effect/After production issuer = 0
PreparedCallableLoopPhysicalizationV1 production issuer = 0
physical Loop emission = 0
production caller switch = 0
retry/fallback/legacy deletion = 0
```

If the resolver site inventory or any source-to-ledger correspondence is
missing, stop with typed `NoSafeSlice` and update the design/current SSOT;
do not add a route-local adapter.
