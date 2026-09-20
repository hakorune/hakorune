---
Status: design_stop__2026-09-21__ResolvedQualifiedReceiverIdentity
Task: MIR-CALL-D1B-RESOLVED-QUALIFIED-RECEIVER-IDENTITY-COSEAL-D0
Date: 2026-09-21
Parent: mir-call-d1b-resolved-method-static-target-coissue-d0-2026-09-21.md
Implementation permission: false; resolver/source relation design only
NextCard: none
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

## Design alternatives

The preferred shape is a private resolver-owned receiver identity relation that
is issued in the existing MethodCall traversal and immediately co-sealed with
the source-backed catalog/import brand. It may carry source identity and
canonical-owner evidence, but not a target, ABI, Recipe key, ValueId, or
physical symbol. A second AST walk or a consumer-side name lookup is rejected;
if the existing traversal cannot issue the relation without either, this row is
`NoSafeSlice` and the Main qualified family remains typed-reject/parked.

## Acceptance and reopen

Acceptance requires one exact relation per qualified site, same owner and brand,
explicit lexical/alias precedence, complete argument-site coverage, and typed
rejection of missing, foreign, duplicate, dynamic, nested, and wrong-kind rows.
Implementation may open only after this relation can be consumed once by the
existing source catalog owner; no target/loan/Call publication is authorized by
this D0.
