---
Status: accepted_design__2026-09-21__ResolvedQualifiedReceiverIdentityCarrier
Task: MIR-CALL-D1B-RESOLVED-QUALIFIED-RECEIVER-IDENTITY-COSEAL-D0
Date: 2026-09-21
Parent: mir-call-d1b-resolved-method-static-target-coissue-d0-2026-09-21.md
Implementation permission: false; carrier design accepted, implementation delegated
NextCard: MIR-CALL-D1B-RESOLVED-QUALIFIED-RECEIVER-IDENTITY-I0
---

# Resolver qualified receiver identity co-seal D0

## Six-line brief

```text
Decision: preserve the exact qualified receiver identity during the existing
  resolver source traversal so a later catalog join can be exact; do not scan
  AST again or turn the identity into a target.
Source authority + canonical issuer: the existing shadow/resolver MethodCall
  traversal plus the same-session source-backed declaration/import catalog;
  one resolver-owned source relation is the only proposed issuer.
Non-authority: receiver spelling read after the traversal, static_candidates,
  name/arity uniqueness, physical symbols, Script inventory, Raw lineage, and
  package-side repair.
Fail-fast boundary: receiver site, callable owner, lexical disposition,
  alias/canonical-owner relation, catalog brand, and method/argument sites must
  be complete and unique before any target or argument effect.
Smallest next slice: define the minimal AST-free receiver identity carrier and
  its same-session catalog/import co-seal; reject unsupported receiver shapes and
  nested-owner crossings explicitly.
Non-claims: no target/Callee, package field, loan, dispatcher, fallback,
  backend, JSON, Call schema, or production switch.
```

## Finite boundary

Only qualified canonical-owner/import-alias MethodCall expressions in the
installed source-backed App Main root are included. The relation must retain the
exact receiver source identity, caller/site, selector, argument sites, lexical
disposition, and catalog/compilation brand. Bare FunctionCall, `me.method`,
instance methods, dynamic receivers, nested lambdas, ScriptRoot, and raw
compatibility are excluded.

## Current gap

`VerifiedResolvedMethodCallSourceV1` currently stores site, receiver class,
arguments, selector, and arity, but `QualifiedUnbound` carries no receiver name
or alias identity. `BodyExpressionShapeV1::QualifiedReceiver` likewise keeps
only the source site. The existing `VerifiedQualifiedCallRouteFactsV1` can derive
the receiver and import mapping only from its AST-backed source-call product.
That split prevents an exact same-session resolver-to-catalog join today.

## Accepted decision

The read-only same-traversal audit found a bounded carrier slice. The existing
shadow resolver already has `ASTNode::Variable { name, .. }` at
`shadow/expr.rs:454-487`; the loss occurs only when
`body_shape_resolver.rs` reduces the row to a site. Retain that exact source
name in the existing body-shape row only for a receiver proven
`QualifiedUnbound`, then co-seal it into the existing
`VerifiedResolvedMethodCallSourceV1` as a private source-identity relation.
This reuses the current resolver issuer and does not create a second traversal,
target, ABI, Recipe key, or physical symbol.

The carrier slice must reject missing/duplicate identity, lexical-bound
receivers, dynamic/`me` receivers, wrong-kind rows, and nested-owner crossings
at the existing resolver/body-shape boundary. The later import-alias and
canonical-owner co-seal remains a separate D0 because the resolver receives a
brand catalog but not `VerifiedStaticImportAliasViewV1`; that view is built by
the existing source catalog owner from the invocation imports.

Consultation evidence: read-only audit
`receiver_identity_same_traversal_audit` on 2026-09-21. It identified the
bounded carrier files and confirmed that the Script AST-backed inventory cannot
be reused as Main authority.

## Design alternatives

The preferred shape is a private resolver-owned receiver identity relation that
is issued in the existing MethodCall traversal and immediately co-sealed with
the source-backed catalog/import brand. It may carry source identity and
canonical-owner evidence, but not a target, ABI, Recipe key, ValueId, or
physical symbol. A second AST walk or a consumer-side name lookup is rejected;
if the existing traversal cannot issue the carrier without either, this row is
`NoSafeSlice` and the Main qualified family remains typed-reject/parked. The
audit resolved that question for the carrier only; full catalog co-seal remains
the next design dependency.

## Acceptance and reopen

Acceptance for this D0 is the accepted carrier contract above and one bounded
implementation row. Exact catalog/import-brand co-seal, target/loan/Call
publication, and production caller selection remain outside this decision and
are owned by `MIR-CALL-D1B-RESOLVED-QUALIFIED-RECEIVER-CATALOG-COISSUE-D0`.
