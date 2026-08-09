---
Status: parked — opens only after `CALLABLE-BODY-SOURCE-AUTHORITY-I0` lands
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
VerifiedInstanceMethodBodySourceCatalogV1
  + VerifiedResolvedFunctionV1
  + VerifiedInstanceMethodDeclarationCatalogV1
      -> VerifiedInstanceMethodBodyOwnerLinkV1
```

`FunctionSemanticResolverSessionV1` remains the sole issuer of
`FunctionOwnerIdV1`. The owner link must verify the same resolver/catalog
brand, nominal Box/method identity, complete branded `SourceBoxMethodSiteV1`,
parser provenance receipt, body-root identity, and exact function origin.

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
method/site/body-root mismatch              -> Rejected
owner/source origin mismatch                -> Rejected
duplicate or missing function owner         -> Rejected
unresolved body/function source             -> Unresolved
unsupported selected/Hako/interface cohort  -> NoSafeSlice
```

Equal owner numbers, method names, source ordinals, inventory placement, or
compilation brands are never sufficient evidence. Body facts remain closed
until this link is sealed.

## Acceptance and closeout

* positive and foreign/mismatch/duplicate/missing negative tests land together;
* the owner link is non-`Clone` and has no public arbitrary constructor;
* no body observer, conformance, target, Recipe, or MIR import is added;
* `src/mir/resolved_semantics/README.md`,
  `docs/reference/language/callable-contracts.md`, this task map, and current
  mirrors are updated in the same implementation slice;
* all touched source files remain below the 760-line split trigger and below
  800 lines.

