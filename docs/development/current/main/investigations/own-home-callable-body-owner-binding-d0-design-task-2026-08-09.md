---
Status: parked — implementation opens only after Query projection I0 and accepted owner-binding D0
Date: 2026-08-09
Parent: `docs/development/current/main/investigations/own-home-callable-body-source-d0-design-task-2026-08-09.md`
Authority: `docs/reference/language/callable-contracts.md`
---

# CALLABLE-BODY-OWNER-BINDING-D0/I0

## Purpose

Bind the AST-free instance-method body-source catalog to the exact resolved
function product before any body observer may read lexical/control facts.
This is a relational co-seal, not a second `FunctionOwnerIdV1` issuer.

```text
VerifiedDeclaredQueryBodySourceCatalogV1
  + selected contract references carried by its rows
  + VerifiedResolvedFunctionV1
      -> VerifiedInstanceMethodBodyOwnerLinkV1
```

`FunctionSemanticResolverSessionV1` remains the sole issuer of
`FunctionOwnerIdV1`. For the bounded direct cohort, the owner link must verify
the resolver-normalized `ResolverBoxMethodSourceSiteV1` coordinate covered by
the enclosing resolver brand and parser provenance, nominal Box/method
identity, body-item coverage, and exact
`VerifiedResolvedFunctionV1::function_origin()` relation. A raw ordinal alone
is not identity. A selected-gate/source-path receipt or a separate body-root
token is outside this cohort and requires a later source-seal decision; this
row must not invent one.

## Input boundary

The general body-source catalog is produced first and remains the source
authority for all supported direct declarations. This first owner-binding
cohort consumes the declared Query projection; it does not perform Query
selection itself. The selected Query row retains the normalized source site,
parser provenance, resolver brand, and body-item coverage needed for this
co-seal. A future non-Query body observer may use the same owner-link shape
only through a separately accepted input boundary.

## Scope

Allowed:

```text
one ordinary direct Rust Box method
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
resolver brand are inherited from the enclosing selected body/contract views;
they are not forged per row. Body facts remain closed until this link is
sealed.

## Acceptance and closeout

* positive and foreign/mismatch/duplicate/missing negative tests land together;
* the owner link is non-`Clone` and has no public arbitrary constructor;
* no body observer, conformance, target, Recipe, or MIR import is added;
* `src/mir/resolved_semantics/README.md`,
  `docs/reference/language/callable-contracts.md`, this task map, and current
  mirrors are updated in the same implementation slice;
* all touched source files remain below the 760-line split trigger and below
  800 lines.
