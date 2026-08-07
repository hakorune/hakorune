# Callable Loop Production Source/Facts Bridge D0

Status: accepted design after worker audit (2026-08-08). Production
implementation remains `0`; the next row is the bounded source-facts issuer
split.

Decision: do not create a new semantic `Bridge` owner. Promote the existing
resolver-backed source ledger plus the existing `SyntaxFacts -> SourceMap`
products into a production issuer, after separating neutral observers from
test fixtures. Recipe/JoinSig/Core/effect/After, Prepared physicalization,
and production selection remain separate rows.

## Problem boundary

The production host is known:

```text
NormalCallableSemanticLoanPortV1::lower_normal_top_level_function
```

It owns outer callable orchestration, but its current loan exposes only
lineage and `CallableSemanticLoweringState`. The loan alone does not expose
the forest, resolver function, source projection, callable index/header, or a
Loop source-facts product. The current single-loop SyntaxFacts, SourceMap,
Recipe co-seal, and physical canary modules are `cfg(test)`.

Removing `cfg(test)`, copying a fixture, or recovering information from the
raw loop route would create a second authority and is forbidden.

## Sole source authority

The bridge may consume only these existing products:

```text
VerifiedNormalCallableSemanticSourceV1
  -> sealed forest + resolver function + source projection

VerifiedNormalCallableSemanticLoanV1
  -> lineage/lowering scope only; not a standalone facts issuer

CallableSemanticSourceLedgerView
  -> declarations, lexical refs, assignments, direct calls, exits,
     captures, and exact Loop membership

ResolvedFunctionLoweringInputV1 / FunctionSourceViewV1
  -> exact canonical source navigation for the Facts observer
```

`VerifiedCallableLoopMembershipV1` is the existing owner-branded identity
receipt. It carries the resolver-issued Loop source token, execution frame,
and Scope/Region pair. It must be moved into the source map; loose owner/site/
frame tuples are not substitutes.

The resolver boundary for this row is the smallest exact projection:
`CallableSemanticSourceLedgerView::only_loop_site()`. It returns the one
resolver-sealed Loop membership when cardinality is exactly one and returns
typed `NoSafeSlice` for zero or multiple sites. It never mints a
`SourceStmtSiteV1`, chooses a route-local ordinal, or exposes a raw iterator.
The observer navigates source only through the owner-branded
`FunctionSourceViewV1::stmt_at(membership)` seam. If either exact seam is missing,
the row stops with typed `NoSafeSlice` rather than adding a route-local
adapter.

## Product boundary: promote existing products, do not aggregate

The production source/facts boundary has two products, not a new aggregate.

### 1. `VerifiedSourceSyntaxFactsV1`

This is the AST-free neutral source observer result. It may inspect an exact
`FunctionSourceViewV1` during the Facts phase, but it stores no AST, names as
identity, route labels, `ValueId`, CFG, PHI, Recipe, or physical policy.

It owns only the admitted profile's as-written shapes:

```text
initial carrier
condition lhs/rhs/operator
step target/lhs/rhs/operator
separate PrefixBoundary
separate terminal Tail
```

The observer must be split from test fixture constructors and mutation helpers.
Its source sites are exact resolver/projection sites, and it rejects opaque,
unsupported, duplicate, extra, nested, or non-terminal shapes for this bounded
single-loop profile. A verified `FunctionSourceViewV1::root_body()` traversal
is allowed for Facts observation such as extra-body totality; raw AST-vector
walking, ordinal/path reconstruction, name-based identity, and route-label
recovery are forbidden.

### 2. `VerifiedCallableSingleLoopSourceMapV1`

This is the source/facts co-seal product already designed in the caller-zero
row. It consumes `VerifiedSourceSyntaxFactsV1` and
`CallableSemanticSourceLedgerView` once and owns exactly:

```text
owner / FunctionOrigin / source-kind
resolver-issued Loop source / frame / Scope-Region
nine typed source-role rows
one separate PrefixBoundary row
  BindingRef / direct-call / assignment targets plus a sealed resolver-exit fact
```

The source map is AST-free and move-only. It is the production source/facts
relation. Do not add `VerifiedCallableLoopSourceFactsBridgeV1`; the word
Bridge names the construction step, not a second semantic product.

## Explicit non-owners

```text
VerifiedCallableSingleLoopRecipeProductV1
  owns Recipe/JoinSig/Core/effect/After only in the later Recipe row

VerifiedLoopOperationPhysicalDemandV1
  owns the full common physical input only after Recipe co-seal

VerifiedCallablePreludeV1 / VerifiedCallableTailV1
  remain callable boundary capabilities, not source-map replacements

CallableSemanticLoweringState
  owns request-local BindingRef -> ValueId projection only; it is not source
  authority and must not be used to recover missing facts

Canonical CFG/SSA/PHI, Completion, ABI, DraftSeal, collector, module
publication, selector, retry, and fallback
  remain their existing sole owners
```

`normal_callable_loop_handoff.rs` is a pre-effect three-row binding schedule
for the legacy raw edge. It is not the full source-map authority and must not
be widened into a second semantic product. It is retired only after the
production source-map and later Recipe path replace its named caller.

## Exact correspondence

| Existing authority | Production receipt | Reject when |
| --- | --- | --- |
| resolver forest/function | owner, origin, source-kind, exact Loop site | foreign/missing owner or site |
| resolver Loop index/site inventory | `VerifiedCallableLoopMembershipV1` | raw site, duplicate site, missing frame |
| `ResolvedFunctionLoweringInputV1` + `FunctionSourceViewV1` | neutral SyntaxFacts sites/shapes | navigation failure, opaque/unsupported shape, extra body |
| `CallableSemanticSourceLedgerView` | BindingRef/direct-call/assignment targets plus sealed resolver-exit fact | missing, duplicate, foreign, unresolved target |
| SyntaxFacts + ledger | one SourceMap row per typed `(site, role, target-kind)` | role/target mismatch or duplicate |
| SourceMap | later Recipe/JoinSig co-seal relation | uncovered key/item or copied Core |

Source-site uniqueness alone is insufficient: one expression may carry more
than one typed role. Coverage is keyed by `(typed site, role, target-kind)`.
Prefix and terminal Tail are callable siblings; they must never be fused with
Loop After.

## Failure and transaction boundary

```text
source-facts issuer preflight:
  no Builder/session effect
  typed NoSafeSlice on missing/foreign/incomplete evidence

later prepared ingress:
  open one fresh CanonicalFunctionLoweringSessionV1
  move Completion into CanonicalSsaFunctionSessionV2 exactly once

physical execution:
  Prelude -> common Loop -> After -> Tail
  finish_for_draft_seal -> DraftSeal prepare/commit

failure after session open:
  CanonicalFunctionLoweringSessionV1::discard_unpublished
  restore caller once
  no same-session retry/fallback
```

The source/facts row itself never opens a function session or publishes a
collector/module draft. Phi rollback is auxiliary cleanup, not atomicity
ownership.

## Required implementation row and acceptance

The next row is
`CALLABLE-LOOP-PRODUCTION-SOURCE-FACTS-ISSUER-S0`:

1. split neutral SyntaxFacts/source-shape/map issuers from test fixtures;
2. use `CallableSemanticSourceLedgerView::only_loop_site()` for exact
   single-site cardinality and `FunctionSourceViewV1::stmt_at(membership)` for
   owner-branded source navigation;
3. expose the existing source ledger and exact `ResolvedFunctionLoweringInput`
   to the observer without AST/name re-walk;
4. issue the existing SourceMap product in production scope;
5. add positive, missing, duplicate, foreign, cross-brand, nested, opaque,
   and SourceMap/ledger parity fixtures;
6. prove caller-zero and Builder-effect-zero construction;
7. keep every touched Rust/check file below 800 lines.

Only after S0 is green may the Recipe/JoinSig/Core/effect/After production
issuer be designed and implemented. Prepared production issuance additionally
requires exact callable index/header retention, ABI, Completion, and the
existing fresh-session/DraftSeal canary.

## Non-claims

```text
production source/facts issuer = 0 until S0
Recipe/JoinSig/Prepared production issuer = 0
physical Loop emission = 0
I0 caller switch = 0
Generic G0 parity = 0
retry/fallback/legacy deletion = 0
```

Every implementation slice must update the affected source README,
`docs/reference/**`, diagnostics, migration note, guards, and current pointer
in the same commit. Reference synchronization is an acceptance condition, not
a follow-up task.
