# Source call target

This module owns disconnected, pre-Builder source-call target proofs.

## SAME-MODULE-CALLABLE-RECEIVER-POLICY0-S0

`SameModuleCallableSourceReceiverPolicyV1` is the sole production projection
from a verified declaration-catalog namespace into the source-neutral shadow
receiver vocabulary:

```text
StaticBoxMethod   -> StaticCurrentOwner
InstanceBoxMethod -> DeclaredInstance
```

The policy owns no target lookup, callable-result inference, lexical receiver
declaration, Builder receiver, MIR, or runtime identity. `FunctionSyntaxViewV1`
for ordinary non-catalog functions and lambdas keeps its existing policy.
Direct `StaticCurrentOwner` production outside this shared policy and focused
tests is structurally forbidden.

## PRELOOP-STAGEB-SOURCE-INVENTORY0-P0

`VerifiedWholeSourceStaticCallTargetInventoryV1` is the sole complete,
Builder-free MethodCall inventory over one declaration-catalog allocation. It
runs the existing shadow MethodCall traversal for every declaration, seals one
exact `VerifiedSourceMethodCallSiteV1` per observed site, and derives the
qualified/current-owner static-target subset only through the existing lexical,
reserved-route, alias, and target factories.

The inventory distinguishes:

```text
complete observed call + exact static target -> target row
complete observed call + bound/dynamic/unresolved receiver -> noncandidate
bounded per-caller source-shape gap
  -> exact first unavailable caller/cause retained; later callers still observed
duplicate/coverage/traversal invariant or invalid/foreign alias authority
  -> typed rejection
```

Absence from the target subset is therefore not interpreted as successful
source observation when the unavailable receipt is present. The product
retains every completed exact call carrier, the deterministic first bounded
unavailable caller/cause, and the one target catalog, all branded by the same
declaration catalog. It owns no Stage-B nested-result policy, candidate
cardinality, Builder, MIR, runtime, or production route.

## RAW-SOURCE-CURSOR0-S0 catalog-backed Raw navigation

`VerifiedRawCallableSourceViewV1` is the Raw route's thin, catalog-branded
source-navigation adapter. It retains the exact declaration-catalog allocation,
the embedded canonical caller key, and that declaration row. Its body,
statement, and expression carriers are stack-scoped borrows of this one view.

It does **not** introduce a second source-navigation engine. Child paths use
the existing `SourcePathV1` and `ExprChildRoleV1` vocabularies, while every
node projection delegates to `project_source_body_node_v1`. The canonical
`FunctionSourceViewV1` remains independent because its carriers are branded by
`FunctionOwnerIdV1`; Raw uses the declaration catalog's allocation identity
instead.

S0 is Builder-free and disconnected: it owns no raw dispatcher change, call
route activation, `ValueId`, `MirType`, type publication, or runtime behavior.
Later association must co-seal catalog allocation, embedded caller key, and
exact `SourceExprSiteV1`; equal-looking catalogs or keys are not authority.

## RAW-LOCATED-INSTANCE-METHOD-INPUT0-S0

`RawLocatedMethodCallInputV1` is the next source-only boundary. Only the
same `VerifiedRawCallableSourceViewV1` that issued a located expression may
seal it, and only when that exact borrowed expression is `ASTNode::MethodCall`.
It retains the view, embedded caller, exact `SourceExprSiteV1`, and borrowed
receiver/method/argument syntax from the same catalog allocation.

It is deliberately not `RawLegacyMethodCallInputV1`: it owns no Builder,
`ValueId`, `MirType`, `type_ctx`, physical Call, result contract, or route
selection. The later candidate-only descent will consume this input without
re-scanning or matching by name; the default raw route remains unchanged.

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

## Accepted Dynamic member target boundary (D0)

The next route family is a source-backed Dynamic member message, not an exact
declaration/provider target. Before its I0, resolved semantics must publish one
AST-free MethodCall row containing the exact call/receiver/result sites,
checked selector/arity, and complete ordered argument sites.

The implementation series generalizes the current static-only catalog names
in place to one `VerifiedSourceCallTargetCatalogV1` with route-disjoint Static
and DynamicMember arms. `DynamicMember(selector, arity)` is runtime message
identity only. It cannot classify receiver Box/type, result class, Home,
effect, ABI, provider, or executable plan. Static/Dynamic rows at the same
caller/site reject as duplicates.

Execution readiness is a later sibling contract. Unknown Dynamic calls need a
selector-independent conservative effect/Fault/suspension/Home envelope before
Recipe CallSlot co-seal; no receipt is inferred from `MirType::Unknown`, MIR
effects, runtime tags, or method spelling. Runtime later combines actual
receiver class with selector/arity and one immutable registry snapshot, selects
one executable plan, and invokes once. Missing or failed selection does not
retry a legacy or provider route.
