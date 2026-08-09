---
Status: closed design 2026-08-09 — general body-source boundary landed
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
  + parser-issued direct source site
  + parser invocation brand
  + exact method body-item coverage/order
        |
        v
resolver body-source issuer
  + resolver declaration catalog identity
  + nominal Box/method identity
        |
        v
VerifiedInstanceMethodBodySourceCatalogV1
  = non-Clone, AST-free body source capability for every
    supported direct instance declaration
  = one exact body source row per declaration identity
        |
        v
VerifiedDeclaredQueryBodySourceCatalogV1
  = deterministic projection of the declared Query selection
  = one exact body row per selected Query declaration
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
  parser-issued direct source rows
  resolver-normalized source-site coordinates
  ordered body-item coverage/cardinality
  private syntax arena/view used only by one observer callback
```

The envelope is the only legal way to pair the returned AST/body syntax with
the AST-free declaration handoff. An API that returns an AST and a body-free
handoff as independent values and later pairs them by name, ordinal, or
`ASTNode` search is not a body-source authority. The existing declaration-only
API remains for declaration/signature consumers; the body envelope is a new
parser-private ingress for this row.

The resolver-facing result is a fresh non-`Clone`
`VerifiedInstanceMethodBodySourceCatalogV1`. It contains only the resolver
brand, parser provenance, resolver-normalized direct method-site coordinate,
and complete ordered body-item coverage. It does not contain an `ASTNode`, a
`FunctionOwnerIdV1`, body effects, Query, Home, semantic types, target,
Recipe, or MIR. The private observer callback may borrow the syntax arena while
the envelope is alive; the callback must finish before the envelope is dropped.

The general body-source issuer does not inspect or reissue Query behavior. A
separate `VerifiedDeclaredQueryBodySourceCatalogV1` projection borrows the
already sealed selected Query view from
`VerifiedDeclaredInstanceMethodContractCatalogV1`. It requires exactly one
validated body row per selected Query declaration, preserves sparse source
order and branded source identity, and emits no default row for non-Query
declarations. A foreign or duplicate row is rejected before projection.

The first cohort is deliberately limited to an ordinary top-level Rust Box
with explicit direct instance methods. Selected-gate, Hako, interface, static,
record, mixed, generated, and compatibility rows remain `NoSafeSlice` until a
separate source-path decision supplies their exact body coordinates.

## Top-down audit corrections (2026-08-09)

The body-source design is accepted only with the following five boundaries
closed. These are design contracts, not permission to start I0.

### One-shot parser transaction

The declaration handoff and body rows must be decomposed exactly once from the
same rich parse product. The parser-private transaction is the only pairing
authority:

```text
ParserResolverBodyTransactionV1  (non-Clone, parser-private)
  owns rich AST, source seal, parser invocation brand, and body rows
        |
        +-- into_parts(self)
              -> ParserBoxResolverSourceHandoffV1
              -> ParserBoxBodySourceEnvelopeV1
```

`into_parts` is a consuming operation that returns the two AST-free parts of
one branded transaction; it never returns the AST. The handoff and envelope
may be passed to their separate issuers only after the issuer verifies the
same parser provenance and complete source-site coverage; swapping parts from
different transactions is `Rejected`. The existing declaration-only parser API
remains a projection for declaration consumers, but the body path may not
rescan or reconstruct its input from that projection. The body envelope owns
only normalized AST-free body DTOs after decomposition; its one-shot callback
must complete before the envelope is dropped and may not return syntax or an
AST pointer.

### Branded source identity and provenance

For the bounded direct cohort, the parser-issued source site is normalized at
the resolver boundary to `ResolverBoxMethodSourceSiteV1` (Box statement
ordinal plus direct member ordinal) and remains branded by the parser
provenance and covered by the resolver catalog brand. This is a
resolver-normalized source-site coordinate, not a bare ordinal. A method name,
selected/generated inventory ordinal, or name-sorted map is not a source
identity. The direct member ordinal inside the coordinate is allowed; using
that integer alone is forbidden.

Selected build-gate paths or generated/delegate origins are outside this I0.
If they must become source identity in a later cohort, a separate parser
source-seal decision must add and validate that path receipt; this row must not
silently claim it.

The parser body envelope and the resolver body-source catalog must retain a
checked parser-invocation provenance token (or an issuer-issued comparison
receipt). `same resolver brand` alone is insufficient to detect a foreign
parser transaction. Resolver comparison is performed through a sealed
`same_as`/co-seal API; callers cannot forge or reinterpret the token.

### Selected identity and cardinality

The body envelope may carry all rows in the bounded direct cohort. The general
body-source issuer first requires exactly one body row for every declaration
identity in that cohort, rejecting duplicate, missing, foreign, or
contradictory rows. It does not select Query methods.

The separate Query projection borrows the aggregate's already sealed selected
view and requires exactly one body row for each selected Query declaration.
For a Query/non-Query/Query source, the projection contains the first and
third source rows with their original identities; it does not rebase ordinals,
sort by name, or create a default row for the non-Query declaration. Query
selection is never rebuilt from rune syntax, names, or inventory placement.

### Function-owner boundary

`CALLABLE-BODY-SOURCE-AUTHORITY-I0` issues only AST-free source/body identity
and ordered coverage. Before any body facts that need lexical/control facts,
close the explicit carrier row and then the owner-link row:

```text
CALLABLE-BODY-OWNER-CARRIER-D0/I0
  resolver-issued instance-method function carrier/catalog

CALLABLE-BODY-OWNER-BINDING-D0/I0
  VerifiedInstanceMethodBodySourceCatalogV1
  + resolver-issued instance-method function carrier/catalog
  + exact VerifiedResolvedFunctionV1 products
  + declaration/catalog identity
  -> one catalog-level branded body-owner link
```

`FunctionSemanticResolverSessionV1` remains the sole issuer of
`FunctionOwnerIdV1`. Equal owner numbers, source ordinals, names, or
compilation brands never establish the link.

### Source-module split boundary

Body fields must not be appended to `source_seal.rs`, `source_authority.rs`, or
`parser/mod.rs`. The I0 row owns dedicated parser body-envelope/row modules
and one dedicated resolver body-source module. The existing files receive
only minimal module wiring. If wiring would cross the 760-line split trigger,
stop and split before implementation; do not hide the growth in a private
helper.

## Function-owner binding is a later boundary

`FunctionSemanticResolverSessionV1` is the sole issuer of
`FunctionOwnerIdV1`/`VerifiedResolvedFunctionV1`. Body-source I0 never mints,
copies, or reinterprets that owner. A preceding carrier D0/I0 must be issued
on the same method-resolution path and retain exact declaration/source
identity, parser provenance, resolver brand, nominal Box identity,
`FunctionOriginV1` as a consistency receipt, the owner-bearing resolved
function, and a resolver-issued body-root/item-coverage receipt. Only then may
the later `InstanceMethodBodyOwnerBindingIssuer` co-seal:

```text
VerifiedInstanceMethodBodySourceCatalogV1
+ resolver-issued function carrier/catalog
+ exact resolved function products
+ declaration/catalog identity
  -> one catalog-level branded instance-method body-owner link
```

Equal-looking owner numbers, `FunctionOriginV1`, source ordinals, method names,
or compilation brands are insufficient. A caller-built map or legacy AST/name
lookup is not an issuer. Until the carrier and link issuers exist, body facts
that need resolved lexical/control facts remain `NoSafeSlice`.

## Input ownership

The general body-source issuer borrows the already landed
`VerifiedInstanceMethodDeclarationCatalogV1`; it does not consume or clone
the declaration catalog. The separate Query projection borrows the already
sealed `VerifiedDeclaredInstanceMethodContractCatalogV1` selected view and
does not consume or clone its Home/Query rows. Both aggregates remain
available for later owner/conformance co-seals. The one-shot move belongs to
the parser body envelope, not to a semantic aggregate.

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

## External review correction (2026-08-09)

The earlier draft combined general body-source authority and Query selection.
The accepted ordering is now explicitly:

```text
ParserResolverBodyTransactionV1
  -> ParserBoxResolverSourceHandoffV1
  -> ParserBoxBodySourceEnvelopeV1
       -> VerifiedInstanceMethodBodySourceCatalogV1
            -> VerifiedDeclaredQueryBodySourceCatalogV1
                 -> body-owner binding
                      -> body facts / conformance
```

The parser envelope is AST-free after decomposition. If a future observer
needs a private syntax view, it may borrow that view only inside the one-shot
callback; the resolver-facing catalogs never retain an AST or syntax pointer.
The body-source I0 therefore proves declaration/body identity and ordered
coverage for the complete supported direct cohort. Query subset projection is
its own bounded design/implementation row, and FunctionOwner binding remains
the following relational co-seal.

`VerifiedResolvedFunctionV1` may supply resolved lexical/control facts only
after it is carried by a resolver-issued instance-method unit and explicitly
co-sealed to the same Box method declaration. An equal `FunctionOwnerIdV1`,
`FunctionOriginV1`, source ordinal, or method name is not sufficient.

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

## Disposition and precedence

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

The precedence is fixed:

```text
unsupported source cohort / missing issuer -> NoSafeSlice
foreign or contradictory identity         -> Rejected
incomplete facts with intact identity     -> Unresolved
fully observed body outside Query cohort  -> Declined
complete bounded source + facts            -> Candidate
```

Thus `selected/generated/Hako/interface/static` is `NoSafeSlice` in this row,
while a body that is fully observed by a future issuer but is outside the
bounded Query meaning is `Declined`. The two outcomes must not be conflated.

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
   source authority census, normalized source-site/body-item contract,
   parser body envelope, resolver borrow boundary, owner-link stop line, and
   fail-fast matrix, one-shot transaction decomposition, provenance bridge,
   selected identity/cardinality, owner-binding insertion, and module split
   boundary (this row)

2. CALLABLE-BODY-SOURCE-AUTHORITY-I0
   one bounded `length(): i64` instance-method body source capability;
   resolver-only, AST-free after handoff, no conformance yet. This row stays
   `NoSafeSlice` until the parser body envelope and its one-shot callback are
   implemented; do not add a test-only constructor.

3. CALLABLE-BODY-OWNER-BINDING-D0/I0
   co-seal the AST-free body source with the exact resolved function product;
   no body facts yet

4. CALLABLE-BODY-FACTS-QUERY-D0
   exact receiver-read/return observation and body-facts owner

5. CALLABLE-BODY-FACTS-QUERY-I0
   positive/negative body facts for the bounded fixture; no contract co-seal

6. CALLABLE-CONFORMANCE-CATALOG-D0/I0
   complete same-brand declared-contract + body-facts co-seal;
   issue `VerifiedConformantCallableCatalogV1`

7. only after 6:
   resolver target -> source-bound relation -> Recipe CallSlot
```

Every implementation row must update its focused tests, owner README, and the
callable-contracts reference in the same commit. No row may add a public
`Verified*` constructor without its canonical issuer and negative matrix.
