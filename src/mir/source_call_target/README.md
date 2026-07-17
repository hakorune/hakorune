# Source call target

This module owns disconnected, pre-Builder source-call target proofs.

## AST-BIND0-S0 exact source site

`VerifiedSourceMethodCallSiteV1` is the sole exact caller/body/site co-seal.
Its constructor accepts only the complete declaration catalog, one canonical
caller key, and one function-relative `SourceExprSiteV1`. The catalog supplies
the body; the shared resolved-semantics projector supplies the exact borrowed
node; only `ASTNode::MethodCall` is admitted. Receiver, method, arguments,
receiver site, and checked arity are derived from that node and can never be
supplied independently.

Sites that cross a nested `FunctionDeclaration` or `Lambda` callable boundary
reject before projection. The top-level catalog key must never be paired with
a nested callable body that has its own semantic owner.

The product is lifetime-bound and non-Clone. It owns no AST, body, lexical
fact, import alias, reserved-route decision, target, ABI, effect, result
representation, MIR, or runtime authority. S0 remains disconnected with zero
production consumers. Lexical Bound/ProvenUnbound observation belongs to the
following L0 row.

## AST-BIND0-L0 lexical disposition

`VerifiedQualifiedReceiverLexicalDispositionsV1` accepts a non-empty set of
S0 products for one exact catalog declaration. Every receiver must be the
pre-verified Variable at that MethodCall's derived receiver site. The product
runs the existing shadow lexical traversal once and publishes exactly one
`Bound` or `ProvenUnbound` row per requested site.

`Bound` deliberately carries no shadow ordinal or new Binding identity.
`ProvenUnbound` is emitted only when lookup and ancestor lookup both fail at a
requested receiver site; missing rows are never interpreted as unbound. An
ordinary unresolved Variable outside the request set keeps the existing
`UnresolvedName` failure. Mixed callers, equal keys from different catalog
declarations, duplicate receiver sites, and current-owner `me` requests reject
typed. Import aliases, reserved routes, targets, ABI/effect/result, Builder,
MIR, and runtime remain outside this disconnected product.

## AST-BIND0-R0 qualified route facts

`VerifiedQualifiedCallRouteFactsV1` accepts only one exact S0 call product,
its matching L0 lexical product, and one catalog-branded immutable import
view. The shared neutral reserved-route policy runs before alias lookup; alias
lookup runs before direct-receiver `Bound` rejection. A direct canonical owner
therefore requires positive `ProvenUnbound`, while a verified imported alias
keeps its existing precedence over a same-spelled lexical binding.

The source FastMem context is derived only from the exact call site's
`SourcePathSegmentV1` ancestry. No raw context flag, receiver, method, arity,
lexical enum, or reserved decision can be supplied separately. Alias views
retain their exact declaration catalog, and cross-catalog call/alias pairings
reject by declaration identity.

R0 remains disconnected. It owns neither target lookup nor ABI, effect,
result representation, Builder emission, runtime, or backend behavior.

## AST-BIND0-CUT0 target factories

The qualified target factory consumes `VerifiedQualifiedCallRouteFactsV1`
directly. One exact immutable import-view instance brands the batch, supplies
the declaration catalog, and must be the same instance retained by every
route-fact row. The target layer reads the exact call's caller/site/method/
arity plus the sealed admission/canonical owner; it never repeats lexical,
alias-precedence, or reserved-route policy.

The current-owner factory consumes only borrowed
`VerifiedSourceMethodCallSiteV1` products. It derives `me`, caller, site,
method, and arity from each exact AST site and looks up the target under the
catalog caller's owner. The final target catalog itself retains the exact
declaration-catalog brand, so a catalog sealed from one source unit cannot be
extended with an equal-key call from another source unit.

Raw qualified/current-owner candidate structs, raw lexical/reserved enums,
independently supplied caller/site/AST/spelling inputs, and candidate
constructors are absent. Both factories build locally and publish only a
complete target catalog; production consumers remain zero.

Q0 and M0 admit two route-disjoint source shapes:

```text
qualified receiver.method(arguments)
  + co-sealed qualified route facts
  + the exact retained import-view instance
  -> canonical static callable key

VerifiedSourceMethodCallSiteV1 { receiver: ASTNode::Me, .. }
  + caller namespace = StaticBoxMethod
  + exact target lookup under caller.owner()
  -> canonical current-owner static callable key
```

The final catalog is keyed by caller canonical key and function-relative
`SourceExprSiteV1`, and retains the exact declaration catalog by reference.
Import aliases live in one sorted immutable view over that same catalog. The
mutable Builder import map is never a sealed authority.

The current-owner route consumes the existing catalog by value and extends it;
it never creates a second target catalog. The caller key is the sole owner
authority. `current_static_box`, function-name splitting, MIR symbols, Builder
state, and runtime tags are not authorities. Parser spellings `me` and `this`
both normalize to `ASTNode::Me`, so M0 proves the canonical semantic receiver,
not preservation of the original token spelling.

Q0 and M0 deliberately have no production producer or consumer. They do not
own:

- bare-call resolution;
- builtin, Core, extern, or value-call routing;
- argument evaluation or result representation;
- MIR symbol parsing, emission, runtime behavior, or fallback.

Imported aliases preserve the current Builder precedence over a same-spelled
local binding. Direct canonical receiver spellings require sealed positive
`ProvenUnbound` evidence. Reserved fastmem/MIR/REPL receiver routes are
rejected before target construction.

Future route families may add variants to the final target vocabulary, but
they must keep route-disjoint sealers and reject duplicate caller/site rows
across variants. They must not turn this module into a replay of the complete
Builder call router.
