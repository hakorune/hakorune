---
Status: design_stop__2026-09-21__ResolvedMethodStaticTargetCoIssue
Task: MIR-CALL-D1B-RESOLVED-METHOD-STATIC-TARGET-COISSUE-D0
Date: 2026-09-21
Parent: mir-call-d1b-main-raw-qualified-method-source-coseal-d0-2026-09-21.md
Implementation permission: false; issuer and traversal design only
NextCard: none
---

# Resolver MethodCall static-target co-issue D0

## Six-line brief

```text
Decision: decide whether the resolver's existing MethodCall relation and the
  source declaration/import catalog can co-issue one qualified static target;
  never repair the relation from names or raw lineage.
Source authority + canonical issuer: one FunctionSemanticResolverSessionV1
  source traversal plus the existing declaration/import catalog owner; the
  resulting AST-free site-to-key relation must be consumed by Main and Script
  without a second target issuer.
Non-authority: WholeSourceStaticCallTargetInventoryV1 as an independent retry,
  ScriptDirectStaticCallLookupIssuerV1 after package install, raw lineage,
  ResolvedDirectCallObservationV1, candidate uniqueness, and physical symbols.
Fail-fast boundary: exact caller/site, receiver disposition, alias/catalog row,
  method declaration, arguments, brand, and owner must be one-to-one before
  target publication or argument effects.
Smallest next slice: map the existing resolver `VerifiedResolvedMethodCallSourceV1`
  rows to catalog/import declarations in the same traversal and prove all
  missing/foreign/duplicate/wrong-kind cases; if impossible, seal NoSafeSlice.
Non-claims: no code, new public receipt, target/Callee emission, loan,
  dispatcher, fallback, backend, JSON, or Call-schema change.
```

## Finite census

The boundary is qualified canonical-owner/import-alias `MethodCall` rows owned
by the installed source-backed App Main root. It includes the resolver source
site, receiver disposition, selector and argument sites, source catalog brand,
import aliases, and the declaration key. It excludes bare FreeStatic calls,
`me.method` CoreMethod, instance methods, nested-owner inheritance, ScriptRoot,
RawCompatibility, and result publication.

## Existing products and missing relation

`VerifiedResolvedMethodCallSourceV1` already preserves the exact AST-free site,
receiver relation, selector, arity, and argument sites. The source-backed
catalog already owns declaration identity, import aliases, and catalog brand.
The missing join is the same-session qualified receiver/alias to
`CanonicalSameModuleCallableKeyV1` relation without rerunning
`observe_method_calls_shadow_view_v0`. The separate Script inventory has that
join but performs its own observation and is therefore not a free Main issuer.

## Decision boundary

This D0 must choose one of two outcomes. The accepted path is a private
co-issuer in the existing resolver/source catalog session that produces an
AST-free, site-keyed relation and is consumed by both selected routes. The
reject path records `NoSafeSlice` if the source catalog cannot receive the
resolver rows without a second traversal, a new parallel semantic authority,
or a public receipt. Neither outcome opens target publication or raw loan
transport.

## Acceptance and reopen

Acceptance requires one owner/site bijection, same catalog/compilation brand,
explicit lexical and alias precedence, complete coverage, and typed rejection
of foreign, duplicate, missing, nested, wrong-kind, and mixed-session rows. A
local Script test or a name/arity match is insufficient. Reopen implementation
only after this relation is issued once and a later atomic row names the
pre-effect target consumer and affine loan lifetime.
