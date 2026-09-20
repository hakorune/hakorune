---
Status: design_stop__2026-09-21__ResolvedQualifiedReceiverCatalogCoissue
Task: MIR-CALL-D1B-RESOLVED-QUALIFIED-RECEIVER-CATALOG-COISSUE-D0
Date: 2026-09-21
Parent: mir-call-d1b-resolved-qualified-receiver-identity-i0-2026-09-21.md
Implementation permission: false; catalog/import co-issuer design only
NextCard: none
---

# Resolver qualified receiver catalog co-issue D0

## Six-line brief

```text
Decision: determine how the resolver-owned receiver identity joins the existing
  declaration catalog and invocation import view exactly once for Main.
Source authority + canonical issuer: the existing resolver method-call row plus
  the invocation-owned VerifiedStaticImportAliasViewV1/declaration catalog;
  one source-catalog co-issuer must own the join.
Non-authority: Script AST inventory, name/arity lookup, physical symbols, MIR,
  compatibility routes, and consumer-side target reconstruction.
Fail-fast boundary: caller/site owner, receiver identity, import-view pointer,
  catalog brand, alias precedence, namespace, selector, and arity must agree.
Smallest next slice: census the Main invocation handoff and choose one existing
  route-facts owner that can consume the carrier without a second AST walk.
Non-claims: no target/loan publication, ABI/Recipe/MIR, dispatcher, fallback,
  backend parity, production switch, or legacy deletion.
```

## Current evidence and finite boundary

The resolver currently receives a brand declaration catalog but not the
invocation-built `VerifiedStaticImportAliasViewV1`. The existing qualified route
facts and target catalog already enforce lexical disposition, alias precedence,
catalog identity, static namespace, selector, and arity, but their source call
product is AST-backed. `normal_script_direct_static_lookup.rs` owns the Script
inventory and is not a Main authority.

The bounded boundary is qualified direct-variable `MethodCall` rows in the
source-backed App Main root. Instance/`me`, dynamic, nested lambda, reserved,
Script, and raw compatibility routes remain excluded.

## Required decision

Choose whether the existing route-facts owner can accept the resolver carrier
through a borrowed source-catalog seam, or whether a small resolver-to-catalog
co-issuer must be added. Reject any solution that re-reads the AST, resolves by
name/arity alone, duplicates import authority, or issues a target before the
exact relation is sealed. If no single owner can consume both products, seal
this family as `NoSafeSlice` rather than adding an adapter or fallback.
