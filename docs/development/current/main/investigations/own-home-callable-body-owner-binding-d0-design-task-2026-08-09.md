---
Status: accepted design; current design stop after owner-carrier I0
Date: 2026-08-09
Parent: `docs/development/current/main/investigations/own-home-callable-body-source-d0-design-task-2026-08-09.md`
Authority: `docs/reference/language/callable-contracts.md`
---

# CALLABLE-BODY-OWNER-CARRIER-D0 / CALLABLE-BODY-OWNER-BINDING-D0

## Purpose

Bind the AST-free instance-method body-source catalog to the exact resolved
function product before any body observer may read lexical/control facts.
The current repository cannot open the link directly: `VerifiedResolvedFunctionV1`
does not carry the Box/method source identity, parser provenance, or an exact
body-root/coverage receipt. `FunctionOriginV1` is only a compilation-unit and
function ordinal and is not a source identity.

Therefore this design stop is deliberately two-stage:

```text
CALLABLE-BODY-OWNER-CARRIER-D0
  resolver-session-owned instance-method function carrier/catalog
  issued on the exact method-resolution path

CALLABLE-BODY-OWNER-BINDING-D0
  selected Query body source
  + that carrier/catalog
  + exact resolved function products
  -> one-to-one relational co-seal
```

```text
VerifiedDeclaredQueryBodySourceCatalogV1
  + selected contract references carried by its rows
  + resolver-issued instance-method function carrier/catalog
  + exact `VerifiedResolvedFunctionV1` products
      -> VerifiedInstanceMethodBodyOwnerCatalogV1
```

`FunctionSemanticResolverSessionV1` remains the sole issuer of
`FunctionOwnerIdV1`. The carrier issuer must run on the same resolver session
path that resolves the instance method, and must co-seal (or borrow) the exact
declaration identity, normalized source site, parser provenance, resolver
brand, nominal Box identity, `FunctionOriginV1`, owner-bearing resolved
function, and a resolver-issued body-root/item-coverage receipt. It must not
join separately-built functions by name, ordinal, inventory placement, or
compilation brand.

Only after that carrier exists may the owner-link issuer verify the selected
Query body-source rows against the carrier and exact resolved functions. The
link issues no owner and does not copy `FunctionOwnerIdV1`; it is a relational
co-seal only. A selected-gate/source-path receipt is outside the bounded direct
Rust cohort and requires a separate source-seal decision.

## Input boundary

The general body-source catalog is produced first and remains the source
authority for all supported direct declarations. This first owner-binding
cohort consumes the declared Query projection; it does not perform Query
selection itself. The selected Query row retains the normalized source site,
parser provenance, resolver brand, and body-item coverage needed for this
co-seal. A future non-Query body observer may use the same owner-link shape
only through a separately accepted input boundary.

## Scope

Allowed after the carrier row is closed:

```text
one ordinary direct Rust Box method
one exact resolver-issued instance-method function carrier
one exact resolved function product
same-brand declaration/body/source relation
AST-free non-Clone owner link
```

Not opened:

```text
body behavior facts
Query/Pure conformance
Home Flow or ABI issuance
resolver target
Recipe/CallSlot
Builder/MIR/CFG/PHI
runtime/provider/fallback
```

## Reject boundaries

```text
foreign resolver/catalog/parser brand       -> Rejected
method/site/body-item coverage mismatch     -> Rejected
owner/source origin mismatch                -> Rejected
duplicate or missing function owner         -> Rejected
unresolved body/function source             -> Unresolved
unsupported selected/Hako/interface cohort  -> NoSafeSlice
```

Equal owner numbers, method names, source ordinals, inventory placement, or
compilation brands are never sufficient evidence. Parser provenance and
resolver brand are inherited from resolver-issued products; they are not
caller-supplied or forged per row. Body facts remain closed until this link is
sealed.

## Required carrier contract before owner-link I0

The carrier/catalog D0 must define a resolver-issued product analogous to the
existing resolved callable function-unit pattern, without changing the
generic `VerifiedResolvedFunctionV1` into a Box-specific type. A suitable
shape is:

```text
VerifiedResolvedInstanceMethodFunctionUnitV1
  declaration identity / normalized source site
  parser provenance
  resolver brand / nominal Box identity
  FunctionOriginV1
  owner-bearing VerifiedResolvedFunctionV1
  resolver-issued body-root and ordered item-coverage receipt
```

The exact public name is decided by the carrier D0. The issuer must be on the
same `FunctionSemanticResolverSessionV1` method-resolution path and must not
have a public arbitrary constructor. If the resolver cannot issue the body
root/coverage receipt without AST re-scan or name lookup, the carrier row
remains `NoSafeSlice`; do not invent an empty or test-only verified product.

## Accepted carrier issuer boundary

The carrier D0 is now closed with the following source lease boundary. The
parser transaction owns the AST until a single higher-ranked-resolver callback
returns; the callback is the only place where a function syntax view may be
constructed.

```text
ParserResolverBodyTransactionV1::with_direct_method_syntax(self, callback)
  callback receives, for one parser invocation:
    ParserBoxResolverSourceHandoffV1
    ParserBoxBodySourceEnvelopeV1
    ParserBoxInstanceMethodSyntaxLeaseV1<'ast>   // parser-private only

  callback returns only AST-free resolver products
  transaction and syntax lease cannot escape or be reused
```

The syntax lease contains exact normalized source coordinates plus borrowed
parameter/body slices. It is not a resolver product, is not `Clone`, and is
never stored in a body-source catalog. The resolver constructs
`FunctionSyntaxViewV1` from the lease, runs the existing
`FunctionSemanticResolverSessionV1` owner-forest issuer, and emits a
resolver-owned instance-method function unit/catalog containing:

```text
declaration/source identity
parser provenance / resolver brand / nominal Box identity
owner-bearing resolved forest/function product
FunctionOriginV1 as a consistency receipt
body-root profile and exact ordered body-item coverage
```

The carrier issuer and body-source issuer consume the same parser provenance;
the owner-link issuer later co-seals their AST-free catalogs. No AST pointer,
method-name lookup, inventory ordinal lookup, caller-built function map, or
second `FunctionOwnerIdV1` issuer is permitted.

## Carrier I0 receipt and next stop

`CALLABLE-BODY-OWNER-CARRIER-I0` is closed. Its focused slice issues one
AST-free direct-method carrier through the transaction-scoped syntax lease and
the existing owner-forest resolver, retaining source identity, both brands,
nominal Box, root/body receipts, and contiguous body coverage. The focused
tests cover direct success, empty-body coverage, and foreign parser
provenance. The carrier does not select Query rows or bind a body source.

The current design stop is this document's owner-link D0. It must define the
catalog-level relation before any owner-link I0 implementation starts.

## Acceptance and closeout

* carrier D0 first fixes the source-bound resolver issuer and body-root/
  coverage receipt;
* owner-link I0 then lands positive and foreign/mismatch/duplicate/missing
  negative tests together;
* the carrier and owner link are non-`Clone` where they cross the semantic
  boundary and have no public arbitrary constructor;
* no body observer, conformance, target, Recipe, or MIR import is added;
* `src/mir/resolved_semantics/README.md`,
  `docs/reference/language/callable-contracts.md`, this task map, and current
  mirrors are updated in the same implementation slice;
* all touched source files remain below the 760-line split trigger and below
  800 lines.

## Explicit task order

```text
1. CALLABLE-BODY-OWNER-CARRIER-D0 (closed)
   exact resolver callback/source-lease path, source identity,
   parser/resolver brands, body-root/item-coverage receipt, no second
   FunctionOwner issuer

2. CALLABLE-BODY-OWNER-CARRIER-I0
   one real direct instance-method carrier and focused negative matrix

3. CALLABLE-BODY-OWNER-BINDING-D0
   selected Query projection + carrier + resolved function co-seal

4. CALLABLE-BODY-OWNER-BINDING-I0
   catalog-level one-to-one owner link; no body facts/conformance

5. CALLABLE-BODY-FACTS-QUERY-D0/I0
```

Before the carrier I0 is landed, the missing issuer is represented as
development `NoSafeSlice`; after the carrier I0 closes, the owner link remains
parked behind its own D0/I0 and must not be implemented against bare
`VerifiedResolvedFunctionV1`.
