---
Status: accepted_design__2026-09-21__MainImportViewOwnership
Task: MIR-CALL-D1B-MAIN-IMPORT-VIEW-OWNERSHIP-D0
Date: 2026-09-21
Parent: mir-call-d1b-resolved-qualified-receiver-catalog-coissue-i0-2026-09-21.md
Implementation permission: accepted bounded transport; next I0 may implement
  the single lifecycle-owned view and relation-only Main handoff
NextCard: MIR-CALL-D1B-MAIN-IMPORT-VIEW-OWNERSHIP-I0
---

# Main import-view ownership D0

## Six-line brief

```text
Decision: choose one invocation-owned import relation for App Main and one
  transport into the semantic package/co-issuer; remove the empty/Script-only
  split before qualified receiver catalog co-seal resumes.
Source authority + canonical issuer: the invocation's using_import_boxes plus
  the same source-backed declaration catalog; one owner seals the exact view.
Non-authority: Script whole-source inventory, package-local empty views, AST
  rescans, name/arity lookup, MIR, compatibility routes, and target inference.
Fail-fast boundary: catalog brand, import alias/canonical owner, duplicate and
  foreign rows, invocation identity, and exactly-once view transport.
Smallest next slice: census the lifecycle/package ownership seam and decide
  whether the existing view can be borrowed once or must become one owned row.
Non-claims: no qualified target/loan/publication, Recipe/ABI/MIR lowering,
  backend parity, production switch, or legacy deletion.
```

## Current evidence and finite boundary

`normal_default_root_catalog_lifecycle.rs` currently converts
`using_import_boxes` for `ScriptDirectStaticCallLookupIssuerV1`, which seals a
`VerifiedStaticImportAliasViewV1` only for the Script route. The normal package
`core_method_source.rs` independently seals an empty view. No Main invocation
product transports a non-empty import view into the resolver/package handoff.

The boundary is one source-backed App Main invocation, its exact declaration
catalog brand, and its finite `using_import_boxes` rows. The decision must
preserve one import authority for the later qualified receiver co-issuer and
must state how Script lookup consumes the same relation without re-sealing a
competing authority. If no existing owner can do this with one borrowed or
owned product, record `NoSafeSlice` and leave the qualified catalog row parked.

## Accepted decision (2026-09-21)

`ModuleBuilderInvocationSession`'s `using_import_boxes` is the invocation
source. `normal_default_root_catalog_lifecycle.rs` is the canonical issuer: it
will seal one `VerifiedStaticImportAliasViewV1` against the already-issued
package declaration catalog after the package exists, then borrow that exact
view into both the Script lookup and the Main qualified-receiver co-issuer.
The view is never stored self-referentially in the package. The Main co-issuer
returns an owned AST-free relation, which the package retains for the later
target/loan row.

`ScriptDirectStaticCallLookupIssuerV1` must consume the borrowed view and stop
sealing its own copy. `core_method_source.rs` remains a brand-only source
route with no qualified import facts; its empty construction must be named as
brand-only rather than treated as an import authority. Duplicate aliases,
empty/foreign owners, catalog-brand drift, and view reuse outside the same
invocation remain typed rejection conditions.

The worker consultation confirmed the exact owner and transport seam. The
existing `VerifiedStaticImportAliasViewV1` borrows the declaration catalog, so
only the lifecycle can share it safely across both consumers; the package
stores only the owned relation rows.

## Required decision checks

1. Name the issuer and prove it sees the exact invocation and declaration
   brand.
2. Reject duplicate aliases, empty/foreign canonical owners, and brand drift
   before any consumer receives the view.
3. Prove exactly-once transport into the Main package/co-issuer and define the
   Script consumer's relationship; do not pass raw `HashMap` rows downstream.
4. Keep `VerifiedQualifiedCallRouteFactsV1` and whole-source Script inventory
   out of Main; they require AST-backed source products.
