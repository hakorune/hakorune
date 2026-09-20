---
Status: accepted_design__2026-09-21__ResolvedQualifiedReceiverCatalogCoissue
Task: MIR-CALL-D1B-RESOLVED-QUALIFIED-RECEIVER-CATALOG-COISSUE-D0
Date: 2026-09-21
Parent: mir-call-d1b-resolved-qualified-receiver-identity-i0-2026-09-21.md
Implementation permission: accepted bounded design; next I0 may implement the
  relation-only co-issuer, with no target/loan/publication claim
NextCard: MIR-CALL-D1B-MAIN-IMPORT-VIEW-OWNERSHIP-D0
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

The existing route-facts owner cannot accept the resolver carrier without
re-reading the AST, and the Script inventory is outside the Main authority.
Accept one small AST-free resolver-to-catalog co-issuer at the existing Main
package handoff. It borrows, exactly once, the resolver batch/source ledger,
the invocation-owned `VerifiedStaticImportAliasViewV1`, and the declaration
catalog, then seals only their qualified-receiver relation. It does not mint a
new target, loan, Recipe, ABI, physical symbol, or publication receipt.

The accepted finite shape is App Main's exact batch slot with one
`QualifiedUnbound` receiver identity and one resolver method-call row. An
alias must resolve through the sealed import view; an unaliased receiver must
match the existing direct-owner relation. Selector and arity come from the
resolver row and must match the catalog's static-box-method declaration. The
issuer rejects missing/duplicate identity, batch/site/brand mismatch, foreign
catalog or import view, duplicate alias, lexically bound/direct ambiguity,
wrong namespace, selector, or arity, plus instance/`me`, dynamic, nested,
reserved, Script, and raw routes.

The read-only worker audit confirmed that the existing
`VerifiedQualifiedCallRouteFactsV1` and `whole_source_inventory.rs` are AST
backed and therefore cannot be reused as Main authority. The next I0 may add
one private borrowed callback seam at the existing package/batch handoff; it
must not add a second traversal, name/arity target lookup, or compatibility
fallback. Target/loan/source-site publication remains a later row.

## Accepted decision (2026-09-21)

`MIR-CALL-D1B-RESOLVED-QUALIFIED-RECEIVER-CATALOG-COISSUE-I0` is the smallest
safe slice. Its implementation permission is limited to the relation product
and focused positive/negative guards. If the existing handoff cannot borrow
all three products with one owner, the I0 must close as `NoSafeSlice` rather
than introduce an adapter or new authority.

The I0 premise audit found that the required invocation-owned import view is
not present in the current Main path: `using_import_boxes` is converted and
sealed by the Script lookup path, while the package CoreMethod helper seals an
empty view independently. Reusing either would duplicate import authority or
silently give Main an empty view. The co-issuer relation therefore remains a
valid bounded design, but its implementation is blocked until
`MIR-CALL-D1B-MAIN-IMPORT-VIEW-OWNERSHIP-D0` selects one owner and transport
for that view.
