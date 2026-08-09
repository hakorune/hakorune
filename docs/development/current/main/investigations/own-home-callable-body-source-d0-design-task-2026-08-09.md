---
Status: accepted design stop — implementation not open
Date: 2026-08-09
Parent: `docs/development/current/main/investigations/own-home-callable-conformance-catalog-d0-design-task-2026-08-09.md`
Authority: `docs/reference/language/callable-contracts.md`
---

# CALLABLE-BODY-SOURCE-AUTHORITY-D0

## Decision

Body conformance is not opened directly after the declared Query/Home
aggregate. A separate source-authority row must first bind each Box method
body to the exact resolver declaration identity.

The current repository has no such issuer:

```text
ParserBoxResolverSourceHandoffV1
  = one-shot, AST-free declaration/signature/typed-rune ingress
  = deliberately carries no method body

VerifiedInstanceMethodDeclarationCatalogV1
  = exact nominal Box/method identity and semantic I64/Unit signature
  = does not own a body or FunctionOwnerId

VerifiedResolvedFunctionV1
  = resolved binding/scope/region/body facts for one function owner
  = is not co-sealed with a Box method declaration

CallableCatalogResolutionSourceV1
  = body view for the existing free-static callable catalog
  = not an instance-Box method source authority
```

Therefore the present body-conformance boundary is `NoSafeSlice`, not
`Candidate`, `Declined`, or `Unresolved` source disposition. No empty body
receipt, AST/name scan, method-name lookup, or MIR-derived proof may be added
to bypass this gap.

## Required final shape

The source/body path must become a one-way sequence:

```text
parser-owned rich source product
  + exact SourceBoxMethodSiteV1
  + parser invocation brand
  + exact method body root/order
        |
        v
resolver body-source issuer
  + resolver declaration catalog identity
  + nominal Box/method identity
        |
        v
VerifiedInstanceMethodBodySourceCatalogV1
  = non-Clone, AST-free body source capability
  = one exact body source row per selected declaration
        |
        v
private body observer
        |
        v
VerifiedCallableBodyFactsCatalogV1
  = source-derived behavioral facts only
        |
        v
CallableContractConformanceIssuerV1
  + declared Query/Home aggregate
  + complete same-brand body facts
        |
        v
VerifiedConformantCallableCatalogV1
```

The public semantic conformance product must not retain AST nodes. A private
observer may borrow parser syntax while issuing the AST-free body facts, but
the observer must consume only the branded body-source capability and the
matching declaration catalog. It may not rescan a program, reconstruct a
method from `Box`/method names, or use selected/generated inventory ordinals as
source identity.

## Body carrier decision

The body carrier is a parser-side, one-shot envelope created by the same rich
parse transaction that issues the declaration handoff. It is not a second
parser authority and it does not change the existing declaration-only
`ParserBoxResolverSourceHandoffV1`:

```text
ParserBoxBodySourceEnvelopeV1   (private parser side, non-Clone)
  parser invocation brand
  declaration-only resolver handoff
  exact direct SourceBoxMethodSiteV1 rows
  exact body root for each row
  ordered body-item source paths/cardinality
  private syntax arena/view used only by one observer callback
```

The envelope is the only legal way to pair the returned AST/body syntax with
the AST-free declaration handoff. An API that returns an AST and a body-free
handoff as independent values and later pairs them by name, ordinal, or
`ASTNode` search is not a body-source authority. The existing declaration-only
API remains for declaration/signature consumers; the body envelope is a new
parser-private ingress for this row.

The resolver-facing result is a fresh non-`Clone`
`VerifiedInstanceMethodBodySourceCatalogV1`. It contains only the exact
resolver/catalog brand, direct method source identity, body-root identity, and
complete ordered-body coverage. It does not contain an `ASTNode`, a
`FunctionOwnerIdV1`, body effects, Query, Home, semantic types, target,
Recipe, or MIR. The private observer callback may borrow the syntax arena while
the envelope is alive; the callback must finish before the envelope is dropped.

The first cohort is deliberately limited to an ordinary top-level Rust Box
with explicit direct instance methods. Selected-gate, Hako, interface, static,
record, mixed, generated, and compatibility rows remain `NoSafeSlice` until a
separate source-path decision supplies their exact body coordinates.

## Function-owner binding is a later boundary

`FunctionSemanticResolverSessionV1` is the sole issuer of
`FunctionOwnerIdV1`/`VerifiedResolvedFunctionV1`. Body-source I0 never mints,
copies, or reinterprets that owner. A later
`InstanceMethodBodyOwnerBindingIssuer` must co-seal:

```text
VerifiedInstanceMethodBodySourceCatalogV1
+ exact resolved function product
+ declaration/catalog identity
  -> one branded instance-method body-owner link
```

Equal-looking owner numbers, source ordinals, method names, or compilation
brands are insufficient. Until this link issuer exists, body facts that need
resolved lexical/control facts remain `NoSafeSlice`.

## Input ownership

The body-source issuer borrows the already landed
`VerifiedDeclaredInstanceMethodContractCatalogV1`; it does not consume or
clone the Home catalog, Query catalog, or declaration catalog. The aggregate
remains available for the later conformance co-seal. The one-shot move belongs
to the parser body envelope, not to the semantic aggregate.

The exact public type names above are design names until the implementation
row closes. The ownership and ordering are normative.

## Body facts for the first Query cohort

The first body-observation cohort is deliberately narrow and must be fixed
before implementation:

```text
allowed:
  exact receiver direct-state read
  expression evaluation with no forbidden effect
  ordinary return matching the declared semantic result

forbidden:
  receiver/global/ambient writes
  unrelated heap reads
  Home consume/create/share/end/escape
  allocation
  IO / FFI
  Fault / throw / qmark propagation
  suspension / async transfer
  non-local control
  transitive callee/read-footprint composition
```

The observer records exact source sites and resolved identities for the facts
it proves. It does not issue `Query`, `Handle`, parameter/result Home
relations, semantic I64, physical ABI, `EffectMask`, or `FunctionSignature`.
Those meanings remain owned by the declaration, Home, and signature issuers.

`VerifiedResolvedFunctionV1` may supply resolved lexical/control facts only
after it is explicitly co-sealed to the same Box method declaration. An equal
`FunctionOwnerIdV1`, source ordinal, or method name is not sufficient.

## Co-seal and coverage

The later conformance issuer consumes, by value or through a branded
one-shot handoff, exactly two complete products:

```text
VerifiedDeclaredInstanceMethodContractCatalogV1
  = declaration + declared Query subset + Home ABI

VerifiedCallableBodyFactsCatalogV1
  = body facts for exactly that declared subset
```

It issues `VerifiedConformantCallableCatalogV1` only when all of these hold:

```text
same resolver/catalog brand
same nominal Box/method identity tuple
same source method site
one body row per declared Query row
no duplicate, foreign, or missing body row
every body fact satisfies its declared contract
no body fact replaces or widens the public declaration
```

Non-Query declarations may remain outside the selected Query cohort. They
must not receive a default body row. A body row for a method without a
declared contract is outside this first conformance product.

## Disposition

```text
NoSafeSlice:
  no parser/resolver body-source issuer with exact method identity

Candidate:
  complete branded body-source and body-facts products exist; all Query
  obligations can be checked without re-inference

Declined:
  body is fully observed but is outside the bounded Query cohort

Unresolved:
  source/body facts are incomplete or opaque while identity is intact

Rejected:
  foreign brand/site/owner, duplicate or missing row, or contradictory
  resolved source relation
```

`NoSafeSlice` is development state only. It must not be encoded as an empty
`VerifiedCallableBodyFactsCatalogV1` or reported as a language disposition.

## Nonclaims and stop lines

This row does not open:

```text
resolver instance target
source-bound call relation
Recipe / CallSlot
Builder / MIR / CFG / PHI
physical ABI projection
runtime/provider dispatch
fallback/retry
module publication
```

The body observer must not infer source semantics from MIR `EffectMask`,
`FunctionSignature`, runtime state, provider metadata, or a method name. The
conformance catalog is a Verify product; publication later consumes it and
performs no semantic re-check.

## Ordered task ladder

```text
1. CALLABLE-BODY-SOURCE-AUTHORITY-D0
   source authority census, exact identity tuple, body-root/order contract,
   parser body envelope, resolver borrow boundary, owner-link stop line, and
   fail-fast matrix (this row)

2. CALLABLE-BODY-SOURCE-AUTHORITY-I0
   one bounded `length(): i64` instance-method body source capability;
   resolver-only, AST-free after handoff, no conformance yet. This row stays
   `NoSafeSlice` until the parser body envelope and its one-shot callback are
   implemented; do not add a test-only constructor.

3. CALLABLE-BODY-FACTS-QUERY-D0
   exact receiver-read/return observation and body-facts owner

4. CALLABLE-BODY-FACTS-QUERY-I0
   positive/negative body facts for the bounded fixture; no contract co-seal

5. CALLABLE-CONFORMANCE-CATALOG-D0/I0
   complete same-brand declared-contract + body-facts co-seal;
   issue `VerifiedConformantCallableCatalogV1`

6. only after 5:
   resolver target -> source-bound relation -> Recipe CallSlot
```

Every implementation row must update its focused tests, owner README, and the
callable-contracts reference in the same commit. No row may add a public
`Verified*` constructor without its canonical issuer and negative matrix.
