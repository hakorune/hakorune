# Resolved Semantics

This module family owns the passive, function-scoped semantic product that is
constructed before Planner and Lower.

```text
canonical function AST
  -> FunctionSemanticResolverV1       (SA1+)
  -> ResolvedFunctionDraftV1
  -> ResolvedFunctionVerifierV1       (SA2+)
  -> VerifiedResolvedFunctionV1
```

SA0 adds only the closed canonical schema and publication boundary. SA1 adds
a physically separate, disconnected shadow resolver. The shadow resolver may
read canonical syntax, but it owns only `Shadow*V0` handles and can publish
only `ShadowResolvedFunctionV0` for tests and deterministic inspection. It
cannot populate the canonical draft, allocate `BindingId`, plan control flow,
or lower MIR.

## OF0/UP0 owner-forest boundary

`VerifiedSemanticOwnerForestV1` is the first cross-owner authority. One
`FunctionSemanticResolverSessionV1` reuses one compilation-scoped owner issuer
and recursively resolves a root function plus non-capturing Lambda children.

```text
borrowed canonical syntax
  -> owner-local shadow traversal
  -> owner-local VerifiedResolvedFunctionV1
  -> primary owners + parent edges
  -> forest verify/derive
  -> VerifiedSemanticOwnerForestV1
```

The forest directly owns every sealed owner product. Its only primary
topology is the owner map plus child-to-parent edges; the single root,
definition-site child index, and normalized graph are derived exactly once at
seal. OF0 rejects a second root, mixed compilation brands, cycles, duplicate
parent/definition sites, and a parent scope that is not the exact lexical
scope containing the Lambda definition.

Lambda syntax is borrowed through an AST-derived view and never cloned into a
semantic product. A child declaration uses a child-local `BindingId`; raw IDs
may repeat across owners without aliasing.

UP0 adds only a structural read relation:

```text
ResolvedLexicalRefV1
  = Local(BindingRefV1)
  | Upvar(UpvarRefV1 {
      capturing_owner,
      source: strict-ancestor BindingRefV1,
    })
```

The relation is deduplicated by `(capturing_owner, source)` and grandchild
references point directly to the original declaration. Forest seal verifies
the source exists, belongs to a strict ancestor, was visible at every Lambda
definition boundary, and is not hidden by a nearer same-name declaration.
Normalized parity uses structural owner keys and binding origins, never raw
owner or binding numbers.

UP1 adds exact-site `Read | Rebind` observations over the same structural
relation. A write is `ResolvedAssignmentTargetV1::UpvarRebind`; it is never
misreported as a local `BindingRebind`. The unique Upvar inventory and ordered
access observations are both seal-derived. They are capture-plan input only:
UP1 creates no CaptureId, synthetic child BindingId, capture mode, forwarding
binding, runtime slot, ValueId, BasicBlockId, Recipe, Planner, or Lower
connection.

## B0-F lexical BlockExpr resolver contract

B0-S adds a disconnected canonical vocabulary for lexical BlockExpr identity:
`ScopeKindV1::BlockExpr` and `RegionKindV1::BlockExpr`. A sealed draft may
represent the expression only as one reciprocal scope/region pair. Both
records carry the same `Source(BlockExprPreludeRoot)` origin; that root is the
lexical anchor for the ordered prelude and the required tail together.

The seal verifier rejects a partial kind pair or different origins and treats
only `BlockExprPrelude(index)` and `BlockExprTail` descendants with the same
prefix as members. Existing owner-branded IDs, parent graphs, and normalized
records remain the only identity and ancestry mechanisms.

B0-F installs one dedicated shadow traversal box over that sealed pair. It
resolves the ordered prelude before the tail, restores the outer lexical scope
after the expression, and gives nested BlockExprs independent pairs. Existing
binding and owner-forest machinery supplies initializer-before-declaration,
outer rebind, and Lambda declaration-order behavior without a second policy.

The neutral AST non-local-exit query is the acceptance SSOT. A rejected
prelude reports its exact statement container and a rejected tail reports its
exact expression container; loop-local Break/Continue remain valid. Planner,
RegionFlow, ProgramV0, and compatibility fallback remain disconnected.
B0-L3a is the first bounded Lower consumer: it queries the sealed BlockExpr
pair by exact expression site and does not change resolver ownership.

## B0-L3b-S1 exact If identity bundle

S1 derives one private ID-only index while sealing
`VerifiedResolvedFunctionV1`. Each exact self-relative statement site maps to
one `If` control RegionId, one required reciprocal `IfThen` scope/region pair,
and zero or one reciprocal `IfElse` pair. The control has no lexical scope;
each branch region is a child of the control and each branch scope is a child
of the surrounding lexical scope.

The authoritative records remain the owner-scoped scope and region arenas.
The private index is a rebuildable seal witness and exists neither in mutable
drafts nor in `ResolvedFunctionDataV1`. Consumers use
`if_region_bundle(site)`; they may not rescan the arenas. Because
`SourceStmtSiteV1` is relative to one function root, this query means “site
inside this verified product.” S2 will prove the cross-owner source/product
closure before RegionFlow consumes it.

S1 proves arena topology only. Matching `else=None` versus
`else=Some(empty)` to located syntax remains an S2 obligation. RegionFlow,
Builder, Lower, ValueId, BasicBlockId, branch effects, ports, join contracts,
and canonical If runtime activation remain disconnected.

## B0-L4-S1 exact Loop identity bundle

S1 derives one private ID-only Loop index while sealing
`VerifiedResolvedFunctionV1`. Each exact self-relative Loop statement site
maps to one reciprocal `Loop` region / `LoopBody` scope pair. The Loop region
is a child of the exact surrounding region; its scope is a child of the exact
surrounding lexical scope. The region origin is the Loop statement site and
the scope origin is its `LoopBodyRoot` child.

The authoritative records remain the owner-scoped scope and region arenas.
The private index is a rebuildable seal witness and exists neither in mutable
drafts nor in `ResolvedFunctionDataV1`. Consumers use
`loop_region_bundle(site)` and may not rescan the arenas. Because
`SourceStmtSiteV1` is relative to one function root, cross-owner
source/product closure remains a later RegionFlow obligation.

S1 proves identity topology only. Located source coverage, carrier effects,
ports, ValueId, BasicBlockId, PHI policy, Builder/Lower connections, and
canonical Loop runtime activation remain disconnected.

## B0-L3b-I1a exact lowering roots

The verified product owns one seal-derived ID-only lowering-root carrier. It
pairs the authoritative Function scope/region roots with exactly one lexical
function-body scope and `Sequence` region rooted at `FunctionBody` or
`LambdaBodyRoot`. The draft/data schema remains carrier-free.

Lower consumes this carrier directly when seeding separate lexical-scope and
control-region stacks. It must not scan the arenas or reconstruct source paths
to rediscover the function-body root. The carrier is a rebuildable seal
witness, not a second scope/region authority, and adds no Builder or runtime
connection by itself.

## B0-L2b shared source-path boundary

`SourcePathSegmentV1` remains the only path grammar. B0-L2b promotes its small
immutable builder to `SourcePathV1` in `source_site.rs`; the shadow resolver's
historical `ShadowSourcePathV0` name is only a local alias. The compiler-side
source projection consumes these paths but this semantic module never imports
compiler or Lower code.

The sealed compiler projection validates that every owner root follows the
forest's exact Lambda definition chain and that semantic declaration/use/
assignment/exit/scope/region sites project to the expected syntax family.
It stores no AST reference or pointer. Future Lower obtains borrowed syntax
only through an immutable function source view and located-node carriers.
B0-L2b adds no resolver acceptance, production source-unit construction,
Planner/RegionFlow/Recipe connection, or MIR materialization.

SA2 installs the real publication boundary. A compilation-scoped issuer with
a process-unique compilation brand is the only owner-brand constructor;
`ResolvedFunctionDraftV1::seal` verifies owner membership, graph
roots/ancestry, binding accounting, supplied source-index integrity, and exact
RegionId control targets before publishing. The sealed product also owns a
deterministic origin-keyed normalized graph for parity. Raw owner, binding,
scope, and region numbers never enter that graph. SA2 still has no canonical
AST resolver producer and no Planner or Lower connection; those are later
authority-cutover slices.

Because the canonical AST is intentionally not stored here, SA2 cannot prove
that a caller omitted no syntax site. Canonical-AST site totality becomes
checkable only when the SA3 resolver co-constructs the closed indexes. SA2
claims referential integrity of supplied indexes, not syntax-independent
omniscience.

## SA3 transport boundary

SA3-A adds a behavior-neutral Lower transport in
`builder/vars/resolved_binding_state.rs`. It can hold one sealed product,
claim an exact `SourceBindingSiteV1` once, then consume that one-shot claim to
publish `BindingId -> ValueId`.  Order, name, and kind alone are never
declaration identity.
Resolved parameter/local publication APIs exist but have no production caller
until the atomic SA3-B switch. During SA3-A the legacy Lower allocator remains
the single active canonical BindingId owner; installing a canonical product is
still zero.

The canonical `FunctionSemanticResolverSessionV1` also exists as a
disconnected producer. It derives and borrows `FunctionSyntaxViewV1` from one
canonical function AST, uses
construction-local draft indices during one traversal, converts them once to
owner-scoped canonical IDs, and immediately verifies/seals. Draft indices
never enter Lower or the normalized parity graph. Production installation
remains zero until the accepted syntax inventory and all declaration callers
can switch together.

## P0c callable-header boundary

`CallableHeaderSyntaxViewV1` is a separate body-free view over one
`FunctionDeclaration`. It exists so source callable name, arity, exact scalar
signature, and physical symbol projection are observed once without widening
the body-oriented `FunctionSyntaxViewV1` or teaching Lower to read raw names.

`VerifiedCallableIndexV1` is the sole callable-header authority. P0c-L0 seals
exactly one static, non-main, all-`i64` header into a deterministic one-entry
index. P0c-S0a keeps it disconnected from production while co-sealing it with
one semantic owner forest in `VerifiedResolvedCallableForestV1`.

`CallableFunctionSyntaxViewV1` derives the header and body views from the same
function AST. The source-unit sidecar owns the index once; each
`VerifiedResolvedFunctionV1` stores only exact
`SourceExprSiteV1 -> ResolvedDirectCallTargetV1` identity rows. Full headers,
symbols, and signatures are never copied into the function product. The
body-only production resolver remains unchanged and produces zero direct-call
target rows until atomic P0c-I1.

```text
CanonicalCallableKeyV1:
  source lookup key

ResolvedCallableRefV1:
  typed reference to the existing FunctionOwnerIdV1 identity

CanonicalCallableSymbolV1:
  one-way physical MIR/backend projection
```

The symbol is never parsed back into source identity. The runtime/plugin
Box-callable registry, MIR module function table, legacy global resolver, and
Lower name comparisons are not callable-index authorities.

## Authority

- The canonical AST owns syntax and source execution order.
- A sealed `VerifiedResolvedFunctionV1` owns resolved binding, scope, region,
  assignment-target, and control-target facts for one function.
- `BindingOriginV1`, `RegionOriginV1`, and source sites are provenance. They
  are not lexical/control equality authorities.
- `BindingRefV1`, `ScopeId`, and `RegionId` carry an invocation-local function
  owner brand. The brand detects cross-function mixing but is not source or
  parity identity. `BindingRefV1` wraps the canonical `BindingId`; it does not
  replace it.
- Lower owns `BindingId -> ValueId` and `RegionId -> BasicBlockId`
  materialization only.

## Forbidden dependencies

This module family must not import or call:

- canonical arena files must not own cloned/normalized AST payloads;
  `FunctionSyntaxViewV1` and the construction resolver may only borrow syntax;
- Planner, Recipe, JoinIR ownership, or Lower modules;
- `ValueId` or `BasicBlockId`;
- `MirBuilder`, `CoreContext`, or any BindingId allocator;
- the GC/debug `mir::region::RegionId`;
- the private `join_ir::ownership` BindingId/ScopeId family.

Mutable drafts remain crate-private. Only a verified sealed product may become
a public consumer input. Unsupported resolution never retries a legacy path.

SA1's `ShadowBindingOrdinalV0` is now also the construction-local draft index
used inside canonical resolution. It is converted and discarded before seal;
it never enters Planner or Lower and is not semantic identity. Unsupported
syntax returns a typed resolver error and never retries an old resolver. There
is no test-only unverified publication bypass.
