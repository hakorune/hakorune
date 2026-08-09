---
Status: revised design stop — owner carrier is missing; implementation not opened
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
1. CALLABLE-BODY-OWNER-CARRIER-D0
   exact resolver path, source identity, parser/resolver brands,
   body-root/item-coverage receipt, no second FunctionOwner issuer

2. CALLABLE-BODY-OWNER-CARRIER-I0
   one real direct instance-method carrier and focused negative matrix

3. CALLABLE-BODY-OWNER-BINDING-D0
   selected Query projection + carrier + resolved function co-seal

4. CALLABLE-BODY-OWNER-BINDING-I0
   catalog-level one-to-one owner link; no body facts/conformance

5. CALLABLE-BODY-FACTS-QUERY-D0/I0
```

Until steps 1–2 close, the current execution row remains `NoSafeSlice` and
the owner link must not be implemented against bare `VerifiedResolvedFunctionV1`.
