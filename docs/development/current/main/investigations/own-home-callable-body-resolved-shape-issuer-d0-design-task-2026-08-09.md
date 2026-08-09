---
Status: accepted design; I0 implementation not landed
Date: 2026-08-09
Parent: `docs/development/current/main/investigations/own-home-callable-body-facts-query-d0-design-task-2026-08-09.md`
Authority: `docs/reference/language/callable-contracts.md`
---

# CALLABLE-BODY-RESOLVED-SHAPE-ISSUER-D0

## Decision

`CALLABLE-BODY-FACTS-QUERY-D0` cannot open an implementation row yet. The
current `VerifiedResolvedFunctionV1` and its source-site inventory prove
identity, bindings, assignments, calls, exits, and coverage membership, but
they do not expose enough neutral body meaning to distinguish the bounded
receiver-read/ordinary-return shape without reopening AST or guessing by
name/ordinal. The result is development `NoSafeSlice`, not a source
disposition. This D0 now closes the missing issuer boundary and opens one
bounded I0; it does not claim that body facts or conformance are implemented.

The next row is the resolver-side issuer implementation:

```text
same parser-private syntax lease / canonical resolver traversal
  -> VerifiedResolvedBodyShapeInventoryV1
       // general neutral body source truth, AST-free after issue
  + VerifiedInstanceMethodBodyOwnerCatalogV1
  -> private Query body observer
  -> VerifiedCallableBodyFactsCatalogV1
```

The owner link remains unchanged. This row must not reopen the parser
transaction, add a second owner issuer, or make Query the owner of general
body source meaning.

## Canonical issuer and product

The only public issuer is a resolver-session operation that extends the
existing shadow traversal and seals neutral shape rows at the same time as
the exact `VerifiedResolvedFunctionV1` root:

```text
FunctionSemanticResolverSessionV1::resolve_with_body_shape(view)
  -> ResolvedFunctionBodyShapeProductV1
       - exact owner-bearing VerifiedResolvedFunctionV1
       - VerifiedResolvedBodyShapeInventoryV1
```

The shadow traversal records private rows while it owns the borrowed
`FunctionSyntaxViewV1`; canonicalization converts them into one non-`Clone`
body-shape catalog branded by the same owner, body root, parser provenance,
resolver brand, and ordered source coverage. A second AST walker, caller map,
or post-seal `Verified*::new` constructor is not allowed.

## Authority split

```text
Parser/private syntax lease:
  exact source syntax view while canonical resolver traversal runs

Canonical resolver body-shape issuer:
  neutral expression/statement/effect/control rows and exact source sites

VerifiedResolvedFunctionV1:
  existing bindings/scopes/regions/calls/exits/coverage only; no retroactive
  inference of missing expression meaning

VerifiedInstanceMethodBodyOwnerCatalogV1:
  exact declaration/body-root/carrier relation; no behavior facts

Query observer:
  deterministic projection of the already-issued neutral shape inventory

Body facts/conformance:
  later products; no Home, Query, signature, ABI, target, Recipe, or MIR
```

The issuer must be created in the same transaction-scoped syntax lease used by
`FunctionSemanticResolverSessionV1`. It may borrow syntax only during issue;
the output must contain no AST node, syntax pointer, method-name lookup, or
parser arena handle. The owner link's parser provenance, resolver brand,
declaration identity, exact body root, and ordered coverage remain the
co-seal keys.

## Minimum neutral vocabulary to decide before I0

The D0 must choose one complete, profile-neutral row vocabulary sufficient for
future body observers. At minimum, it must answer:

```text
body statement kind and exact source site
expression kind and parent/child relation
ordinary return and exact returned-value site (or Unit)
receiver lexical read versus field/index/method-call read
field identity, if direct receiver state is claimed
write/alloc/IO/FFI/call/await/suspension markers
qmark/throw/panic/non-local control markers
complete body-root and nested-callable boundary coverage
```

A partial `receiver_read: bool` is not sufficient: the existing path model
shares receiver segments across field, method, and index forms, and the
canonical resolver currently discards field/method identity. If the first
bounded Query cohort is narrowed to receiver lexical reads only, record that
explicitly. If direct receiver state reads remain required, add a separate
field/state declaration issuer D0; do not use Builder-only field declarations
as resolver authority.

The neutral rows are fixed at this level for I0; they are not a second Recipe:

```text
BodyStatementShapeV1:
  SequenceItem { site }
  Return { site, value: Option<SourceExprSiteV1> }

BodyExpressionShapeV1:
  Variable { site, resolved: ResolvedLexicalRefV1 }
  Me { site, receiver: BindingRefV1 }
  FieldAccess { site, object: SourceExprSiteV1, field: Box<str> }
  MethodCall { site, object: SourceExprSiteV1, method: Box<str>, arity: u32 }
  Other { site, kind: BodyExpressionKindV1 }

BodyEffectShapeV1:
  Write | Allocation | Call | Await | QMark | Throw | NonLocalControl

BodyShapeRelationV1:
  parent statement/expression site -> exact child site + role
```

The first I0 cohort is explicitly narrowed to receiver lexical reads (`Me` /
resolved receiver binding) and ordinary return value relations. Direct
receiver field/state reads remain outside I0 until a separate resolver-owned
field/state declaration issuer D0 exists; Builder-only field declarations are
never resolver authority.

## Required product shape

The public semantic boundary should be one catalog-level, non-`Clone`
aggregate with a canonical issuer, for example:

```text
VerifiedResolvedBodyShapeInventoryV1
  - same parser/resolver brands as the owner carrier
  - exact declaration/body-root identity
  - complete ordered body rows
  - neutral operation/effect/control/source-site evidence
  - no Query/Home/signature/ABI meaning
```

Private per-row DTOs are allowed, but callers must not receive independently
constructible `Verified*` fragments that can be recombined. Missing,
duplicate, foreign, opaque, or incomplete rows are rejected at the issuer;
there is no empty/default verified inventory.

## Disposition and fail-fast rules

```text
issuer not implemented / required vocabulary unavailable:
  NoSafeSlice (development state)

issuer exists but source evidence is opaque:
  Unresolved

fully observed body outside the bounded Query cohort:
  Declined

foreign owner/root/brand/site, duplicate or conflicting rows:
  Rejected

complete neutral shape:
  Candidate for the private Query projection only
```

The issuer must not infer a public Query contract from body shape. Query
selection remains the aggregate-owned selected view already landed.

## Negative matrix

```text
AST rescan after owner link
name/arity/ordinal/vector-order pairing
method-call or field identity discarded as a boolean read flag
return without returned-value relation
missing or duplicate body rows
nested callable body counted in the parent
foreign parser provenance or resolver brand
wrong body root/source kind
write, allocation, IO, FFI, await, suspension, qmark, throw, panic
non-local break/continue/return escape
Builder-only field declaration used as source authority
empty/default verified inventory
```

## Implementation order after this D0

```text
1. implement the resolver-session shape ledger and canonical conversion
2. add the bounded receiver-lexical-read/ordinary-return positive fixture
3. add exact negative coverage and same-session/provenance guards
4. update module/reference receipts in the same I0 commit
5. reopen CALLABLE-BODY-FACTS-QUERY-I0 as a private projection only
6. keep direct field/state authority, body conformance, publishable catalog,
   target, Recipe/CallSlot, MIR, and production closed
```

This D0 authorizes only the bounded I0 above. It does not authorize direct
field/state reads, generic body facts, conformance, target, Recipe/CallSlot,
MIR, or production.

## Non-claims

```text
no AST-bearing resolver product
no second parser transport
no second FunctionOwner issuer
no Query/Home/signature/ABI re-issuance
no body conformance
no target/source-bound call/Recipe/CallSlot
no Builder/MIR/CFG/PHI/physicalization
no fallback/retry/provider/runtime dispatch
```
